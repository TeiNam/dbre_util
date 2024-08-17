pub mod http_request;
pub mod http_response;

pub use http_request::HttpRequest;
pub use http_request::Method;
pub use http_request::Version;
pub use http_request::Resource;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_http_request_creation() {
        let s = String::from("GET /api HTTP/2.0\r\nHost: example.com\r\nUser-Agent: rust-test\r\n\r\n");
        let req: HttpRequest = s.into();

        assert_eq!(req.method, Method::Get);
        assert_eq!(req.version, Version::V2_0);
        assert_eq!(req.resource, Resource::Path("/api".to_string()));

        let mut expected_headers = HashMap::new();
        expected_headers.insert("Host".to_string(), "example.com".to_string());
        expected_headers.insert("User-Agent".to_string(), "rust-test".to_string());

        assert_eq!(req.headers, expected_headers);
    }
}