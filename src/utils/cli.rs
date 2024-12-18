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
use clap::Parser;
/// nget - A modernized wget implementation
#[derive(Parser)]
#[command(name = "nget")]
#[command(version = "0.1.0")]
#[command(author = "Peggun <peggundev@gmail.com>")]
#[command(about = "Download files from the web", long_about = None)]
pub struct Args {
    /// URL for GET request
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    pub urls: Vec<String>,

    /// File Path
    #[arg(short, long, default_value = "./")]
    pub output_dir: String,

    /// Number of retries if download fails
    #[arg(long, default_value = "3")]
    pub retries: i32,

    /// Delay between retries in seconds,
    #[arg(long, default_value = "5")]
    pub delay: u64,

    /// Enable verbose output
    #[arg(short = 'v', long, conflicts_with = "quiet")]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short = 'q', long, conflicts_with = "verbose")]
    pub quiet: bool,

    /// HTTP version to use (unstable & unimplemented for HTTP/3)
    #[arg(long, default_value_t = HttpVersion::Http11, value_enum)]
    pub http_version: HttpVersion,
}
