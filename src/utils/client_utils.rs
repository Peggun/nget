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

use reqwest::{Client, Proxy};

use crate::proxy_utils::ProxyConfig;
use crate::error::NgetError;
use crate::enums::HttpVersion;

/// Creates a Client with proxy support for each HTTP version.
pub fn get_client(proxy_config: &ProxyConfig, http_version: &HttpVersion) -> Result<Client, NgetError> {
    if proxy_config.proxy_url.is_empty() {
        match http_version {
            HttpVersion::Http11 => Client::builder().build().map_err(|e| NgetError::from(e)),
            HttpVersion::Http2 => Client::builder().http2_prior_knowledge().build().map_err(|e| NgetError::from(e)),
            HttpVersion::Http3 => Client::builder().build().map_err(|e| NgetError::from(e))
        }
    } else {
        match http_version {
            HttpVersion::Http11 => {
                if !proxy_config.proxy_password.is_empty() || !proxy_config.proxy_user.is_empty() {
                    Client::builder()
                        .proxy(Proxy::all(proxy_config.proxy_url.clone())?
                            .basic_auth(&proxy_config.proxy_user, &proxy_config.proxy_password))
                        .build()
                        .map_err(|e| NgetError::from(e))
                } else {
                    Client::builder()
                    .proxy(Proxy::all(proxy_config.proxy_url.clone())?)
                    .build()
                    .map_err(|e| NgetError::from(e)) // Map errors to your custom NgetError
                }
            },
            HttpVersion::Http2 => {
                if !proxy_config.proxy_password.is_empty() || !proxy_config.proxy_user.is_empty() {
                    Client::builder()
                        .proxy(Proxy::all(proxy_config.proxy_url.clone())?
                            .basic_auth(&proxy_config.proxy_user, &proxy_config.proxy_password))
                        .http2_prior_knowledge()
                        .build()
                        .map_err(|e| NgetError::from(e))
                } else {
                    Client::builder()
                    .proxy(Proxy::all(proxy_config.proxy_url.clone())?)
                    .http2_prior_knowledge()
                    .build()
                    .map_err(|e| NgetError::from(e)) // Map errors to your custom NgetError
                }
            },
            HttpVersion::Http3 => {
                if !proxy_config.proxy_password.is_empty() || !proxy_config.proxy_user.is_empty() {
                    Client::builder()
                        .proxy(Proxy::all(proxy_config.proxy_url.clone())?
                            .basic_auth(&proxy_config.proxy_user, &proxy_config.proxy_password))
                        .build()
                        .map_err(|e| NgetError::from(e))
                } else {
                    Client::builder()
                    .proxy(Proxy::all(proxy_config.proxy_url.clone())?)
                    .build()
                    .map_err(|e| NgetError::from(e)) // Map errors to your custom NgetError
                }
            },
        }
    }
}