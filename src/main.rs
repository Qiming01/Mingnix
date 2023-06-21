#![allow(clippy::unused_io_amount)]
use std::sync::{Arc, Mutex};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use clap::{Arg, App};

mod http;
mod server;

use server::handlers::{Echo, Handler, Index, NotFound, StaticFile, VisitCount};

#[derive(Clone, Default)]
pub struct SharedData {
    pub visit_count: u32,
}

#[tokio::main]
async fn main() {
    let matches = App::new("Minginx")
        .version("1.0")
        .author("Qi Ming <qimingme@gmail.com>")
        .about("run server")
        .arg(Arg::with_name("port")
             .short('p')
             .long("port")
             .value_name("PORT")
             .help("Sets a custom port")
             .takes_value(true))
        .get_matches();

    let port = matches.value_of("port").unwrap_or("5000");
    
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    println!("Server start at http://127.0.0.1:{}", port);

    let shared_data = Arc::new(Mutex::new(SharedData { visit_count: 0 }));
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        let shared_data = Arc::clone(&shared_data);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            stream.read(&mut buffer).await.unwrap();
            // println!("{}", String::from_utf8_lossy(&buffer));

            route(&mut stream, &buffer, shared_data).await;
        });
    }
}

async fn route(stream: &mut TcpStream, buffer: &[u8], shared_data: Arc<Mutex<SharedData>>) {
    if buffer.starts_with(b"GET / HTTP/1.1") {
        // Index page
        Index.handle(stream, shared_data).await;
    } else if buffer.starts_with(b"GET /static/") {
        // Static file
        let handler = StaticFile { path_buf: buffer };
        handler.handle(stream, shared_data).await;
    } else if buffer.starts_with(b"GET /count") {
        // Visit count
        VisitCount.handle(stream, shared_data).await;
    } else if buffer.starts_with(b"GET /echo") {
        // Echo
        let handler = Echo { path_buf: buffer };
        handler.handle(stream, shared_data).await;
    } else {
        // 404
        NotFound.handle(stream, shared_data).await;
    }
}
