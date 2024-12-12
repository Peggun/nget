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

// Run using cargo test
#[cfg(test)]
mod tests {
    use nget::error::NgetError;
    use nget::http;

    #[tokio::test]
    async fn test_download_file_invalid_url() {
        let url = "http://nonexistent.url/file.txt";
        let file_path = "./test_file.txt";

        let result = http::download_file(url, file_path).await; // There are errors due to the addition of the progress bars 

        // Assert that the result is an error
        assert!(result.is_err());

        // Match the specific error type and message
        if let Err(err) = result {
            match err {
                NgetError::HttpRequest(message) => {
                    assert!(message.contains("Failed to send request"), "Unexpected message: {}", message);
                }
                _ => panic!("Unexpected error type: {:?}", err),
            }
        }
    }
}
