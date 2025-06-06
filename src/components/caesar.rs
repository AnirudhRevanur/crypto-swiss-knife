use crate::algorithms::classical_ciphers::{caesar_cipher_decrypt, caesar_cipher_encrypt};
use crate::components::cipher_component::CipherComponent;
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub enum CaesarMode {
    #[default]
    Encrypt,
    Decrypt,
}

pub struct CaesarCipherComponent {
    input: String,
    key: i32,
    output: String,
    mode: CaesarMode,
}

impl Default for CaesarCipherComponent {
    fn default() -> Self {
        Self {
            input: String::new(),
            key: 3,
            output: String::new(),
            mode: CaesarMode::Encrypt,
        }
    }
}

impl CipherComponent for CaesarCipherComponent {
    fn title(&self) -> &'static str {
        "Caesar Cipher"
    }

    fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Tab => {
                self.mode = match self.mode {
                    CaesarMode::Encrypt => CaesarMode::Decrypt,
                    CaesarMode::Decrypt => CaesarMode::Encrypt,
                };
            }
            KeyCode::Enter => {
                self.output = match self.mode {
                    CaesarMode::Encrypt => caesar_cipher_encrypt(self.input.clone(), self.key),
                    CaesarMode::Decrypt => caesar_cipher_decrypt(self.input.clone(), self.key),
                }
            }
            KeyCode::Up => {
                self.key = self.key.wrapping_add(1);
            }
            KeyCode::Down => {
                self.key = self.key.wrapping_sub(1);
            }
            _ => {}
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

        Paragraph::new(format!("Mode: {:?} (Tab to switch)", self.mode))
            .block(Block::default().title("Mode").borders(Borders::ALL))
            .render(layout[0], buf);

        Paragraph::new(self.input.as_str())
            .block(Block::default().title("Input").borders(Borders::ALL))
            .render(layout[1], buf);

        Paragraph::new(format!("Key: {:?}", self.key))
            .block(Block::default().title("Key").borders(Borders::ALL))
            .render(layout[2], buf);

        Paragraph::new(self.output.as_str())
            .block(Block::default().title("Output").borders(Borders::ALL))
            .render(layout[3], buf);
    }
}
