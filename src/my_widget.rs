use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    style::Style,
};

pub struct MyWidget<'a> {
    text: &'a str,
}

impl<'a> Default for MyWidget<'a> {
    fn default() -> MyWidget<'a> {
        MyWidget { text: "" }
    }
}

impl<'a> Widget for MyWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), self.text, Style::default());
    }
}

impl<'a> MyWidget<'a> {
    pub fn text(mut self, text: &'a str) -> MyWidget<'a> {
        self.text = text;
        self
    }
}

