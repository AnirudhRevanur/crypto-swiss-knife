use crate::components::{aes::AesCipherComponent, cipher_component::CipherComponent};
use ratatui::crossterm::event::KeyCode;
use ratatui::widgets::{Paragraph, Widget};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders},
};

pub struct SymmetricTab {
    selected: usize,
    mode: SymmetricMode,
    components: Vec<Box<dyn CipherComponent>>,
}

#[derive(Default)]
enum SymmetricMode {
    #[default]
    Selecting,
    Editing,
}

impl Default for SymmetricTab {
    fn default() -> Self {
        Self {
            selected: 0,
            mode: SymmetricMode::Selecting,
            components: vec![Box::new(AesCipherComponent::default())],
        }
    }
}

impl SymmetricTab {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            SymmetricMode::Selecting => self.render_selection_mode(area, buf),
            SymmetricMode::Editing => self.render_editing_mode(area, buf),
        }
    }

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

    fn render_editing_mode(&self, area: Rect, buf: &mut Buffer) {
        let [title_area, content_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        // Create a padded title area
        let title_area = Rect {
            x: area.x + 2,
            y: title_area.y,
            width: area.width.saturating_sub(4),
            height: title_area.height,
        };

        // Render the title with enhanced styling
        Paragraph::new(format!(" {} ", self.current_title()))
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .style(Style::default().bg(Color::Black))
            )
            .centered()
            .render(title_area, buf);

        // Add padding to the content area
        let content_area = Rect {
            x: area.x + 2,
            y: content_area.y,
            width: area.width.saturating_sub(4),
            height: content_area.height,
        };

        // Render the cipher component
        self.components[self.selected].render(content_area, buf);
    }

    pub fn is_editing(&self) -> Option<bool> {
        Some(matches!(self.mode, SymmetricMode::Editing))
    }

    pub fn handle_event(&mut self, key: KeyCode) {
        match self.mode {
            SymmetricMode::Selecting => self.handle_selection_mode(key),
            SymmetricMode::Editing => self.handle_editing_mode(key),
        }
    }

    fn handle_selection_mode(&mut self, key: KeyCode) {
        match key {
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
                self.mode = SymmetricMode::Editing;
            }
            _ => {}
        }
    }

    fn handle_editing_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.mode = SymmetricMode::Selecting;
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
