// This file is part of nget
//
// Copyright (C) 2024 Peggun
//
// nget is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// nget is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

use crate::enums::HttpVersion;
use crate::error::NgetError;
use crate::utils::client_utils::get_client;
use crate::utils::resolver_utils::build_resolver;
use crate::utils::url_utils::{self, get_file_name};
use crate::utils::proxy_utils::ProxyConfig;

use futures_util::StreamExt;

use indicatif::ProgressBar;

use reqwest::Version;
use reqwest::header;

use std::error::Error;
use std::fs::metadata;
use std::path::Path;

use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub async fn download_file(
    url: &str,
    save_dir: &str,
    output_file_name: &Option<&str>,
    progress_bar: &ProgressBar,
    http_version: &HttpVersion,
    config: &ProxyConfig,
) -> Result<(), NgetError> {
    match http_version {
        HttpVersion::Http11 => http11_download(url, save_dir, *output_file_name, progress_bar, config).await,
        HttpVersion::Http2 => http2_download(url, save_dir, *output_file_name, progress_bar, config).await,
        HttpVersion::Http3 => http11_download(url, save_dir, *output_file_name, progress_bar, config).await,
    }
}

pub async fn http11_download(
    url: &str,
    save_dir: &str,
    output_file_name: Option<&str>,
    progress_bar: &ProgressBar,
    config: &ProxyConfig
) -> Result<(), NgetError> {
    let _ = env_logger::try_init();

    // Create a client
    let client = match get_client(&config, &HttpVersion::Http11) {
        Ok(client) => {
            client
        }
        Err(e) => {
            // Handle the error (e.g., print it, log it, etc.)
            return Err(e);
        }
    };

    let parsed_url = url_utils::parse_url(url);

    // Make sure the URL actually exists using DNS lookup
    let resolver = build_resolver();

    let _dns_response = resolver
        .lookup_ip(parsed_url.host_str().unwrap_or_default())
        .await?;

    // Extract file name from URL or use user arg file name.
    let file_name = match output_file_name {
        Some(name) => name.to_string(),
        None => get_file_name(parsed_url),
    };

    progress_bar.set_message(file_name.clone());
    let file_path = Path::new(save_dir).join(&file_name);

    // Check if the file already exists and get its size
    let existing_size = metadata(&file_path).map(|meta| meta.len()).unwrap_or(0);

    let mut request = client.get(url);

    // If the file exists, set the range to download only the remaining part
    if existing_size > 0 {
        request = request.header(header::RANGE, format!("bytes={}-", existing_size));
    }

    println!("Sending request to URL: {}", url);

    let response = request
        .send()
        .await
        .map_err(|e| NgetError::HttpRequest(format!("Request error for {}: {}", url, e)))?;

    // Handle the response status
    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT
    {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(NgetError::InvalidUrl(
                "URL is Invalid. Please make sure the URL is correct.".to_string(),
            ));
        }
        return Err(NgetError::HttpRequest(format!(
            "Unexpected status: {}",
            response.status()
        )));
    }

    // Set the total length in the progress bar if available
    if let Some(total_length) = response.content_length() {
        progress_bar.set_length(total_length + existing_size);
    }

    // Open the file in append mode for writing the downloaded data
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to open file: {}", e)))?;

    let mut stream = response.bytes_stream();

    // Process each chunk of the response data
    while let Some(chunk) = stream.next().await {
        let data =
            chunk.map_err(|e| NgetError::HttpRequest(format!("Failed to read chunk: {}", e)))?;
        // Write the chunk to the file
        file.write_all(&data)
            .await
            .map_err(|e| NgetError::FileError(format!("Failed to write file: {}", e)))?;

        // Update progress bar with the number of bytes downloaded
        progress_bar.inc(data.len() as u64);
    }

    // Finish the progress bar with a message indicating completion
    progress_bar.finish_with_message(format!("Saved to: {}", file_path.display()));
    Ok(())
}

pub async fn http2_download(
    url: &str,
    save_dir: &str,
    output_file_name: Option<&str>,
    progress_bar: &ProgressBar,
    config: &ProxyConfig
) -> Result<(), NgetError> {
    let _ = env_logger::try_init();

    // Create a client
    let client = match get_client(&config, &HttpVersion::Http2) {
        Ok(client) => {
            client
        }
        Err(e) => {
            // Handle the error (e.g., print it, log it, etc.)
            return Err(e);
        }
    };

    let parsed_url = url_utils::parse_url(url);

    //log::info!("Parsed URL: {}", parsed_url);

    // Make sure the URL actually exists using DNS lookup
    let resolver = build_resolver();

    let _dns_response = resolver
        .lookup_ip(parsed_url.host_str().unwrap_or_default())
        .await?;

    // Extract file name from URL or use user arg file name.
    let file_name = match output_file_name {
        Some(name) => name.to_string(),
        None => get_file_name(parsed_url),
    };

    progress_bar.set_message(file_name.clone());
    let file_path = Path::new(save_dir).join(&file_name);

    // Check if the file already exists and get its size
    let existing_size = metadata(&file_path).map(|meta| meta.len()).unwrap_or(0);

    //log::info!(
    //"File path: {:?}, Existing size: {}",
    //file_path,
    //existing_size
    //);

    // Prepare HTTP request with HTTP/2
    let request = client.get(url).version(Version::HTTP_2);

    // Handle the response and catch errors
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => {
            if let Some(inner) = e.source() {
                if inner.to_string().contains("UserUnsupportedVersion") {
                    //log::error!("HTTP/2 unsupported for URL: {}", url);
                    return Err(NgetError::InvalidUrl(
                        "URL is Invalid. Please make sure the URL is correct.".to_string(),
                    ));
                }
            }
            return Err(NgetError::HttpRequest(format!("Request failed: {}", e)));
        }
    };

    //log::info!("Response status: {}", response.status());

    // Handle the response status
    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT
    {
        return Err(NgetError::HttpRequest(format!(
            "Unexpected status: {} for URL: {}",
            response.status(),
            url
        )));
    }

    // Set the total length in the progress bar if available
    if let Some(total_length) = response.content_length() {
        progress_bar.set_length(total_length + existing_size);
    } else {
        log::warn!("Content length is not provided. Progress bar will not be set.");
        progress_bar.set_length(0); // Default or no progress tracking
    }

    // Open the file in append mode for writing the downloaded data
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to open file: {}", e)))?;

    let mut stream = response.bytes_stream();

    // Process each chunk of the response data
    while let Some(chunk) = stream.next().await {
        let data =
            chunk.map_err(|e| NgetError::HttpRequest(format!("Failed to read chunk: {}", e)))?;
        file.write_all(&data)
            .await
            .map_err(|e| NgetError::FileError(format!("Failed to write file: {}", e)))?;
        progress_bar.inc(data.len() as u64);
    }

    // Finish the progress bar with a message indicating completion
    progress_bar.finish_with_message(format!("Saved to: {}", file_path.display()));
    Ok(())
}
