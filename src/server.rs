use std::ops::Add;
use crate::connection::{TcpConnectionReader, TcpConnectionWriter};

pub struct ServerConfig {
    addr: String,
    port: u32
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            addr: "0.0.0.0".to_string(),
            port: 8080
        }
    }
}

impl ServerConfig {
    pub fn new(addr: String, port: u32) -> Self {
        ServerConfig {
            addr,
            port
        }
    }
}

pub async fn init_server(server_config: ServerConfig) {
    let str_port = server_config.port.to_string();
    let full_address = server_config.addr
        .add(":")
        .add(str_port.as_str());

    println!("Server is building on address: {}", full_address);

    let tcp_listener = match tokio::net::TcpListener::bind(full_address).await {
        Ok(tcp) => tcp,
        Err(e) => panic!("Error occur while binding TcpListener: {:?}", e)
    };


    let (tx, _) = tokio::sync::broadcast::channel::<String>(10);

    loop {
        let (socket, _) = match tcp_listener.accept().await {
            Ok(s) => {
                println!("New connection accepted from {}", s.1);
                s
            },
            Err(e) => {
                println!("Error occur while accepting new connection : {:?}", e);
                continue
            }
        };

        let (read_half, write_half) = socket.into_split();

        let sender = tx.clone();
        //Task that generates for each tcp connection and listens for messages from
        // remote socket
        tokio::spawn(async move {
            let mut con = TcpConnectionReader::new(sender, read_half);
            con.read_from_socket().await
        });

        let receiver = tx.subscribe();
        //Task that generates for each tcp connection and listens for messages from tx
        //in our case is just a tcp server
        tokio::spawn(async move {
            let mut con = TcpConnectionWriter::new(receiver, write_half);
            con.write_to_socket().await;
        });
    }

}

