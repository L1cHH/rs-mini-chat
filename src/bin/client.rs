
use mini_chat::client::connect_to_server;
use mini_chat::config::ServerConfig;

#[tokio::main]
async fn main() {
    connect_to_server(ServerConfig::new("192.168.147.1".to_string(), 8080)).await;
}