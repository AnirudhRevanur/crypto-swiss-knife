//! Template for a new tab category
//! Copy this as `my_category.rs`, then export it in `tabs/mod.rs` and wire it in `main.rs`.

use crate::components::cipher_component::CipherComponent;
use ratatui::crossterm::event::KeyCode;
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::{Paragraph, Widget}};

pub struct MyCategoryTab {
    selected: usize,
    mode: MyCategoryMode,
    components: Vec<Box<dyn CipherComponent>>,
}

#[derive(Default)]
enum MyCategoryMode {
    #[default]
    Selecting,
    Editing,
}

impl Default for MyCategoryTab {
    fn default() -> Self {
        Self {
            selected: 0,
            mode: MyCategoryMode::Selecting,
            components: vec![/* Box::new(MyCipherComponent::default()) */],
        }
    }
}

impl MyCategoryTab {
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.mode {
            MyCategoryMode::Selecting => self.render_selection_mode(area, buf),
            MyCategoryMode::Editing => self.render_editing_mode(area, buf),
        }
    }

    fn render_selection_mode(&self, area: Rect, buf: &mut Buffer) {
        let items = self.components.iter().enumerate().map(|(i, c)| {
            let marker = if i == self.selected { "> " } else { "  " };
            format!("{}{}", marker, c.title())
        }).collect::<Vec<_>>().join("\n");
        Paragraph::new(items).render(area, buf);
    }

    fn render_editing_mode(&self, area: Rect, buf: &mut Buffer) {
        let [title_area, content_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).areas(area);

        Paragraph::new(format!(" {} ", self.current_title())).centered().render(title_area, buf);
        self.components[self.selected].render(content_area, buf);
    }

    pub fn is_editing(&self) -> Option<bool> {
        Some(matches!(self.mode, MyCategoryMode::Editing))
    }

    pub fn handle_event(&mut self, key: KeyCode) {
        match self.mode {
            MyCategoryMode::Selecting => self.handle_selection_mode(key),
            MyCategoryMode::Editing => self.handle_editing_mode(key),
        }
    }

    fn handle_selection_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                self.selected = (self.selected + 1) % self.components.len();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected == 0 { self.selected = self.components.len().saturating_sub(1); } else { self.selected -= 1; }
            }
            KeyCode::Enter => { self.mode = MyCategoryMode::Editing; }
            _ => {}
        }
    }

    fn handle_editing_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => { self.mode = MyCategoryMode::Selecting; }
            _ => { self.components[self.selected].handle_event(key); }
        }
    }

    pub fn current_title(&self) -> &'static str {
        self.components[self.selected].title()
    }
}
