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

use nget::cli;
use nget::http;
use nget::error::NgetError;

use clap::Parser;
use futures::future;
use tokio;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    let urls = args.urls;
    let output_dir = args.output_dir;
    let retries = args.retries;
    let delay = args.delay;
    let verbose = args.verbose;
    let quiet = args.quiet;

    // Create MultiProgress instance
    let multi_progress = MultiProgress::new();

    // Ensure the output directory exists
    tokio::fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to create directory: {}", e)))?;

    // Download tasks
    let download_tasks: Vec<_> = urls.into_iter().map(|url| {
        let output_dir = output_dir.clone();
        let url = url.clone();
        let http_version = args.http_version.clone();
        let pb = if quiet {
            ProgressBar::hidden()
        } else {
            let pb = multi_progress.add(ProgressBar::new_spinner());
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) eta: ({eta})")
                    .unwrap()
                    .progress_chars("=> "),
            );
            pb
        };

        tokio::spawn(async move {
            let mut attempt = 0;
            loop {
                attempt += 1;
                if verbose {
                    eprintln!("Attempt {} for URL: {}", attempt, url);
                }

                match http::download_file(&url, &output_dir, &pb, &http_version).await {
                    Ok(_) => {
                        let file_name = url.split('/').last().unwrap_or("index.html");
                        pb.finish_with_message(format!("Download complete: {}/{}", output_dir, file_name));
                        break;
                    }
                    Err(e) if attempt < retries => {
                        if verbose {
                            eprintln!("Attempt {} failed for URL {}: {:?}. Retrying...", attempt, url, e);
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
                    }
                    Err(e) => {
                        pb.abandon_with_message(format!("Failed to download {}: {:?}", url, e));
                        break;
                    }
                }
            }
        })
    }).collect();

    // Wait for all tasks to complete
    let results = future::join_all(download_tasks).await;

    // Handle task results
    for (i, result) in results.into_iter().enumerate() {
        if let Err(err) = result {
            eprintln!("Task {} panicked: {:?}", i, err);
        }
    }

    Ok(())
}
