use mini_chat::config::ServerConfig;
use mini_chat::server::init_server;

#[tokio::main]
async fn main() {
    init_server(ServerConfig::new("192.168.147.1".to_string(), 8080)).await
}