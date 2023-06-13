use super::response::HttpVersion;

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    method: HttpMethod,
    path: String,
    version: HttpVersion,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: HttpMethod::Get,
            path: "".into(),
            version: HttpVersion::V1_1,
        }
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn version(&self) -> HttpVersion {
        self.version
    }
}

impl Request {
    pub fn parse_params(&self) -> Vec<&str> {
        let path = self.path();
        // path/<param>/<param>...
        // path/<param>
        if path.trim() != "" {
            self.path.split('/').collect::<Vec<&str>>()
        } else {
            vec![""]
        }
    }

    pub fn parse_queries(&self) -> HashMap<&str, &str> {
        // <param>?<key>=<value>?...
        let parsed = self.parse_params();
        let mut result = HashMap::new();
        for i in parsed {
            if i.contains('?') {
                let query = i.split('?').collect::<Vec<&str>>()[1..].to_vec();

                for i in query {
                    let query = i.split('=').collect::<Vec<&str>>();
                    let (k, v) = (query.first().unwrap_or(&""), query.get(1).unwrap_or(&""));
                    result.insert(*k, *v);
                }
            }
        }
        result
    }
}

impl Request {
    pub fn set_method(&mut self, method: HttpMethod) -> Self {
        self.method = method;
        self.to_owned()
    }

    pub fn set_path(&mut self, path: String) -> Self {
        self.path = path;
        self.to_owned()
    }

    pub fn set_version(&mut self, version: HttpVersion) -> Self {
        self.version = version;
        self.to_owned()
    }
}

impl From<Vec<u8>> for Request {
    fn from(req: Vec<u8>) -> Self {
        // GET / HTTP/1.1
        let req_string = String::from_utf8_lossy(&req);

        let mut req_string = req_string.split_whitespace();

        let method = req_string.next().unwrap_or_default().into();
        let path = req_string.next().unwrap_or_default();
        let version = req_string.next().unwrap_or_default().into();

        Self {
            method,
            path: path.to_string(),
            version,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Get,
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Request;

    #[test]
    fn test_parse_req() {
        let expected = Request {
            method: super::HttpMethod::Get,
            path: "/test".into(),
            version: crate::http::response::HttpVersion::V1_1,
        };
        let req = b"GET /test HTTP/1.1".to_vec();
        let req_parsed = req.into();

        assert_eq!(expected, req_parsed);
    }

    #[test]
    fn test_parse_params() {
        let expected = vec!["", "a", "b"];
        let req = b"GET /a/b HTTP/1.1".to_vec();
        let req_parsed: Request = req.into();
        assert_eq!(expected, req_parsed.parse_params())
    }

    #[test]
    fn test_parse_queries() {
        let expected = HashMap::from([("a", "b"), ("aa", "bb")]);
        let req = Request {
            method: super::HttpMethod::Get,
            path: "get?a=b?aa=bb".into(),
            version: crate::http::response::HttpVersion::V1_1,
        };
        let parsed = req.parse_queries();
        println!("{:?}", parsed);
        assert_eq!(expected, parsed);
    }
}
