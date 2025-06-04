use ratatui::crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait CipherComponent {
    fn title(&self) -> &'static str;
    fn handle_event(&mut self, key: KeyCode);
    fn render(&self, area: Rect, buf: &mut Buffer);
}
