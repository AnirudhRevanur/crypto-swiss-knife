mod components;
mod tabs;
use algorithms::aes_only::{decrypt_with_aes, encrypt_with_aes};
use algorithms::classical_ciphers::{caesar_cipher_decrypt, caesar_cipher_encrypt};
use algorithms::gen_key_pair::generate_key_pair;
use algorithms::rsa_hybrid::{decrypt_file, encrypt_file};
use tabs::classical::ClassicalTab;
mod algorithms;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize, palette::tailwind},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
};
use std::error::Error;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use tabs::symmetric::SymmetricTab;

fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    state: AppState,
    selected_tab: CryptoTab,
    classical_tab: ClassicalTab,
    symmetric_tab: SymmetricTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum CryptoTab {
    #[default]
    #[strum(to_string = "Classical")]
    Classical,
    #[strum(to_string = "Symmetric")]
    Symmetric,
    #[strum(to_string = "Asymmetric")]
    Asymmetric,
    #[strum(to_string = "Misc")]
    Misc,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_key_press(key.code);
            }
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key_code: KeyCode) {
        match self.selected_tab {
            CryptoTab::Classical => self.handle_classical_tab_events(key_code),
            CryptoTab::Symmetric => self.handle_symmetric_tab_events(key_code),
            _ => self.handle_global_events(key_code),
        }
    }

    fn handle_classical_tab_events(&mut self, key_code: KeyCode) {
        if let Some(editing) = self.classical_tab.is_editing() {
            if !editing {
                self.handle_global_events(key_code);
            }
            self.classical_tab.handle_event(key_code);
        } else {
            self.handle_global_events(key_code);
        }
    }

    fn handle_symmetric_tab_events(&mut self, key_code: KeyCode) {
        if let Some(editing) = self.symmetric_tab.is_editing() {
            if !editing {
                self.handle_global_events(key_code);
            }
            self.symmetric_tab.handle_event(key_code);
        } else {
            self.handle_global_events(key_code);
        }
    }

    fn handle_global_events(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
            KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            _ => {}
        }
    }

    fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl CryptoTab {
    fn previous(self) -> Self {
        let idx = self as usize;
        Self::from_repr(idx.saturating_sub(1)).unwrap_or(self)
    }

    fn next(self) -> Self {
        let idx = self as usize;
        Self::from_repr(idx + 1).unwrap_or(self)
    }

    fn palette(self) -> tailwind::Palette {
        match self {
            Self::Classical => tailwind::BLUE,
            Self::Symmetric => tailwind::EMERALD,
            Self::Asymmetric => tailwind::INDIGO,
            Self::Misc => tailwind::PINK,
        }
    }

    fn title(self) -> Line<'static> {
        format!("  {}  ", self)
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header, body, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [tabs_area, title_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(25)]).areas(header);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        // self.selected_tab.render(body, buf);
        match self.selected_tab {
            CryptoTab::Classical => self.classical_tab.render(body, buf),
            CryptoTab::Symmetric => self.symmetric_tab.render(body, buf),
            CryptoTab::Asymmetric => Paragraph::new("Coming soon").render(body, buf),
            CryptoTab::Misc => Paragraph::new("Coming soon").render(body, buf),
        }
        render_footer(footer, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = CryptoTab::iter().map(CryptoTab::title);
        Tabs::new(titles)
            .select(self.selected_tab as usize)
            .highlight_style((Color::default(), self.selected_tab.palette().c700))
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

impl Widget for CryptoTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = match self {
            Self::Classical => "Caesar",
            Self::Symmetric => "AES",
            Self::Asymmetric => "RSA + AES Hybrid",
            Self::Misc => "Hashing",
        };
        Paragraph::new(content)
            .block(
                Block::bordered()
                    .border_set(symbols::border::ROUNDED)
                    .border_style(self.palette().c700)
                    .padding(Padding::horizontal(1)),
            )
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Crypto Swiss Knife".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("Arrow keys or h/l to switch | q or Esc to quit")
        .centered()
        .render(area, buf);
}
