//! Template for a new cipher component
//! Copy this file as `my_cipher.rs`, add `pub mod my_cipher;` to `components/mod.rs`,
//! and add the component to a tab (e.g., `tabs/classical.rs`).

use crate::components::cipher_component::CipherComponent;
use ratatui::crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Default)]
pub struct MyCipherComponent {
    input: String,
    key: String,
    output: String,
}

impl CipherComponent for MyCipherComponent {
    fn title(&self) -> &'static str { "My Cipher" }

    fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => { self.input.pop(); },
            KeyCode::Enter => {
                // TODO: call algorithms::my_cipher
                self.output = String::new();
            }
            _ => {}
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(ratatui::widgets::Paragraph::new(self.output.as_str()), area, buf);
    }
}
