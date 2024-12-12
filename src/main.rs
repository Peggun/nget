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

    // Create MultiProgress instance
    let multi_progress = MultiProgress::new();

    // Ensure the output directory exists
    tokio::fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| NgetError::FileError(format!("Failed to create directory: {}", e)))?;

    let download_tasks: Vec<_> = urls.iter().map(|url| {
        let output_dir = output_dir.clone();
        let url = url.clone();
        let pb = multi_progress.add(ProgressBar::new(0));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("=> "),
        );
        tokio::spawn(async move {
            pb.set_message(format!("Downloading: {}", url));
            match http::download_file(&url, &output_dir, &pb).await {
                Ok(_) => {
                    let file_name = url
                        .split('/')
                        .last()
                        .unwrap_or("index.html");
                    pb.finish_with_message(format!(
                        "Download complete. Saved to {}/{}",
                        output_dir, file_name
                    ));
                }
                Err(e) => {
                    pb.abandon_with_message(format!("Failed to download {}: {:?}", url, e));
                }
            }
        })
    }).collect();

    // Wait for all tasks to complete
    let results = future::join_all(download_tasks).await;

    // Handle task results
    for (i, result) in results.into_iter().enumerate() {
        if let Err(err) = result {
            eprintln!("Task {} failed: {:?}", i, err);
        }
    }

    Ok(())
}