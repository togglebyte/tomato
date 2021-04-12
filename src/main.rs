use std::env;
use std::time::{Instant, Duration};

use tinybit::events::{events, Event, EventModel, KeyCode, KeyEvent};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::{term_size, ScreenPos, ScreenSize, Viewport, Color};
use tinybit::widgets::Text;

enum State {
    Work,
    Chill,
}

fn main() {
    let work_min = env::args().skip(1).next().expect("provide some minutes please");
    let work_min = work_min.parse::<u64>().unwrap_or(20);

    let chill_min = env::args().skip(2).next().expect("provide some minutes please");
    let chill_min = chill_min.parse::<u64>().unwrap_or(5);

    let (width, _) = term_size().unwrap();

    let mut viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, 1));

    let target = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(target);

    let mut time = Duration::new(work_min * 60, 0);
    let mut now = Instant::now();
    let mut state = State::Work;

    for event in events(EventModel::Fps(1)) {
        match event {
            Event::Tick => {
                time -= now.elapsed();
                now = Instant::now();
                if time.as_secs() == 0 {
                    match state {
                        State::Work => {
                            state = State::Chill;
                            time = Duration::new(chill_min * 60, 0);
                        }
                        State::Chill => {
                            state = State::Work;
                            time = Duration::new(work_min * 60, 0);
                        }
                    }
                }

                let colour = match state {
                    State::Work => None,
                    State::Chill => Some(Color::Green),
                };

                let text = Text::new(format!(" M: {} S: {}", time.as_secs() / 60, time.as_secs() % 60), colour, None);
                viewport.draw_widget(&text, ScreenPos::zero());
                renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return,
            _ => {}
        }
    }
}
