// This file is part of nget
//
// Copyright (C) 2024 Peggun
//
// Peggun is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Peggun is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NgetError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("HTTP request failed: {0}")]
    HttpRequest(String),

    #[error("Failed to save file: {0}")]
    FileError(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),

    #[error("Unknown error occurred.")]
    Unknown,
}