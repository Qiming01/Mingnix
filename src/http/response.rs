use std::{collections::HashMap, fmt};

#[derive(Debug)]
pub struct Response<'a> {
    version: HttpVersion,
    status: HttpStatus,
    headers: HashMap<String, String>, // Content-Type: text/html
    body: &'a [u8],
}

// Getters
impl Response<'_> {
    pub fn version(&self) -> HttpVersion {
        self.version
    }

    pub fn status(&self) -> HttpStatus {
        self.status
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    // HTTP headers (&str)
    pub fn headers_http(&self) -> String {
        let mut result = String::new();
        for (k, v) in self.headers().iter() {
            result.push_str(&format!("{k}: {v}\r\n"));
        }
        result
    }

    pub fn body(&self) -> &[u8] {
        self.body
    }

    // Response as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let res = format!(
            "{} {}\r\n{}\r\n",
            self.version(),
            self.status(),
            self.headers_http()
        )
        .as_bytes()
        .to_owned();
        let res = [res, self.body().to_vec()].concat();
        res
    }
}

// Setters
impl<'a> Response<'a> {
    pub fn new() -> Self {
        Self {
            version: HttpVersion::V1_1,
            status: HttpStatus::Ok,
            headers: HashMap::new(),
            body: b"",
        }
    }

    pub fn set_version(&mut self, version: HttpVersion) -> &mut Self {
        self.version = version;
        self
    }

    pub fn set_status(&mut self, status: HttpStatus) -> &mut Self {
        self.status = status;
        self
    }

    pub fn set_headers(&mut self, key: String, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }

    pub fn set_body(&'a mut self, body: &'a [u8]) -> &mut Self {
        self.body = body;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    V1_1,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpVersion::V1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

impl From<&str> for HttpVersion {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "HTTP/1.1" => Self::V1_1,
            _ => Self::V1_1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HttpStatus {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpStatus::Ok => write!(f, "200 OK"),
            HttpStatus::NotFound => write!(f, "404 Not Found"),
            HttpStatus::BadRequest => write!(f, "400 Bad Request"),
            HttpStatus::InternalServerError => write!(f, "500 Internal Server Error"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Html,
    PlainText,
    AvifImage,
    Css,
}

impl From<&str> for ContentType {
    // Parse the `Content-Type` field from &str
    fn from(value: &str) -> Self {
        match value {
            "text/plain" => Self::PlainText,
            "text/html" => Self::Html,
            "text/css" => Self::Css,
            "image/avif" => Self::AvifImage,
            _ => Self::PlainText,
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Html => write!(f, "text/html; charset=utf-8"),
            ContentType::Css => write!(f, "text/css; charset=utf-8"),
            ContentType::PlainText => write!(f, "text/plain; charset=utf-8"),
            ContentType::AvifImage => write!(f, "image/avif"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::http::response::ContentType;

    use super::{HttpStatus, HttpVersion, Response};

    #[test]
    fn test_status_to_string() {
        let expected = "500 Internal Server Error".to_string();
        let res = HttpStatus::InternalServerError;
        assert_eq!(expected, res.to_string());
    }

    #[test]
    fn test_all() {
        let expected =
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 3\r\n\r\nHi!".as_bytes();

        let content = "Hi!";
        let content_length = content.len().to_string();

        let version = HttpVersion::V1_1;
        let status = HttpStatus::Ok;

        let mut res = Response::new();
        let res = res
            .set_status(status)
            .set_version(version)
            .set_body(content.as_bytes())
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), content_length);

        let temp = res.as_bytes();
        let res_http: &[u8] = &temp;

        println!("{}", String::from_utf8_lossy(expected));
        println!("{}", String::from_utf8_lossy(res_http));

        assert_eq!(expected, res_http);
    }
}
