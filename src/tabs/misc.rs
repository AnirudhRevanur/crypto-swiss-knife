use crate::components::{cipher_component::CipherComponent, hash::HashComponent};
use ratatui::crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Paragraph, Widget},
};

pub struct MiscTab {
    selected: usize,
    mode: MiscMode,
    components: Vec<Box<dyn CipherComponent>>,
}

#[derive(Default)]
enum MiscMode {
    #[default]
    Selecting,
    Editing,
}

impl Default for MiscTab {
    fn default() -> Self {
        Self {
            selected: 0,
            mode: MiscMode::Selecting,
            components: vec![Box::new(HashComponent::default())],
        }
    }
}

impl MiscTab {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            MiscMode::Selecting => self.render_selection(area, buf),
            MiscMode::Editing => self.render_editing(area, buf),
        }
    }

    fn render_selection(&self, area: Rect, buf: &mut Buffer) {
        let items = self
            .components
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let marker = if i == self.selected { "> " } else { "  " };
                format!("{}{}", marker, c.title())
            })
            .collect::<Vec<_>>()
            .join("\n");
        Paragraph::new(items).render(area, buf);
    }

    fn render_editing(&self, area: Rect, buf: &mut Buffer) {
        let [title_area, content_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        Paragraph::new(format!(" {} ", self.current_title()))
            .centered()
            .render(title_area, buf);

        self.components[self.selected].render(content_area, buf);
    }

    pub fn is_editing(&self) -> Option<bool> {
        Some(matches!(self.mode, MiscMode::Editing))
    }

    pub fn handle_event(&mut self, key: KeyCode) {
        match self.mode {
            MiscMode::Selecting => match key {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.selected = (self.selected + 1) % self.components.len();
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.selected == 0 {
                        self.selected = self.components.len() - 1;
                    } else {
                        self.selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    self.mode = MiscMode::Editing;
                }
                _ => {}
            },
            MiscMode::Editing => match key {
                KeyCode::Esc => {
                    self.mode = MiscMode::Selecting;
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
