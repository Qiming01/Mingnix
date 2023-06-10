
use tokio::runtime;
use config::*;

mod config;
mod util;

fn main() {
    runtime::Builder::new_multi_thread()
        .thread_name(default::SERVER_NAME)
        .enable_all()
        .build()
        .unwrap_or_else(|err| exit!("Cannot create async runtime\n{:?}", err))
        .block_on(async_main());
}

async fn async_main() {
    let configs = match run() {
        RunType::Start(addr, path) => {
            let config = default::quick_start_config(path.clone(), addr);
            let port = match addr.port() {
                80 => String::new(),
                _ => format!(":{}", addr.port()),
            };
            println!("Serving path   : {}", path.display());
            println!(
                "Serving address: {}",
                format!("http://{}{}", addr.ip(), port)
            );
            vec![config]
        }
        RunType::Config(config_path, is_test) => {
            let configs = ServerConfig::new(&config_path).await;
            // Check configuration file
            if is_test {
                return println!(
                    "There are no errors in the configuration file '{}'",
                    config_path
                );
            }
            configs
        }
    };
    bind_tcp(configs).await;
}