use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::broadcast::Receiver;

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

    pub async fn listen_write(&mut self) {
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