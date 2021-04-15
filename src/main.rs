use std::env;
use std::time::{Duration, Instant};
use std::process::Command;

use tinybit::events::{events, Event, EventModel, KeyCode, KeyEvent, KeyModifiers};
use tinybit::render::{Renderer, StdoutTarget};
use tinybit::widgets::Text;
use tinybit::{term_size, Color, ScreenPos, ScreenSize, Viewport};

enum State {
    Work,
    Chill,
}

fn get_beep_command() -> Option<Command> {
    let command = env::args().skip(3).next()?;
    let args = env::args().skip(4).collect::<Vec<_>>();
    let mut command = Command::new(command);
    command.args(args);
    Some(command)
}

fn main() {
    if env::args().len() == 1 {
        println!("Usage:");
        println!(
            "{} [work-minutes] [break-minutes]",
            env::args().next().unwrap()
        );
        return;
    }

    let work_min = env::args()
        .skip(1)
        .next()
        .expect("Provide workperiod length in minutes");
    let work_min = work_min.parse::<u64>().unwrap_or(20);

    let chill_min = env::args()
        .skip(2)
        .next()
        .expect("Provide breakperiod length in minutes");
    let chill_min = chill_min.parse::<u64>().unwrap_or(5);
    
    let mut beep_command = get_beep_command();

    let (width, _) = term_size().unwrap();

    let mut viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, 1));

    let target = StdoutTarget::new().unwrap();
    let mut renderer = Renderer::new(target);

    let mut time = Duration::new(work_min * 60, 0);
    let mut now = Instant::now();
    let mut state = State::Work;

    let mut spinner = Spinner::default();

    for event in events(EventModel::Fps(1 * spinner.animation.len() as u64)) {
        match event {
            Event::Tick => {
                time -= now.elapsed();
                now = Instant::now();
                if time.as_secs() == 0 {
                    match state {
                        State::Work => {
                            state = State::Chill;
                            time = Duration::new(chill_min * 60, 0);
                            spinner = Spinner::beep();
                            beep_command.as_mut().map(|bc| bc.spawn());
                        }
                        State::Chill => {
                            state = State::Work;
                            time = Duration::new(work_min * 60, 0);
                            spinner = Spinner::default();
                        }
                    }
                }

                let spinner_colour = match state {
                    State::Work => (Some(Color::Green), Some(Color::Black)),
                    State::Chill => (Some(Color::Green), Some(Color::Black)),
                };

                let colour = match state {
                    State::Work => (None, Some(Color::Black)),
                    State::Chill => (Some(Color::Black), Some(Color::Green)),
                };

                let spinner_text = Text::new(
                    format!("{}[     ]", spinner.next_frame()),
                    spinner_colour.0,
                    spinner_colour.1,
                );

                let text = Text::new(
                    format!("{:02}:{:02}", time.as_secs() / 60, time.as_secs() % 60),
                    colour.0,
                    colour.1,
                );

                viewport.draw_widget(&spinner_text, ScreenPos::new(1, 0));
                viewport.draw_widget(&text, ScreenPos::new(3, 0));
                renderer.render(&mut viewport);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => return,
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return,
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
            }) if modifiers.contains(KeyModifiers::CONTROL) => return,
            _ => {}
        }
    }
}

struct Spinner {
    animation: Vec<char>,
    current_frame: usize,
}
impl Spinner {
    fn default() -> Spinner {
        Spinner {
            animation: vec!['⠢', '⠒', '⠔', '⠤'],
            current_frame: 0,
        }
    }

    fn beep() -> Spinner {
        Spinner {
            animation: vec!['⠶', ' '],
            current_frame: 0,
        }
    }

    fn next_frame(&mut self) -> char {
        let frame = self.current_frame;

        self.current_frame += 1;

        if self.current_frame > self.animation.len() - 1 {
            self.current_frame = 0;
        }

        self.animation[frame]
    }
}
