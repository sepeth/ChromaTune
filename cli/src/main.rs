mod core;

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use crossbeam_channel::unbounded;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use shared::{Effect, Event};

#[derive(Parser, Clone)]
enum Command {
    DetectPitch(DetectPitchArgs),
}

#[derive(Parser, Clone)]
struct DetectPitchArgs {
    path: String,
}

impl From<Command> for Event {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::DetectPitch(args) => {
                let (samples, sample_rate) =
                    wavers::read(&args.path).expect("Failed to read WAV file");
                Event::DetectPitch(samples.to_vec(), sample_rate as f64)
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    let format = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(filter)
        .with(format)
        .init();

    let command = Args::parse().cmd;

    let core = core::new();
    let event = command.into();
    let (tx, rx) = unbounded::<Effect>();

    core::update(&core, event, &Arc::new(tx))?;

    while let Ok(effect) = rx.recv() {
        match effect {
            Effect::Render(_) => {
                let view = core.view();
                println!("{} (pitch)", view.pitch);
                println!("{} (pitch-detector)", view.pd_pitch);
            }
        }
    }

    Ok(())
}
