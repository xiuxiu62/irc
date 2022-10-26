use crate::DynResult;
use tokio::{
    fs::{self, File, OpenOptions},
    io::AsyncWriteExt,
    time::Instant,
};

pub struct Logger {
    session_id: String,
    file: File,
    epoch: Instant,
}

impl Logger {
    pub async fn new(session_id: &str, path: &str) -> DynResult<Self> {
        fs::write(path, "").await?;

        Ok(Self {
            session_id: session_id.to_owned(),
            file: OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .await?,
            epoch: Instant::now(),
        })
    }

    pub async fn log(&mut self, data: &str) -> DynResult<()> {
        let message = format!("[{}] {}: {data}\n", self.format_elapsed(), self.session_id);
        self.file.write_all(message.as_bytes()).await?;

        Ok(())
    }

    fn format_elapsed(&self) -> String {
        format!("{:.3}", self.epoch.elapsed().as_secs_f32())
    }
}
