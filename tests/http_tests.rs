#[cfg(test)]
mod tests {
    use nget::error::NgetError;
    use nget::http;

    #[tokio::test]
    async fn test_download_file_invalid_url() {
        let url = "http://nonexistent.url/file.txt";
        let file_path = "./test_file.txt";

        let result = http::download_file(url, file_path).await; // ik theres errors with the additions to the progress bars. 

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
