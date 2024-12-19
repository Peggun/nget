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

use hickory_resolver::error::ResolveError;

use std::convert::Infallible;
use std::error::Error as StdError;
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NgetError {
    // Errors related to URL parsing
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Invalid TCP URL: {0}")]
    InvalidTcpUrl(String),
    #[error("Invalid URI: {0}")]
    InvalidUri(http::uri::InvalidUri),

    #[error("TCP Connection Error: {0}")]
    TcpConnectionError(io::Error),

    #[error("DNS Resolution Error: {0}")]
    DnsResolutionError(String),

    #[error("Invalid Domain: {0}")]
    InvalidDomain(String),

    #[error("TLS Error: {0}")]
    TlsError(String),

    #[error("Reqwest Error: {0}")]
    ReqwestError(reqwest::Error),

    #[error("Unable to resolve address: {0}")]
    ResolveAddressError(ResolveError),

    #[error("H2 Error: {0}")]
    H2Error(h2::Error),

    #[error("URL Parse Error: {0}")]
    UrlParseError(url::ParseError),

    #[error("Invalid DNS Name Error: {0}")]
    InvalidDnsNameError(tokio_rustls::rustls::pki_types::InvalidDnsNameError),

    #[error("Connection Error: {0}")]
    ConnectionError(String),

    #[error("To String Error: {0}")]
    ToStringError(std::fmt::Error),

    #[error("Unsupported HTTP Version: {0}")]
    UnsupportedHTTPVersion(String),

    // Errors related to HTTP requests
    #[error("HTTP request failed: {0}")]
    HttpRequest(String),
    #[error("Invalid HTTP status code: {0}")]
    InvalidStatusCode(String),
    #[error("HTTP redirection loop detected.")]
    RedirectionLoop,

    // File-related errors
    #[error("Failed to save file: {0}")]
    FileError(String),
    #[error("Checksum validation failed for file: {0}")]
    FileChecksumMismatch(String),
    #[error("File already exists and is locked: {0}")]
    FileLocked(String),
    #[error("IO Error: {0}")]
    IoError(String),

    // Client and configuration issues
    #[error("Unable to build HTTP Client: {0}")]
    ClientError(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    // Networking issues
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("DNS resolution failed for URL: {0}")]
    DnsResolutionFailed(String),

    // Timeout-related issues
    #[error("Connection timeout for URL: {0}")]
    ConnectionTimeout(String),
    #[error("Read timeout for URL: {0}")]
    ReadTimeout(String),

    // Retry and rate limiting
    #[error("Too many retries for URL: {0}")]
    TooManyRetries(String),
    #[error("Rate limit exceeded. Please wait before retrying.")]
    RateLimitExceeded,

    // Unexpected or generic errors
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Unknown error occurred.")]
    Unknown,
}

// Implement From traits for automatic conversion
impl From<std::io::Error> for NgetError {
    fn from(err: std::io::Error) -> Self {
        NgetError::TcpConnectionError(err)
    }
}

impl From<http::uri::InvalidUri> for NgetError {
    fn from(err: http::uri::InvalidUri) -> Self {
        NgetError::InvalidUri(err)
    }
}

impl From<String> for NgetError {
    fn from(err: String) -> Self {
        NgetError::Unexpected(err)
    }
}

impl From<reqwest::Error> for NgetError {
    fn from(err: reqwest::Error) -> Self {
        NgetError::ReqwestError(err)
    }
}

impl From<http::Error> for NgetError {
    fn from(err: http::Error) -> Self {
        NgetError::HttpRequest(format!("HTTP error: {}", err))
    }
}

impl From<Infallible> for NgetError {
    fn from(_err: Infallible) -> Self {
        // This case should be unreachable, but it is required for `FromResidual`
        NgetError::Unexpected("Infallible error".to_string())
    }
}

impl From<h2::Error> for NgetError {
    fn from(err: h2::Error) -> Self {
        NgetError::H2Error(err)
    }
}

impl From<&str> for NgetError {
    fn from(err: &str) -> Self {
        NgetError::Unexpected(err.to_string())
    }
}

impl From<url::ParseError> for NgetError {
    fn from(err: url::ParseError) -> Self {
        NgetError::UrlParseError(err)
    }
}

impl From<Box<dyn StdError>> for NgetError {
    fn from(err: Box<dyn StdError>) -> Self {
        NgetError::Unexpected(format!("An error occurred: {}", err))
    }
}

impl From<tokio_rustls::rustls::pki_types::InvalidDnsNameError> for NgetError {
    fn from(err: tokio_rustls::rustls::pki_types::InvalidDnsNameError) -> Self {
        NgetError::InvalidDnsNameError(err)
    }
}

impl From<std::fmt::Error> for NgetError {
    fn from(err: std::fmt::Error) -> NgetError {
        NgetError::ToStringError(err)
    }
}

impl From<ResolveError> for NgetError {
    fn from(err: ResolveError) -> Self {
        NgetError::ResolveAddressError(err)
    }
}
