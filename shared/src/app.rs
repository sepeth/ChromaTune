use circular_buffer::CircularBuffer;
use crux_core::{
    macros::effect,
    render::{render, RenderOperation},
    Command,
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

// Populated from https://www.seventhstring.com/resources/notefrequencies.html
const HZ_PITCH_PAIRS: [(f64, &str); 6] = [
    (82.41, "E2"),
    (110.0, "A2"),
    (146.8, "D3"),
    (196.0, "G3"),
    (246.9, "B3"),
    (329.6, "E4"),
];

fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

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

    idx = clamp(idx, 1, HZ_PITCH_PAIRS.len() - 1);

    // We have the range (hz is in between idx-1 and idx)
    let (prev_hz, prev_pitch) = HZ_PITCH_PAIRS[idx - 1];
    let (curr_hz, curr_pitch) = HZ_PITCH_PAIRS[idx];
    let prev_diff = hz - prev_hz;
    let curr_diff = curr_hz - hz;

    if prev_diff < curr_diff {
        (prev_pitch, prev_diff)
    } else {
        (curr_pitch, curr_diff)
    }
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
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
                let (hz, _amplitude) = pitch::detect(&data);
                model.readings.push_back(hz);
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
