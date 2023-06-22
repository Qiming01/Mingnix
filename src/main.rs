#![allow(clippy::unused_io_amount)]
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use log::{error, info, LevelFilter};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use clap::{App, Arg};

use env_logger::Builder;

mod http;
mod server;

use server::handlers::{Echo, Handler, StaticFile, VisitCount};

#[derive(Clone, Default)]
pub struct SharedData {
    pub visit_count: u32,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("Minginx")
        .version("1.0")
        .author("Qi Ming <qimingme@gmail.com>")
        .about("run server")
        .arg(
            Arg::with_name("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Sets a custom port")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log")
                .long("log")
                .value_name("LOG")
                .help("Enables logging")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("log") {
        Builder::new().filter(None, LevelFilter::Info).init();
    }

    let port = matches.value_of("port").unwrap_or("5000");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("Server start at http://127.0.0.1:{}", port);

    let shared_data = Arc::new(Mutex::new(SharedData { visit_count: 0 }));
    loop {
        match listener.accept().await {
            Ok((mut stream, addr)) => {
                info!("New connection accepted");
                let shared_data = Arc::clone(&shared_data);

                tokio::spawn(async move {
                    let mut buffer = [0; 1024];

                    stream.read(&mut buffer).await.unwrap();
                    //info!("{}", String::from_utf8_lossy(&buffer));
                    // Log the client's information
                    log_client_info(addr, &buffer);

                    route(&mut stream, &buffer, shared_data).await;
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn route(stream: &mut TcpStream, buffer: &[u8], shared_data: Arc<Mutex<SharedData>>) {
    if buffer.starts_with(b"GET /") {
        if buffer.starts_with(b"GET /count") {
            VisitCount.handle(stream, shared_data).await;
        } else if buffer.starts_with(b"GET /echo") {
            // Echo
            let handler = Echo { path_buf: buffer };
            handler.handle(stream, shared_data).await;
        } else {
            let handler = StaticFile { path_buf: buffer };
            handler.handle(stream, shared_data).await;
        }
    }
}

fn log_client_info(addr: SocketAddr, buffer: &[u8]) {
    let request_line = std::str::from_utf8(buffer)
        .unwrap_or_default()
        .lines()
        .next()
        .unwrap_or_default();

    info!("Client {}: {}", addr, request_line);
}
