use crate::algorithms::classical_ciphers::{vigenere_cipher_decrypt, vigenere_cipher_encrypt};
use crate::components::cipher_component::CipherComponent;
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub enum VigenereMode {
    #[default]
    Encrypt,
    Decrypt,
}

pub struct VigenereCipherComponent {
    input: String,
    key: String,
    output: String,
    mode: VigenereMode,
    current_field: usize,
}

impl Default for VigenereCipherComponent {
    fn default() -> Self {
        Self {
            input: String::new(),
            key: String::new(),
            output: String::new(),
            mode: VigenereMode::Encrypt,
            current_field: 0,
        }
    }
}

impl CipherComponent for VigenereCipherComponent {
    fn title(&self) -> &'static str {
        "Vigenere Cipher"
    }

    fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.mode = match self.mode {
                    VigenereMode::Encrypt => VigenereMode::Decrypt,
                    VigenereMode::Decrypt => VigenereMode::Encrypt,
                };
            }
            KeyCode::Up => {
                self.current_field = (self.current_field + 1) % 2;
            }
            KeyCode::Down => {
                self.current_field = (self.current_field + 1) % 2;
            }
            KeyCode::Char(c) => match self.current_field {
                0 => self.input.push(c),
                1 => self.key.push(c),
                _ => unreachable!(),
            },
            KeyCode::Backspace => match self.current_field {
                0 => {
                    self.input.pop();
                }
                1 => {
                    self.key.pop();
                }
                _ => unreachable!(),
            },
            KeyCode::Enter => {
                self.output = match self.mode {
                    VigenereMode::Encrypt => {
                        vigenere_cipher_encrypt(self.input.clone(), self.key.clone())
                    }
                    VigenereMode::Decrypt => {
                        vigenere_cipher_decrypt(self.input.clone(), self.key.clone())
                    }
                }
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

        let input_style = if self.current_field == 0 {
            Block::default()
                .title(">> Input Text")
                .borders(Borders::ALL)
        } else {
            Block::default().title("Input Text").borders(Borders::ALL)
        };
        Paragraph::new(self.input.as_str())
            .block(input_style)
            .render(layout[1], buf);

        let key_style = if self.current_field == 1 {
            Block::default()
                .title(">> Key (letters only)")
                .borders(Borders::ALL)
        } else {
            Block::default()
                .title("Key (letters only)")
                .borders(Borders::ALL)
        };
        Paragraph::new(self.key.as_str())
            .block(key_style)
            .render(layout[2], buf);

        Paragraph::new(self.output.as_str())
            .block(Block::default().title("Output").borders(Borders::ALL))
            .render(layout[3], buf);
    }
}
