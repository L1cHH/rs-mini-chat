use std::ops::Add;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use crate::config::ServerConfig;

pub async fn connect_to_server(server_config: ServerConfig) {
    let port = server_config.port.to_string();
    let full_addr = server_config.addr.add(":").add(&port);

    let socket = TcpStream::connect(full_addr)
        .await
        .expect("Tcp server is unavailable");

    let (mut read_socket, mut write_socket) = socket.into_split();

    let read_task = tokio::spawn(async move {
        loop {
            let mut len_buf = [0u8; 4];
            if let Err(_) = read_socket.read_exact(&mut len_buf).await {
                println!("Connection lost!");
                break
            }

            let msg_len = u32::from_be_bytes(len_buf) as usize;
            let mut msg_buf = vec![0u8; msg_len];
            if let Err(_) = read_socket.read_exact(&mut msg_buf).await {
                println!("Connection lost!");
                break
            }

            let message = String::from_utf8_lossy(&msg_buf);
            println!("You received a new message: {}", message)
        }
    });

    let write_task = tokio::spawn(async move {
        let mut line = String::new();
        let mut reader = BufReader::new(tokio::io::stdin());
        loop {
            line.clear();
            if reader.read_line(&mut line).await.unwrap() == 0 {
                break;
            }
            let msg_len = line.len() as u32;
            let msg_len = msg_len.to_be_bytes();
            if let Err(_) = write_socket.write_all(&msg_len).await {
                break
            }

            if let Err(_) = write_socket.write_all(line.as_bytes()).await {
                break
            }
        }
    });

    tokio::join!(read_task, write_task);
}