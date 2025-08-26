# Components (TUI)

UI widgets for cipher interactions. Keep state and rendering here; delegate all crypto to `algorithms`.

## Contract
Each component implements the `CipherComponent` trait:

```rust
pub trait CipherComponent {
    fn title(&self) -> &'static str;
    fn handle_event(&mut self, key: ratatui::crossterm::event::KeyCode);
    fn render(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer);
}
```

## Add a new component
1. Copy the template below to `src/components/my_cipher.rs`.
2. Implement `CipherComponent` methods.
3. Register in `components/mod.rs` with `pub mod my_cipher;`.
4. Add it to a tab (e.g., `tabs/classical.rs` or `tabs/symmetric.rs`).

## Template
```rust
use crate::components::cipher_component::CipherComponent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use ratatui::crossterm::event::KeyCode;

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
                // Call into algorithms::my_cipher here
                self.output = format!("processed:{}:{}", self.key, self.input);
            }
            _ => {}
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Render your UI here (blocks/paragraphs/layout)
        Widget::render(ratatui::widgets::Paragraph::new(self.output.as_str()), area, buf);
    }
}
```
