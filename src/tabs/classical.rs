use crate::components::{caesar::CaesarCipherComponent, cipher_component::CipherComponent};
use ratatui::crossterm::event::KeyCode;
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{buffer::Buffer, layout::Rect};

pub struct ClassicalTab {
    selected: usize,
    mode: ClassicalMode,
    components: Vec<Box<dyn CipherComponent>>,
}

impl Default for ClassicalTab {
    fn default() -> Self {
        Self {
            selected: 0,
            mode: ClassicalMode::Selecting,
            components: vec![Box::new(CaesarCipherComponent::default())],
        }
    }
}

#[derive(Default)]
enum ClassicalMode {
    #[default]
    Selecting,
    Editing,
}

impl ClassicalTab {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            ClassicalMode::Selecting => {
                let items = self
                    .components
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        let marker = if i == self.selected { "> " } else { " " };
                        format!("{}{}", marker, c.title())
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                Paragraph::new(items).render(area, buf);
            }
            ClassicalMode::Editing => {
                self.components[self.selected].render(area, buf);
            }
        }
    }

    pub fn is_editing(&self) -> Option<bool> {
        Some(matches!(self.mode, ClassicalMode::Editing))
    }

    pub fn handle_event(&mut self, key: KeyCode) {
        match self.mode {
            ClassicalMode::Selecting => match key {
                KeyCode::Char('j') => {
                    self.selected = (self.selected + 1) % self.components.len();
                }
                KeyCode::Char('k') => {
                    if self.selected == 0 {
                        self.selected = self.components.len() - 1;
                    } else {
                        self.selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    self.mode = ClassicalMode::Editing;
                }
                _ => {}
            },
            ClassicalMode::Editing => match key {
                KeyCode::Esc => {
                    self.mode = ClassicalMode::Selecting;
                }
                _ => {
                    self.components[self.selected].handle_event(key);
                }
            },
        }
    }

    pub fn current_title(&self) -> &'static str {
        self.components[self.selected].title()
    }
}
