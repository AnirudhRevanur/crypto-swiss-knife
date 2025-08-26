use crate::algorithms::hash::{hash_str_md5, hash_str_sha1, hash_str_sha256, hash_str_sha512};
use crate::components::cipher_component::CipherComponent;
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Clone, Copy, Debug)]
enum Algo {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl Default for Algo {
    fn default() -> Self {
        Self::Sha256
    }
}

impl Algo {
    fn next(self) -> Self {
        match self {
            Self::Md5 => Self::Sha1,
            Self::Sha1 => Self::Sha256,
            Self::Sha256 => Self::Sha512,
            Self::Sha512 => Self::Md5,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Md5 => "MD5",
            Self::Sha1 => "SHA-1",
            Self::Sha256 => "SHA-256",
            Self::Sha512 => "SHA-512",
        }
    }
}

#[derive(Default)]
pub struct HashComponent {
    input: String,
    output: String,
    algo: Algo,
}

impl CipherComponent for HashComponent {
    fn title(&self) -> &'static str {
        "Digests"
    }

    fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.algo = self.algo.next();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.output = match self.algo {
                    Algo::Md5 => hash_str_md5(&self.input),
                    Algo::Sha1 => hash_str_sha1(&self.input),
                    Algo::Sha256 => hash_str_sha256(&self.input),
                    Algo::Sha512 => hash_str_sha512(&self.input),
                };
            }
            _ => {}
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

        Paragraph::new(format!("Algorithm: {} (Tab to switch)", self.algo.name()))
            .block(Block::default().title("Mode").borders(Borders::ALL))
            .render(layout[0], buf);

        Paragraph::new(self.input.as_str())
            .block(Block::default().title("Input").borders(Borders::ALL))
            .render(layout[1], buf);

        Paragraph::new(self.output.as_str())
            .block(Block::default().title("Output (hex)").borders(Borders::ALL))
            .render(layout[2], buf);
    }
}
