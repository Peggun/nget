use reqwest::{header, Client};
use url::Url;
use std::error::Error;
use std::process;
use std::fs::metadata;
use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use indicatif::ProgressBar;
use futures_util::StreamExt;
use crate::error::NgetError;
use super::enums::HttpVersion;
use reqwest::Version;

pub async fn download_file(
    url: &str,
    save_dir: &str,
    progress_bar: &ProgressBar,
    http_version: &HttpVersion,
) -> Result<(), NgetError> {

    tracing_subscriber::fmt::init();

    match http_version {
        HttpVersion::Http11 => {
            http11_download(url, save_dir, progress_bar).await
        },
        HttpVersion::Http2 => {
            http2_download(url, save_dir, progress_bar).await
        },
        HttpVersion::Http3 => {
            http11_download(url, save_dir, progress_bar).await
        }
    }
}

pub async fn http11_download(
    url: &str,
    save_dir: &str,
    progress_bar: &ProgressBar
) -> Result<(), NgetError> {

    let _ = env_logger::try_init();

    // Reuse HTTP client for multiple requests
    static CLIENT: once_cell::sync::Lazy<Client> = once_cell::sync::Lazy::new(|| {
        Client::builder().build().unwrap()
    });

    let parsed_url = Url::parse(url).map_err(|e| {
        NgetError::InvalidUrl(format!("Failed to parse URL: {}", e))
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

    let response = request.send().await.map_err(|e| {
        NgetError::HttpRequest(format!("Request error for {}: {}", url, e))
    })?;

    // Handle the response status
    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err(NgetError::HttpRequest(format!("Unexpected status: {}", response.status())));
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
        .map_err(|e| {
            NgetError::FileError(format!("Failed to open file: {}", e))
        })?;

    let mut stream = response.bytes_stream();

    // Process each chunk of the response data
    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|e| {
            NgetError::HttpRequest(format!("Failed to read chunk: {}", e))
        })?;
        // Write the chunk to the file
        file.write_all(&data).await.map_err(|e| {
            NgetError::FileError(format!("Failed to write file: {}", e))
        })?;

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
        Client::builder().build().unwrap()
    });

    let parsed_url = Url::parse(url).map_err(|e| {
        NgetError::InvalidUrl(format!("Failed to parse URL: {}", e))
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

    // Prepare HTTP request with HTTP/2
    let request = CLIENT
        .get(url)
        .version(Version::HTTP_2);

    // Handle the response and catch errors
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => {
            // Check if the error is UserUnsupportedVersion
            if let Some(inner) = e.source() {
                if inner.to_string().contains("UserUnsupportedVersion") {
                    eprintln!(
                        "This URL doesn't support HTTP/2. Please try using HTTP/1.1 instead."
                    );
                    process::exit(1);
                }
            }
            return Err(NgetError::HttpRequest(format!("Request failed: {}", e)));
        }
    };

    // Handle the response status
    if !response.status().is_success() && response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        return Err(NgetError::HttpRequest(format!("Unexpected status: {}", response.status())));
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
        .map_err(|e| {
            NgetError::FileError(format!("Failed to open file: {}", e))
        })?;

    let mut stream = response.bytes_stream();

    // Process each chunk of the response data
    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|e| {
            NgetError::HttpRequest(format!("Failed to read chunk: {}", e))
        })?;
        // Write the chunk to the file
        file.write_all(&data).await.map_err(|e| {
            NgetError::FileError(format!("Failed to write file: {}", e))
        })?;

        // Update progress bar with the number of bytes downloaded
        progress_bar.inc(data.len() as u64);
    }

    // Finish the progress bar with a message indicating completion
    progress_bar.finish_with_message(format!("Saved to: {}", file_path.display()));
    Ok(())
}