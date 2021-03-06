extern crate tui;

use tui::backend::MouseBackend;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Widget;
use tui::Terminal;

struct Label<'a> {
    text: &'a str,
}

impl<'a> Default for Label<'a> {
    fn default() -> Label<'a> {
        Label { text: "" }
    }
}

impl<'a> Widget for Label<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, &Style::default());
    }
}

impl<'a> Label<'a> {
    fn text(&mut self, text: &'a str) -> &mut Label<'a> {
        self.text = text;
        self
    }
}

fn main() {
    let mut terminal = Terminal::new(MouseBackend::new().unwrap()).unwrap();
    let size = terminal.size().unwrap();
    terminal.clear().unwrap();
    terminal
        .draw(|mut f| {
            Label::default().text("Test").render(&mut f, size);
        })
        .unwrap();
}
