use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Paragraph, Widget}, DefaultTerminal};

use crate::{errors::AppError, testsuite::{TestSuite, TomlLoader}};


#[derive(Debug)]
pub struct App {
    exit: bool,
    pub typing_test: String,
}

impl App {
    fn new(test_str: &str) -> Self {
        Self {
            exit: false,
            typing_test: String::from(test_str),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            crossterm::event::KeyCode::Char('q') => {
                self.exit = true;
            }
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Backspace".bold());
        Paragraph::new(title).render(area, buf);
    }
}

pub fn run(id: &str) -> Result<(), AppError> {
    println!("Running typing test {}...", id);
    let typing_tests_file = "./data/typing-tests/data.toml";
    let suite = TestSuite::<TomlLoader>::load(typing_tests_file)?;
    if let Some(typing_test) = suite.tests.into_iter()
        .find(|test| test.id == id) {
        ratatui::run(|terminal| App::new(typing_test.content.as_str()).run(terminal))?;
        Ok(())
    } else {
        println!("Typing test with id '{}' not found.", id);
        Ok(())
    }
}
