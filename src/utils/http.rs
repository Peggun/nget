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



use reqwest::Client;
use url::Url;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use indicatif::ProgressBar;
use futures_util::StreamExt;

use crate::error::NgetError;

pub async fn download_file(
    url: &str,
    save_dir: &str,
    progress_bar: &ProgressBar, // Change to ProgressBar
) -> Result<(), NgetError> {
    let client = Client::new();

    // Parse the URL
    let parsed_url = Url::parse(url).map_err(|e| {
        NgetError::InvalidUrl(format!("Failed to parse URL: {}", e))
    })?;

    // Send a GET request
    let response = client.get(url).send().await.map_err(|e| {
        NgetError::HttpRequest(format!("Failed to send request for URL {}: {}", url, e))
    })?;

    // Ensure the request was successful
    if !response.status().is_success() {
        return Err(NgetError::HttpRequest(format!(
            "Failed to fetch file: {}",
            response.status()
        )));
    }

    // Get Content-Length if available
    if let Some(length) = response.content_length() {
        progress_bar.set_length(length);
    }

    // Determine the file name
    let file_name = if parsed_url.path() == "/" {
        "index.html".to_string()
    } else {
        parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("index")
            .to_string()
    };

    let file_path = Path::new(save_dir).join(file_name);

    // Stream the file to the disk
    let mut file = fs::File::create(&file_path).await.map_err(|e| {
        NgetError::FileError(format!("Failed to create file {}: {}", file_path.display(), e))
    })?;

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|e| {
            NgetError::HttpRequest(format!("Failed to read chunk for {}: {}", url, e))
        })?;
        file.write_all(&data).await.map_err(|e| {
            NgetError::FileError(format!("Failed to write to file {}: {}", file_path.display(), e))
        })?;
        progress_bar.inc(data.len() as u64);
    }

    progress_bar.finish_with_message(format!(
        "Download complete: saved to {}",
        file_path.display()
    ));
    
    Ok(())
}