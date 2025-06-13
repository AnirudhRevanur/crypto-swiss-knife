use crate::algorithms::aes_only::{decrypt_with_aes, encrypt_with_aes};
use crate::components::cipher_component::CipherComponent;
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub enum AesMode {
    #[default]
    Encrypt,
    Decrypt,
}

pub struct AesCipherComponent {
    input_path: String,
    key_path: String,
    output_path: String,
    mode: AesMode,
    status_message: String,
    current_field: usize,
}

impl Default for AesCipherComponent {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            key_path: String::new(),
            output_path: String::new(),
            mode: AesMode::Encrypt,
            status_message: String::new(),
            current_field: 0,
        }
    }
}

impl CipherComponent for AesCipherComponent {
    fn title(&self) -> &'static str {
        "AES Cipher"
    }

    fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.mode = match self.mode {
                    AesMode::Encrypt => AesMode::Decrypt,
                    AesMode::Decrypt => AesMode::Encrypt,
                };
            }
            KeyCode::Up => {
                self.current_field = (self.current_field + 2) % 3;
            }
            KeyCode::Down => {
                self.current_field = (self.current_field + 1) % 3;
            }
            KeyCode::Char(c) => {
                match self.current_field {
                    0 => self.input_path.push(c),
                    1 => self.key_path.push(c),
                    2 => self.output_path.push(c),
                    _ => unreachable!(),
                }
            }
            KeyCode::Backspace => {
                match self.current_field {
                    0 => { self.input_path.pop(); }
                    1 => { self.key_path.pop(); }
                    2 => { self.output_path.pop(); }
                    _ => unreachable!(),
                }
            }
            KeyCode::Enter => {
                self.process_files();
            }
            _ => {}
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

        // Mode display
        Paragraph::new(format!("Mode: {:?} (Tab to switch)", self.mode))
            .block(Block::default().title("Mode").borders(Borders::ALL))
            .render(layout[0], buf);

        // Input path
        let input_style = if self.current_field == 0 {
            Block::default().title("Input File Path").borders(Borders::ALL)
        } else {
            Block::default().title("Input File Path").borders(Borders::ALL)
        };
        Paragraph::new(self.input_path.as_str())
            .block(input_style)
            .render(layout[1], buf);

        // Key path
        let key_style = if self.current_field == 1 {
            Block::default().title("Key File Path").borders(Borders::ALL)
        } else {
            Block::default().title("Key File Path").borders(Borders::ALL)
        };
        Paragraph::new(self.key_path.as_str())
            .block(key_style)
            .render(layout[2], buf);

        // Output path
        let output_style = if self.current_field == 2 {
            Block::default().title("Output File Path").borders(Borders::ALL)
        } else {
            Block::default().title("Output File Path").borders(Borders::ALL)
        };
        Paragraph::new(self.output_path.as_str())
            .block(output_style)
            .render(layout[3], buf);

        // Status message
        Paragraph::new(self.status_message.as_str())
            .block(Block::default().title("Status").borders(Borders::ALL))
            .render(layout[4], buf);
    }
}

impl AesCipherComponent {
    pub fn process_files(&mut self) {
        match self.mode {
            AesMode::Encrypt => {
                match encrypt_with_aes(&self.input_path, &self.output_path, &self.key_path) {
                    Ok(_) => {
                        self.status_message = format!(
                            "Encrypted to: {} and Key is saved to: {}",
                            self.output_path, self.key_path
                        );
                    }
                    Err(err) => {
                        self.status_message = format!("Encryption failed: {err}");
                    }
                }
            }

            AesMode::Decrypt => {
                match decrypt_with_aes(&self.input_path, &self.output_path, &self.key_path) {
                    Ok(_) => {
                        self.status_message =
                            format!("Decrypted file is stored at: {}", self.output_path);
                    }
                    Err(err) => {
                        self.status_message = format!("Decryption failed: {err}");
                    }
                }
            }
        }
    }
}
