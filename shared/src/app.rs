use circular_buffer::CircularBuffer;
use crux_core::{
    macros::effect,
    render::{render, RenderOperation},
    App, Command,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Event {
    DetectPitch(Vec<f64>),
}

#[effect(typegen)]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default)]
pub struct Model {
    readings: CircularBuffer<4, f64>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ViewModel {
    pub pitch: String,
    pub diff: String,
}

#[derive(Default)]
pub struct Counter;

// Populated from https://www.seventhstring.com/resources/notefrequencies.html
const HZ_PITCH_PAIRS: [(f64, &str); 6] = [
    (82.41, "E2"),
    (110.0, "A2"),
    (146.8, "D3"),
    (196.0, "G3"),
    (246.9, "B3"),
    (329.6, "E4"),
];

fn find_pitch(hz: f64) -> (&'static str, f64) {
    // Find the range that the hz argument in
    let mut idx = 0;
    for (k, _) in HZ_PITCH_PAIRS {
        if k < hz {
            idx += 1;
        } else {
            break;
        }
    }

    if idx >= HZ_PITCH_PAIRS.len() {
        idx = HZ_PITCH_PAIRS.len() - 1;
    }
    // We have the range (hz is in between idx and idx+1)
    let (prev_hz, prev_pitch) = HZ_PITCH_PAIRS[idx];
    let next_pair = if (idx + 1) < HZ_PITCH_PAIRS.len() {
        Some(HZ_PITCH_PAIRS[idx + 1])
    } else {
        None
    };

    let mut pitch = prev_pitch;
    let mut diff = hz - prev_hz;
    if let Some((next_hz, next_pitch)) = next_pair {
        if (next_hz - hz) < diff {
            pitch = next_pitch;
            diff = next_hz - hz;
        }
    }
    return (pitch, diff);
}

impl App for Counter {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = (); // will be deprecated, so use unit type for now
    type Effect = Effect;

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &(), // will be deprecated, so prefix with underscore for now
    ) -> Command<Effect, Event> {
        match event {
            Event::DetectPitch(data) => {
                if data.len() < 10 {
                    println!("Error: Input data is too small for pitch detection.");
                } else {
                    let (hz, _amplitude) = pitch::detect(&data);
                    model.readings.push_back(hz);
                }
            }
        }

        render()
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        let (pitch, diff) = if let Some(hz) = model.readings.back() {
            find_pitch(*hz)
        } else {
            ("", 0.)
        };
        ViewModel {
            pitch: String::from(pitch),
            diff: String::from(format!("{:.2}", diff)),
        }
    }
}
