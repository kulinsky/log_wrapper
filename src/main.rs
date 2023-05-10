mod config;
mod log_message;

use anyhow::Error;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::io::{BufReader, BufWriter};
use tokio::signal::unix::{signal, SignalKind};

use log_message::LogMessage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut input = String::new();

    let env_params = config::get_env_params();

    loop {
        input.clear();

        tokio::select! {
            // _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
            //     println!("Tick");
            // }
            _ = reader.read_line(&mut input) => {
                let mut msg: LogMessage = input.trim().into();

                for (k, v) in env_params.iter() {
                    msg.enrich(k, v)?;
                }

                let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

                msg.enrich_with_timestamp(&timestamp)?;

                writer.write_all(msg.to_string().as_bytes()).await?;
            }
            _ = sigint.recv() => {
                println!("SIGINT received");
                break;
            }
            _ = sigterm.recv() => {
                println!("SIGTERM received");
                break;
            }
        }

        writer.write_u8(b'\n').await?;
        writer.flush().await?;
    }

    Ok(())
}
