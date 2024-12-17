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

use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone)]
pub enum HttpVersion {
    /// Use HTTP/3 (unstable & unimplemented)
    #[clap(name = "http3")]
    Http3,

    /// Use HTTP/2
    #[clap(name = "http2")]
    Http2,

    /// Use HTTP/1.1 (default)
    #[clap(name = "http11")]
    Http11,
}