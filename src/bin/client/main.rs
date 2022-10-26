use lib_irc::{log::Logger, DynResult, TEST_ADDRESS};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const LOG_PATH: &str = "data/client.log";

#[tokio::main]
async fn main() -> DynResult<()> {
    Client::new("Client").await?.run(TEST_ADDRESS).await
}

struct Client {
    stream: Option<TcpStream>,
    logger: Logger,
    message_buffer: Vec<u8>,
    alive: bool,
}

impl Client {
    pub async fn new(session_id: &str) -> DynResult<Self> {
        Ok(Self {
            stream: None,
            logger: Logger::new(session_id, LOG_PATH).await?,
            message_buffer: vec![],
            alive: false,
        })
    }

    pub async fn run(mut self, address: &str) -> DynResult<()> {
        let socket_address = address.parse::<SocketAddr>()?;
        self.stream = Some(TcpStream::connect(socket_address).await?);
        self.alive = true;
        self.logger
            .log(format!("Connected to {address}").as_str())
            .await?;

        self.try_send("hello").await?;
        self.try_recieve().await?;

        Ok(())
    }

    async fn try_send(&mut self, data: &str) -> DynResult<()> {
        self.stream
            .as_mut()
            .unwrap()
            .write_all(data.as_bytes())
            .await?;

        Ok(())
    }

    async fn try_recieve(&mut self) -> DynResult<()> {
        self.stream
            .as_mut()
            .unwrap()
            .read(&mut self.message_buffer)
            .await?;

        Ok(())
    }

    fn try_kill(&mut self) -> DynResult<()> {
        if let Err(err) = self.stream.as_mut().unwrap().try_write(b"/quit") {
            return Err(Box::new(err));
        }

        self.alive = false;
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if self.alive {
            self.try_kill().expect("Failed to close connection");
        }
    }
}
