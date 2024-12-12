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
}