use crate::components::vigenere::VigenereCipherComponent;
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
            components: vec![
                Box::new(CaesarCipherComponent::default()),
                Box::new(VigenereCipherComponent::default()),
            ],
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
            ClassicalMode::Selecting => self.render_selection_mode(area, buf),
            ClassicalMode::Editing => self.render_editing_mode(area, buf),
        }
    }

    /// Render the tab in selection mode, showing the list of available ciphers
    fn render_selection_mode(&self, area: Rect, buf: &mut Buffer) {
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

    /// Render the tab in editing mode, showing the selected cipher component
    fn render_editing_mode(&self, area: Rect, buf: &mut Buffer) {
        self.components[self.selected].render(area, buf);
    }

    pub fn is_editing(&self) -> Option<bool> {
        Some(matches!(self.mode, ClassicalMode::Editing))
    }

    pub fn handle_event(&mut self, key: KeyCode) {
        match self.mode {
            ClassicalMode::Selecting => self.handle_selection_mode(key),
            ClassicalMode::Editing => self.handle_editing_mode(key),
        }
    }

    /// Handle keyboard events in selection mode
    fn handle_selection_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Up => {
                self.selected = (self.selected + 1) % self.components.len();
            }
            KeyCode::Char('k') | KeyCode::Down => {
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
        }
    }

    /// Handle keyboard events in editing mode
    fn handle_editing_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.mode = ClassicalMode::Selecting;
            }
            _ => {
                self.components[self.selected].handle_event(key);
            }
        }
    }

    pub fn current_title(&self) -> &'static str {
        self.components[self.selected].title()
    }
}
