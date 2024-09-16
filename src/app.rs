use anyhow::Result;
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, prelude::CrosstermBackend, widgets::Widget, Terminal};
use std::io::Stderr;

#[derive(Default, Debug)]
pub struct App {
    mode: Mode,
    scripts: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stderr>>,
        scripts: Vec<String>,
    ) -> Result<()> {
        self.scripts = scripts;

        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn handle_events(&self) -> Result<()> {
        todo!()
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [left, right] = layout.areas(area);
    }
}
