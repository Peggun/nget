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

use crate::error::NgetError;

use url::Url;

/// Parses a given URL with NgetError mapping.
pub fn parse_url(url: &str) -> Url {
    Url::parse(url)
        .map_err(|e| {
            log::error!("Failed to parse URL '{}': {}", url, e);
            NgetError::InvalidUrl(format!("Failed to parse URL: {}", e))
        })
        .expect("Failed to parse URL.")
}

/// Gets the file name from a given URL
pub fn get_file_name(url: Url) -> String {
    if url.path() == "/" {
        "index.html".to_string()
    } else {
        url.path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("index")
            .to_string()
    }
}
