use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum HttpVersion {
    Http1_1,
    Http2_0,
}

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion::Http1_1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: HttpVersion,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: HttpVersion::default(),
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        version: HttpVersion,
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        response.version = version;
        response.status_code = status_code;
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    fn version(&self) -> &str {
        match self.version {
            HttpVersion::Http1_1 => "HTTP/1.1",
            HttpVersion::Http2_0 => "HTTP/2.0",
        }
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = String::new();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        match res1.version {
            HttpVersion::Http1_1 => format!(
                "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
                &res1.version(),
                &res1.status_code(),
                &res1.status_text(),
                &res1.headers(),
                &res.body.as_ref().map_or(0, |b| b.len()),
                &res1.body()
            ),
            HttpVersion::Http2_0 => format!(
                ":status: {}\r\n{}content-length: {}\r\n\r\n{}",
                &res1.status_code(),
                &res1.headers(),
                &res.body.as_ref().map_or(0, |b| b.len()),
                &res1.body()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200_http1_1() {
        let response_actual = HttpResponse::new(
            HttpVersion::Http1_1,
            "200",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HttpResponse {
            version: HttpVersion::Http1_1,
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404_http2_0() {
        let response_actual = HttpResponse::new(
            HttpVersion::Http2_0,
            "404",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HttpResponse {
            version: HttpVersion::Http2_0,
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation_http1_1() {
        let response_expected = HttpResponse {
            version: HttpVersion::Http1_1,
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";
        assert_eq!(http_string, response_actual);
    }

    #[test]
    fn test_http_response_creation_http2_0() {
        let response_expected = HttpResponse {
            version: HttpVersion::Http2_0,
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual = ":status: 404\r\nContent-Type:text/html\r\ncontent-length: 33\r\n\r\nItem was shipped on 21st Dec 2020";
        assert_eq!(http_string, response_actual);
    }
}