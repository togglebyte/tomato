use std::env;
use std::time::{Instant, Duration};

use tinybit::events::{events, Event, EventModel, KeyCode, KeyEvent};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::{term_size, ScreenPos, ScreenSize, Viewport};
use tinybit::widgets::Text;

fn main() {
    let min = env::args().skip(1).next().expect("provide some minutes please");
    let min = min.parse::<u64>().unwrap();

    let (width, _) = term_size().unwrap();

    let mut viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, 1));

    let target = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(target);

    let mut time = Duration::new(min * 60, 0);
    let mut now = Instant::now();

    for event in events(EventModel::Fps(1)) {
        match event {
            Event::Tick => {
                time -= now.elapsed();
                now = Instant::now();
                let text = Text::new(format!(" M: {} S: {}", time.as_secs() / 60, time.as_secs() % 60), None, None);
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
