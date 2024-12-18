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

use super::enums::HttpVersion;
use crate::error::NgetError;
use futures_util::StreamExt;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use indicatif::ProgressBar;
use reqwest::Version;
use reqwest::{header, Client};
use std::error::Error;
use std::fs::metadata;
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use url::Url;

pub async fn download_file(
    url: &str,
    save_dir: &str,
    progress_bar: &ProgressBar,
    http_version: &HttpVersion,
) -> Result<(), NgetError> {
    match http_version {
        HttpVersion::Http11 => http11_download(url, save_dir, progress_bar).await,
        HttpVersion::Http2 => http2_download(url, save_dir, progress_bar).await,
        HttpVersion::Http3 => http11_download(url, save_dir, progress_bar).await,
    }
}

pub async fn http11_download(
    url: &str,
    save_dir: &str,
    progress_bar: &ProgressBar,
) -> Result<(), NgetError> {
    let _ = env_logger::try_init();

    // Reuse HTTP client for multiple requests
    static CLIENT: once_cell::sync::Lazy<Client> =
        once_cell::sync::Lazy::new(|| Client::builder().build().unwrap());

    let parsed_url = Url::parse(url).map_err(|e| {
        log::error!("Failed to parse URL '{}': {}", url, e);
        NgetError::InvalidUrl(format!("Failed to parse URL: {}", e))
    })?;

    // Make sure the URL actually exists. Using DNS lookup.
    #[cfg(target_os = "windows")]
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    #[cfg(target_os = "linux")]
    let resolver = TokioAsyncResolver::tokio_from_system_conf();

    let _dns_response = resolver
        .lookup_ip(parsed_url.host_str().unwrap_or_default())
        .await
        .map_err(|e| {
            log::error!("Failed to resolve DNS for '{}': {}", url, e);
            NgetError::DnsResolutionError(format!("Failed to resolve DNS: {}", e))
        })?;

    // Extract file name from URL
    let file_name = if parsed_url.path() == "/" {
        "index.html".to_string()
    } else {
        parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("index")
            .to_string()
    };

    progress_bar.set_message(file_name.clone());
    let file_path = Path::new(save_dir).join(&file_name);

    // Check if the file already exists and get its size
    let existing_size = metadata(&file_path).map(|meta| meta.len()).unwrap_or(0);

    let mut request = CLIENT.get(url);

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
            return Err(NgetError::InvalidUrl(format!(
                "URL is Invalid. Please make sure the URL is correct."
            )));
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
    progress_bar: &ProgressBar,
) -> Result<(), NgetError> {
    let _ = env_logger::try_init();

    // Reuse HTTP client for multiple requests
    static CLIENT: once_cell::sync::Lazy<Client> = once_cell::sync::Lazy::new(|| {
        Client::builder()
            .http2_prior_knowledge() // Enforce HTTP/2
            .build()
            .unwrap()
    });

    let parsed_url = Url::parse(url)
        .map_err(|e| NgetError::InvalidUrl(format!("Failed to parse URL: {}", e)))?;

    log::info!("Parsed URL: {}", parsed_url);

    // Make sure the URL actually exists using DNS lookup
    #[cfg(target_os = "windows")]
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    #[cfg(target_os = "linux")]
    let resolver = TokioAsyncResolver::tokio_from_system_conf();

    let _dns_response = resolver
        .lookup_ip(parsed_url.host_str().unwrap_or_default())
        .await
        .map_err(|e| {
            log::error!("Failed to resolve DNS for '{}': {}", url, e);
            NgetError::DnsResolutionError(format!("Failed to resolve DNS: {}", e))
        })?;

    // Extract file name from URL
    let file_name = if parsed_url.path() == "/" {
        "index.html".to_string()
    } else {
        parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("index")
            .to_string()
    };

    progress_bar.set_message(file_name.clone());
    let file_path = Path::new(save_dir).join(&file_name);

    // Check if the file already exists and get its size
    let existing_size = metadata(&file_path).map(|meta| meta.len()).unwrap_or(0);

    log::info!(
        "File path: {:?}, Existing size: {}",
        file_path,
        existing_size
    );

    // Prepare HTTP request with HTTP/2
    let request = CLIENT.get(url).version(Version::HTTP_2);

    // Handle the response and catch errors
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => {
            if let Some(inner) = e.source() {
                if inner.to_string().contains("UserUnsupportedVersion") {
                    log::error!("HTTP/2 unsupported for URL: {}", url);
                    return Err(NgetError::UnsupportedHTTPVersion(format!(
                        "This URL doesn't support HTTP/2. Please try using HTTP/1.1 instead."
                    )));
                }
            }
            return Err(NgetError::HttpRequest(format!("Request failed: {}", e)));
        }
    };

    log::info!("Response status: {}", response.status());

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
