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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: &[(&str, &str)] = &[
        ("../resources/E2.wav", "E2"),
        ("../resources/A2.wav", "A2"),
        ("../resources/D3.wav", "D3"),
        ("../resources/G3.wav", "G3"),
        ("../resources/B3.wav", "B3"),
        ("../resources/E4.wav", "E4"),
    ];

    #[tokio::test]
    async fn test_primary_pitch_detection() {
        for (wav_file, expected_pitch) in TEST_CASES {
            let (samples, sample_rate) =
                wavers::read(wav_file).expect(&format!("Failed to read WAV file: {}", wav_file));

            let event = Event::DetectPitch(samples.to_vec(), sample_rate as f64);

            let core = core::new();
            let (tx, rx) = unbounded::<Effect>();

            core::update(&core, event, &Arc::new(tx)).expect("Failed to process event");

            // Process the render effect
            while let Ok(effect) = rx.recv() {
                match effect {
                    Effect::Render(_) => {
                        let view = core.view();
                        assert_eq!(
                            view.pitch, *expected_pitch,
                            "Pitch detection failed for {}. Expected: {}, Got: {}",
                            wav_file, expected_pitch, view.pitch
                        );
                        println!("✓ {} detected correctly as {}", wav_file, expected_pitch);
                    }
                }
            }
        }
    }
}
