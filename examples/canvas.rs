extern crate termion;
extern crate tui;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use termion::event;
use termion::input::TermRead;

use tui::backend::MouseBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

struct App {
    size: Rect,
    x: f64,
    y: f64,
    ball: Rect,
    playground: Rect,
    vx: u16,
    vy: u16,
    dir_x: bool,
    dir_y: bool,
}

impl App {
    fn new() -> App {
        App {
            size: Default::default(),
            x: 0.0,
            y: 0.0,
            ball: Rect::new(10, 30, 10, 10),
            playground: Rect::new(10, 10, 100, 100),
            vx: 1,
            vy: 1,
            dir_x: true,
            dir_y: true,
        }
    }

    fn advance(&mut self) {
        if self.ball.left() < self.playground.left() || self.ball.right() > self.playground.right()
        {
            self.dir_x = !self.dir_x;
        }
        if self.ball.top() < self.playground.top() || self.ball.bottom() > self.playground.bottom()
        {
            self.dir_y = !self.dir_y;
        }

        if self.dir_x {
            self.ball.x += self.vx;
        } else {
            self.ball.x -= self.vx;
        }

        if self.dir_y {
            self.ball.y += self.vy;
        } else {
            self.ball.y -= self.vy
        }
    }
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    // Channels
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let clock_tx = tx.clone();

    // Input
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    // Tick
    thread::spawn(move || loop {
        clock_tx.send(Event::Tick).unwrap();
        thread::sleep(time::Duration::from_millis(500));
    });

    // App
    let mut app = App::new();

    // First draw call
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();
    app.size = terminal.size().unwrap();
    draw(&mut terminal, &app).unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }

        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => match input {
                event::Key::Char('q') => {
                    break;
                }
                event::Key::Down => {
                    app.y += 1.0;
                }
                event::Key::Up => {
                    app.y -= 1.0;
                }
                event::Key::Right => {
                    app.x += 1.0;
                }
                event::Key::Left => {
                    app.x -= 1.0;
                }

                _ => {}
            },
            Event::Tick => {
                app.advance();
            }
        }
        draw(&mut terminal, &app).unwrap();
    }

    terminal.show_cursor().unwrap();
}

fn draw(t: &mut Terminal<MouseBackend>, app: &App) -> Result<(), io::Error> {
    t.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(app.size);
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("World"))
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::White,
                    resolution: MapResolution::High,
                });
                ctx.print(app.x, -app.y, "You are here", Color::Yellow);
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .render(&mut f, chunks[0]);
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("List"))
            .paint(|ctx| {
                ctx.draw(&Line {
                    x1: f64::from(app.ball.left()),
                    y1: f64::from(app.ball.top()),
                    x2: f64::from(app.ball.right()),
                    y2: f64::from(app.ball.top()),
                    color: Color::Yellow,
                });
                ctx.draw(&Line {
                    x1: f64::from(app.ball.right()),
                    y1: f64::from(app.ball.top()),
                    x2: f64::from(app.ball.right()),
                    y2: f64::from(app.ball.bottom()),
                    color: Color::Yellow,
                });
                ctx.draw(&Line {
                    x1: f64::from(app.ball.right()),
                    y1: f64::from(app.ball.bottom()),
                    x2: f64::from(app.ball.left()),
                    y2: f64::from(app.ball.bottom()),
                    color: Color::Yellow,
                });
                ctx.draw(&Line {
                    x1: f64::from(app.ball.left()),
                    y1: f64::from(app.ball.bottom()),
                    x2: f64::from(app.ball.left()),
                    y2: f64::from(app.ball.top()),
                    color: Color::Yellow,
                });
            })
            .x_bounds([10.0, 110.0])
            .y_bounds([10.0, 110.0])
            .render(&mut f, chunks[1]);
    })
}
