use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use tokio::process::Command;

use tokio::{fs, io::AsyncWriteExt, net::TcpStream};

use crate::{
    http::{
        request::Request,
        response::{ContentType, HttpStatus, Response},
    },
    SharedData,
};

pub struct NotFound;
pub struct VisitCount;
pub struct Echo<'a> {
    pub path_buf: &'a [u8],
}

pub struct PHPFile<'a> {
    pub path_buf: &'a [u8],
}

pub struct StaticFile<'a> {
    pub path_buf: &'a [u8],
}

#[async_trait]
pub trait Handler {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>);
}

#[async_trait]
impl Handler for PHPFile<'_> {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        let buf = &String::from_utf8_lossy(self.path_buf); // Buffer to string
        let start_index = buf.find("GET ").unwrap() + 4;
        let end_index = buf.find(" HTTP/").unwrap();
        let mut path = String::from(&buf[start_index..end_index]);
        path = format!(".{}", path);
        let output = Command::new("php").arg(&path).output().await.unwrap();

        if !output.status.success() {
            log::error!("PHP execution failed {}", String::from_utf8_lossy(&output.stderr));

            NotFound.handle(stream, Arc::clone(&shared_data)).await;
        }

        let out = String::from_utf8_lossy(&output.stdout);
        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::Ok)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), out.len().to_string())
            .set_body(&out.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

#[async_trait]
impl Handler for VisitCount {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        shared_data.lock().unwrap().visit_count += 1;
        let visit_count = shared_data.lock().unwrap().visit_count;

        let body = format!("{} Times!", visit_count);

        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::Ok)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), body.len().to_string())
            .set_body(body.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

#[async_trait]
impl Handler for Echo<'_> {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let req: Request = self.path_buf.to_vec().into();
        let queries = req.parse_queries();

        let body = queries.get("content").unwrap_or(&"Need some arguments");

        let mut response = Response::new();
        let response = response
            .set_status(HttpStatus::Ok)
            .set_headers("Content-Type".into(), ContentType::Html.to_string())
            .set_headers("Content-Length".into(), body.len().to_string())
            .set_body(body.as_bytes());

        stream.write_all(&response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
    }
}

#[async_trait]
impl Handler for StaticFile<'_> {
    async fn handle(&self, stream: &mut TcpStream, shared_data: Arc<Mutex<SharedData>>) {
        let buf = &String::from_utf8_lossy(self.path_buf); // Buffer to string
        let start_index = buf.find("GET ").unwrap() + 4;
        let end_index = buf.find(" HTTP/").unwrap();
        let mut path = String::from(&buf[start_index..end_index]);
        if let Ok(metadata) = fs::metadata(format!(".{path}")).await {
            if metadata.is_dir() {
                let mut html = String::new();
                html.push_str(
                    format!("<html>\n<head>\n<title>{path}</title>\n</head>\n<body>\n").as_str(),
                );
                html.push_str("<h1>Directory Listing</h1>\n");
                // Generate links for files
                let mut dir_entries = tokio::fs::read_dir(format!(".{path}")).await.unwrap();
                while let Some(entry) = dir_entries.next_entry().await.unwrap() {
                    let file_name = entry.file_name();
                    let file_path = entry.path();
                    let file_path = file_path.to_string_lossy();
                    let current_path_vec: Vec<&str> = file_path.split('/').collect();
                    let current_path = current_path_vec.iter().rev().nth(1).unwrap();
                    html.push_str(&format!(
                        "<a href=\"./{}/{}\">{}</a><br>\n",
                        current_path,
                        file_name.to_string_lossy(),
                        file_name.to_string_lossy()
                    ));
                }
                html.push_str("</body>\n</html>");
                let mut response = Response::new();
                let response = response
                    .set_status(HttpStatus::Ok)
                    .set_headers("Content-Type".into(), ContentType::Html.to_string())
                    .set_headers("Content-Length".into(), html.len().to_string())
                    .set_body(&html.as_bytes());

                stream.write_all(&response.as_bytes()).await.unwrap();
                stream.flush().await.unwrap();
            } else {
                path = format!(".{}", path);
                //println!("{}", path);
                //println!("Get: {}", path);
                let file = fs::read(&path).await;
                if let Ok(f) = file {
                    let content_type = parse_content_type(&path);

                    let mut response = Response::new();
                    let response = response
                        .set_status(HttpStatus::Ok)
                        .set_headers("Content-Type".into(), content_type.to_string())
                        .set_headers("Content-Length".into(), f.len().to_string())
                        .set_body(&f);

                    stream.write_all(&response.as_bytes()).await.unwrap();
                    stream.flush().await.unwrap();
                } else {
                    NotFound.handle(stream, Arc::clone(&shared_data)).await;
                }
            }
        } else {
            NotFound.handle(stream, Arc::clone(&shared_data)).await;
        }
    }
}

#[async_trait]
impl Handler for NotFound {
    async fn handle(&self, stream: &mut TcpStream, _shared_data: Arc<Mutex<SharedData>>) {
        let file = fs::read("./static/404.html").await;
        if let Ok(f) = file {
            let mut response = Response::new();
            let response = response
                .set_status(HttpStatus::NotFound)
                .set_headers("Content-Type".into(), ContentType::Html.to_string())
                .set_headers("Content-Length".into(), f.len().to_string())
                .set_body(&f);

            stream.write_all(&response.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        }
    }
}

// Parse the `Content-Type` from request
fn parse_content_type(req: &str) -> ContentType {
    // .html or .htm
    if req.contains(".htm") {
        ContentType::Html
    } else if req.contains(".txt") {
        ContentType::PlainText
    } else if req.contains(".css") {
        ContentType::Css
    } else if req.contains(".png") || req.contains(".jpg") || req.contains(".ico") {
        ContentType::AvifImage
    } else if req.contains(".mp4") {
        ContentType::Mp4Video
    } else {
        ContentType::Html
    }
}
