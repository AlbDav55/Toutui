use crate::App;
use crate::app::AppView;
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::io;
use tui_textarea::{Input, Key, TextArea};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
};


impl App {
    pub fn search_active(&mut self) -> io::Result<String> {
        let stdout = io::stdout();
        let stdout = stdout.lock();

        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        let mut textarea = TextArea::default();
        textarea.set_block(
            Block::default()
            .borders(Borders::ALL)
            .title("Search")
            .border_style(Style::default().fg(Color::LightBlue)),
        );

        let size = term.size()?;
        let search_area = Rect {
            x: 1,
            y: size.height - 4,
            width: size.width - 2,
            height: 3,
        };

        loop {

            term.draw(|f| {
                f.render_widget(&textarea, search_area);
            })?;
            match crossterm::event::read()?.into() {
                Input { key: Key::Enter, .. } => {
                    self.search_mode = false;
                    self.search_query = textarea.lines().join("\n");
                    self.view_state = AppView::SearchBook;
                    self.list_state_search_results.select(Some(0));
                    break;
                }
                Input { key: Key::Esc, .. } => {
                    self.search_mode = false;
                    break;
                }
                input => {
                    textarea.input(input);
                }
            }
        }
        term.draw(|f| {
            let empty_block = Block::default();
            f.render_widget(empty_block, search_area); 
        })?;

        Ok(textarea.lines().join("\n"))

    }
}
