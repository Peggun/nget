#[cfg(test)]
mod proxy_tests {
    use indicatif::ProgressBar;
    use nget::enums::HttpVersion;
    use nget::http::download_file;
    use nget::proxy_utils::ProxyConfig;
    use std::path::Path;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_proxy_with_mock_server() {
        let proxy_server = MockServer::start().await; // Mock proxy server
        let target_server = MockServer::start().await; // Mock target server

        // Mock the proxy server behavior to forward requests to the target server
        Mock::given(method("GET"))
            .and(path("/proxytest"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Proxied content"))
            .mount(&proxy_server)
            .await;

        let proxy_config = ProxyConfig {
            proxy_url: proxy_server.uri(), // Point to the mock proxy
            proxy_user: String::new(),
            proxy_password: String::new(),
        };

        let url = format!("{}/proxytest", target_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        // Create directory for test output
        tokio::fs::create_dir_all(save_dir).await.unwrap();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http11,
            &proxy_config,
        )
        .await;

        // Assert success and validate file contents
        assert!(result.is_ok(), "Proxy request failed: {:?}", result.err());
        let saved_file_path = Path::new(save_dir).join("proxytest");
        assert!(saved_file_path.exists());

        let content = tokio::fs::read_to_string(&saved_file_path).await.unwrap();
        assert_eq!(content, "Proxied content");

        // Clean up
        tokio::fs::remove_file(&saved_file_path).await.unwrap();
        tokio::fs::remove_dir_all(save_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_proxy_failure() {
        let proxy_server = MockServer::start().await; // Mock proxy server
        let target_server = MockServer::start().await; // Mock target server

        // Simulate the proxy rejecting the connection
        Mock::given(method("CONNECT")) // Proxy tunneling
            .respond_with(ResponseTemplate::new(502)) // Bad Gateway, proxy failure
            .mount(&proxy_server)
            .await;

        let proxy_config = ProxyConfig {
            proxy_url: proxy_server.uri(), // Use the mock proxy
            ..Default::default()
        };

        let url = format!("{}/proxytest", &target_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        tokio::fs::create_dir_all(save_dir).await.unwrap();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http11,
            &proxy_config,
        )
        .await;

        assert!(result.is_err()); // The proxy failure should cause an error

        let saved_file_path = Path::new(save_dir).join("proxytest");
        assert!(!saved_file_path.exists()); // Ensure no file is created

        tokio::fs::remove_dir_all(save_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_no_proxy() {
        let target_server = MockServer::start().await;

        // Simulate a successful target server response
        Mock::given(method("GET"))
            .and(path("/noproxytest"))
            .respond_with(ResponseTemplate::new(200).set_body_string("Direct content"))
            .mount(&target_server)
            .await;

        let proxy_config = ProxyConfig::empty(); // No proxy

        let url = format!("{}/noproxytest", &target_server.uri());
        let save_dir = "./test_output";
        let output_file_name = None;
        let progress_bar = ProgressBar::hidden();

        tokio::fs::create_dir_all(save_dir).await.unwrap();

        let result = download_file(
            &url,
            save_dir,
            &output_file_name,
            &progress_bar,
            &HttpVersion::Http11,
            &proxy_config,
        )
        .await;

        assert!(result.is_ok());
        let saved_file_path = Path::new(save_dir).join("noproxytest");
        assert!(saved_file_path.exists());

        let content = tokio::fs::read_to_string(&saved_file_path).await.unwrap();
        assert_eq!(content, "Direct content");

        tokio::fs::remove_file(&saved_file_path).await.unwrap();
        tokio::fs::remove_dir_all(save_dir).await.unwrap();
    }
}
