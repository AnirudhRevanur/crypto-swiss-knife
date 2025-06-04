mod components;
mod tabs;
use aes_only::{decrypt_with_aes, encrypt_with_aes};
use classical_ciphers::{caesar_cipher_decrypt, caesar_cipher_encrypt};
use gen_key_pair::generate_key_pair;
use rsa_hybrid::{decrypt_file, encrypt_file};
use tabs::classical::ClassicalTab;
mod aes_only;
mod classical_ciphers;
mod gen_key_pair;
mod rsa_hybrid;
use color_eyre::Result;
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
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

fn main() -> Result<()> {
    color_eyre::install()?;
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
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.selected_tab {
                    CryptoTab::Classical => {
                        if let Some(editing) = self.classical_tab.is_editing() {
                            if !editing {
                                match key.code {
                                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                                    _ => self.classical_tab.handle_event(key.code),
                                }
                            } else {
                                self.classical_tab.handle_event(key.code);
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                                KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                                _ => self.classical_tab.handle_event(key.code),
                            }
                        }
                    }
                    _ => match key.code {
                        KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        _ => {}
                    },
                }
            }
        }

        Ok(())
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
            CryptoTab::Symmetric => Paragraph::new("Coming soon").render(body, buf),
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
