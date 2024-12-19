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

pub struct ProxyConfig {
    pub proxy_url: String,
    pub proxy_user: String,
    pub proxy_password: String,
}

impl ProxyConfig {
    /// Creates a new empty instance of ProxyConfig. Good for test use
    pub fn empty() -> Self {
        ProxyConfig {
            proxy_url: String::new(),
            proxy_user: String::new(),
            proxy_password: String::new()
        }
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            proxy_url: String::new(),
            proxy_user: String::new(),
            proxy_password: String::new(),
        }
    }
}