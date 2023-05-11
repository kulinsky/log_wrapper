mod config;
mod log_message;

use anyhow::Error;
use log::{debug, error};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::io::{BufReader, BufWriter};
use tokio::signal::unix::{signal, SignalKind};

use log_message::LogMessage;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut input = String::new();

    let env_params = config::get_env_params();

    loop {
        input.clear();
        writer.flush().await?;

        tokio::select! {
            // _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
            //     println!("Tick");
            // }
            _ = reader.read_line(&mut input) => {
                if input.is_empty() {
                    debug!("EOF received");
                    break;
                }

                let mut msg: LogMessage = input.trim().into();

                let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

                if let Err(e) = enrich_with_params(&mut msg, &env_params, &timestamp) {
                    error!("Failed to enrich log message: {}", e);

                    continue;
                }

                writer.write_all(format!("{}\n", msg).as_bytes()).await?;
            }
            _ = sigint.recv() => {
                debug!("SIGINT received");
                break;
            }
            _ = sigterm.recv() => {
                debug!("SIGTERM received");
                break;
            }
        }
    }

    writer.flush().await?;

    Ok(())
}

fn enrich_with_params(
    msg: &mut LogMessage,
    env_params: &config::EnvParams,
    timestamp: &str,
) -> Result<(), Error> {
    for (k, v) in env_params.iter() {
        msg.enrich(k, v)?;
    }

    msg.enrich("@wrapper_version", VERSION)?;
    msg.enrich_with_timestamp(&timestamp)?;

    Ok(())
}
