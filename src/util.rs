use async_compression::Level;
use globset::{Glob, GlobMatcher};
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Method, StatusCode, Uri};
use regex::Regex;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use time::util::validate_format_string;
use tokio::fs;

/// 定义了一个名为exit的宏，用于打印带有错误提示信息的错误消息，并退出程序。它接受任意数量的参数，并将它们打印为错误信息。
#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
        {
            eprint!("{}", "[ERROR]: ");
            eprintln!($($arg)*);
            std::process::exit(1)
        }
    };
}

/// 将字符串解析为SocketAddr类型的网络地址
pub fn to_socket_addr(s: &str) -> Result<SocketAddr, String> {
    // 解析出SocketAddr
    if let Ok(addr) = s.parse::<SocketAddr>() {
        return Ok(addr);
    }

    // 解析出ip地址
    if let Ok(ip) = s.parse::<Ipv4Addr>() {
        if let Ok(addr) = format!("{}:80", ip).parse::<SocketAddr>() {
            return Ok(addr);
        }
    }

    // 解析出端口号
    if let Ok(port) = s.parse::<u16>() {
        if let Ok(addr) = format!("0.0.0.0:{}", port).parse::<SocketAddr>() {
            return Ok(addr);
        }
    }

    // 无法解析
    Err(format!("Cannot parse `{}` to SocketAddr", s))
}

/// 获取当前的工作目录，并返回一个PathBuf类型的路径。如果获取失败，它会打印错误信息并退出程序
pub fn current_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|err| exit!("Can't get working directory\n{:?}", err))
}

pub fn to_glob(s: &str) -> Result<GlobMatcher, String> {
    Glob::new(s)
        .map(|g| g.compile_matcher())
        .map_err(|err| format!("Cannot parse `{}` to glob matcher\n{}", s, err))
}

pub fn to_header_name(s: &str) -> Result<HeaderName, String> {
    HeaderName::from_str(s)
        .map_err(|err| format!("Cannot parse `{}` to http header name\n{}", s, err))
}

pub fn to_header_value(s: &str) -> Result<HeaderValue, String> {
    HeaderValue::from_str(s)
        .map_err(|err| format!("Cannot parse `{}` to http header value\n{}", s, err))
}

pub fn to_method(s: &str) -> Result<Method, String> {
    Method::from_str(s).map_err(|err| format!("Cannot parse `{}` to http method\n{}", s, err))
}

pub fn to_status_code(s: &str) -> Result<StatusCode, String> {
    StatusCode::from_str(s).map_err(|err| format!("Cannot parse `{}` to http status\n{}", s, err))
}

pub fn to_regex(s: &str) -> Result<Regex, String> {
    Regex::new(s).map_err(|err| format!("Cannot parse `{}` to regular expression\n{}", s, err))
}

pub fn to_ip_addr(s: &str) -> Result<IpAddr, String> {
    s.parse::<IpAddr>()
        .map_err(|_| format!("Cannot parse `{}` to IP addr", s))
}

pub fn to_url(s: &str) -> Result<Uri, String> {
    s.parse::<Uri>()
        .map_err(|err| format!("Cannot parse `{}` to http url\n{}", s, err))
}

pub fn to_compress_level(s: &str) -> Result<Level, String> {
    match s {
        "fast" => Ok(Level::Fastest),
        "default" => Ok(Level::Default),
        "best" => Ok(Level::Best),
        _ => Err(format!(
            "Wrong compression level `{}`, optional value: `fast` `default` `best`",
            s
        )),
    }
}

pub fn check_strftime(s: &str) -> Result<(), String> {
    validate_format_string(s).map_err(|err| format!("Cannot parse `{}` to time format\n{}", s, err))
}

// Convert path to absolute path
pub fn absolute_path<P: AsRef<Path>, R: AsRef<Path>>(path: P, root: R) -> PathBuf {
    let path = path.as_ref();
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.as_ref().join(path)
    }
}
