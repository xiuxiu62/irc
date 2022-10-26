use std::{net::SocketAddr, sync::Arc};

use lib_irc::{log::Logger, DynResult, TEST_ADDRESS};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

const LOG_PATH: &str = "data/server.log";

#[tokio::main]
async fn main() -> DynResult<()> {
    Server::new("Server").await?.run(TEST_ADDRESS).await
}

pub struct Server {
    logger: Logger,
    message_buffer: Vec<u8>,
}

impl Server {
    pub async fn new(session_id: &str) -> DynResult<Self> {
        Ok(Self {
            logger: Logger::new(session_id, LOG_PATH).await?,
            message_buffer: vec![],
        })
    }

    pub async fn run(mut self, address: &str) -> DynResult<()> {
        let socket_address = address.parse::<SocketAddr>()?;
        let listener = TcpListener::bind(socket_address).await?;
        self.logger
            .log(format!("Listening on {socket_address}").as_str())
            .await?;
        let server = Arc::new(Mutex::new(self));

        loop {
            let connection = listener.accept().await?;
            Self::handle_connection(Arc::clone(&server), connection).await;
        }
    }

    async fn handle_connection(
        server: Arc<Mutex<Self>>,
        (mut socket, address): (TcpStream, SocketAddr),
    ) {
        tokio::spawn(async move {
            let mut server_handle = server.lock().await;

            server_handle
                .logger
                .log(format!("Accepted connection from {address}").as_str())
                .await
                .expect("Failed to write to log");

            loop {
                server_handle
                    .try_recieve(&mut socket)
                    .await
                    .expect("Failed to recieve data");

                server_handle
                    .try_send(&mut socket, "hello world")
                    .await
                    .expect("Failed to send data");
            }
        });
    }

    async fn try_send(&mut self, socket: &mut TcpStream, message: &str) -> DynResult<()> {
        socket.write_all(message.as_bytes()).await?;
        self.logger
            .log(format!("Sent {} bytes to {}", message.len(), socket.peer_addr()?).as_str())
            .await?;

        Ok(())
    }

    async fn try_recieve(&mut self, socket: &mut TcpStream) -> DynResult<()> {
        self.message_buffer.clear();
        socket.read(&mut self.message_buffer).await?;
        self.logger
            .log(
                format!(
                    "Recieved {} bytes from {}",
                    self.message_buffer.len(),
                    socket.peer_addr()?
                )
                .as_str(),
            )
            .await?;

        Ok(())
    }
}
