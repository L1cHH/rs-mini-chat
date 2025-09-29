use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast::{Receiver, Sender};

// TcpConnectionWriter is used for sending
// messages to remote socket. It is using
// Receiver from the tokyo under the hood
pub struct TcpConnectionWriter {
    rx: Receiver<String>,
    socket_writer: OwnedWriteHalf
}

impl TcpConnectionWriter {
    pub fn new(rx: Receiver<String>, socket_writer: OwnedWriteHalf) -> Self {
        TcpConnectionWriter {
            rx,
            socket_writer
        }
    }

    pub async fn write_to_socket(&mut self) {
        loop {
            match self.rx.recv().await {
                Ok(msg) => {
                    if let Err(_) = self
                        .socket_writer
                        .write_all(format!("You received a new message: {}", msg).as_bytes())
                        .await { break }
                },
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    if let Err(_) = self
                        .socket_writer
                        .write_all(b"Server is closed... So try later!")
                        .await { break }
                },
                Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
                    if let Err(_) = self
                        .socket_writer
                        .write_all(format!("Connection is lagging... You have missed {}", missed.to_string()).as_bytes())
                        .await { break }
                }
            }
        }
    }
}

// TcpConnectionReader is used for reading
// messages from remote socket.
pub struct TcpConnectionReader {
    socket_reader: OwnedReadHalf,
    tx: Sender<String>
}

impl TcpConnectionReader {
    pub fn new(tx: Sender<String>, socket_reader: OwnedReadHalf) -> Self {
        TcpConnectionReader {
            socket_reader,
            tx
        }
    }

    pub async fn read_from_socket(&mut self) {
        loop {
            let mut len_buf = [0u8; 4];
            if let Err(_) = self.socket_reader.read_exact(&mut len_buf).await {
                break
            }

            let msg_len = u32::from_be_bytes(len_buf) as usize;
            let mut msg_buf = vec![0u8; msg_len];
            if let Err(_) = self.socket_reader.read_exact(&mut msg_buf).await {
                break
            }

            let message = String::from_utf8_lossy(&msg_buf);
            if let Err(_) = self.tx.send(message.to_string()) {
                println!("Nobody is available to send message yet")
            };

        }

    }
}