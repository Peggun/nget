#[cfg(test)]
mod tests {
    use indicatif::ProgressBar;

    use nget::enums::HttpVersion;
    use nget::http::download_file;
    use nget::proxy_utils::ProxyConfig;

    use std::path::Path;

    use tokio::fs;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_http11_download_success() {
        let mock_server = MockServer::start().await;

        // Mock a successful HTTP/1.1 response
        let mock_response = ResponseTemplate::new(200).set_body_string("Test file content");
        Mock::given(method("GET"))
            .and(path("/testfile"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;

        let url = format!("{}/testfile", &mock_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden(); // Hidden to prevent output during testing

        // Ensure the directory exists
        fs::create_dir_all(save_dir).await.unwrap();

        let config = ProxyConfig::empty();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http11,
            &config
        )
        .await;

        assert!(result.is_ok());
        let saved_file_path = Path::new(save_dir).join("testfile");
        assert!(saved_file_path.exists());

        // Verify the content
        let content = fs::read_to_string(&saved_file_path).await.unwrap();
        assert_eq!(content, "Test file content");

        // Clean up
        fs::remove_file(saved_file_path).await.unwrap();
        fs::remove_dir_all(save_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_http11_download_invalid_url() {
        let invalid_url = "http://nonexistent.url.invalid";
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        let config = ProxyConfig::empty();

        let result = download_file(
            invalid_url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http11,
            &config
        )
        .await;

        // Invalid URL should produce an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_http2_download_success() {
        let mock_server = MockServer::start().await;

        // Mock a successful HTTP/2 response
        let mock_response = ResponseTemplate::new(200).set_body_string("HTTP/2 file content");
        Mock::given(method("GET"))
            .and(path("/http2file"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;

        let url = format!("{}/http2file", &mock_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        let config = ProxyConfig::empty();

        // Ensure the directory exists
        fs::create_dir_all(save_dir).await.unwrap();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http2,
            &config
        )
        .await;

        assert!(result.is_ok());
        let saved_file_path = Path::new(save_dir).join("http2file");
        assert!(saved_file_path.exists());

        // Verify the content
        let content = fs::read_to_string(&saved_file_path).await.unwrap();
        assert_eq!(content, "HTTP/2 file content");

        // Clean up
        fs::remove_file(saved_file_path).await.unwrap();
        fs::remove_dir_all(save_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_http2_download_unsupported_version() {
        let mock_server = MockServer::start().await;

        // Mock an unsupported HTTP version response
        let mock_response = ResponseTemplate::new(505); // 505 HTTP Version Not Supported
        Mock::given(method("GET"))
            .and(path("/unsupported"))
            .respond_with(mock_response)
            .mount(&mock_server)
            .await;

        let url = format!("{}/unsupported", &mock_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        let config = ProxyConfig::empty();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http2,
            &config
        )
        .await;

        // 505 response should result in an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_http11_partial_content() {
        let mock_server = MockServer::start().await;

        // Mock a partial content response
        let partial_response = ResponseTemplate::new(206)
            .set_body_string("Partial")
            .insert_header("Content-Range", "bytes 0-6/7"); // Partial content header
        Mock::given(method("GET"))
            .and(path("/partial"))
            .respond_with(partial_response)
            .mount(&mock_server)
            .await;

        let url = format!("{}/partial", &mock_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        let config = ProxyConfig::empty();

        fs::create_dir_all(save_dir).await.unwrap();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http11,
            &config,
        )
        .await;

        assert!(result.is_ok());

        let saved_file_path = Path::new(save_dir).join("partial");
        assert!(saved_file_path.exists());

        // Validate content
        let content = fs::read_to_string(&saved_file_path).await.unwrap();
        assert_eq!(content, "Partial");

        fs::remove_file(saved_file_path).await.unwrap();
        fs::remove_dir_all(save_dir).await.unwrap();
    }
}
