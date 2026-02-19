use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    errors::AppError,
    testsuite::{TestSuite, TomlLoader},
};

/// The `Action` enum defines the possible actions that can be dispatched to update the state of
/// the application.
enum Action {
    Exit,
    MoveIndexForward,
    CharacterTypingError(usize),
    CharacterTypingSuccess(usize),
}

/// The `Store` struct holds the state of the application,
#[derive(Debug)]
struct Store {
    exit: bool,
    typing_test: String,
    current_typing_index: usize,
    success_indices: Vec<usize>,
    error_indices: Vec<usize>,
}

impl Store {
    fn new(test_str: &str) -> Self {
        Self {
            exit: false,
            typing_test: String::from(test_str),
            current_typing_index: 0,
            success_indices: Vec::new(),
            error_indices: Vec::new(),
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Exit => self.exit = true,
            Action::MoveIndexForward => self.current_typing_index += 1,
            Action::CharacterTypingError(index) => self.error_indices.push(index),
            Action::CharacterTypingSuccess(index) => self.success_indices.push(index),
        }
    }
}

/// The `Dispatcher` struct is responsible for dispatching actions to the store to update the state of
#[derive(Debug)]
struct Dispatcher {
    store: Store,
}

impl Dispatcher {
    fn dispatch(&mut self, action: Action) {
        self.store.update(action);
    }
}

#[derive(Debug)]
pub struct App {
    dispatcher: Dispatcher,
}

impl App {
    fn new(typing_test_str: &str) -> Self {
        let store = Store::new(typing_test_str);
        let dispatcher = Dispatcher { store };
        Self { dispatcher }
    }

    fn store(&self) -> &Store {
        &self.dispatcher.store
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.store().exit {
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
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            crossterm::event::KeyCode::Esc => {
                self.dispatcher.dispatch(Action::Exit);
                return;
            }
            crossterm::event::KeyCode::Char(c) => {
                let state = self.store();
                let correct_char = state.typing_test.chars().nth(state.current_typing_index);
                if Some(c) == correct_char {
                    self.dispatcher
                        .dispatch(Action::CharacterTypingSuccess(state.current_typing_index));
                } else {
                    self.dispatcher
                        .dispatch(Action::CharacterTypingError(state.current_typing_index));
                }
                self.dispatcher.dispatch(Action::MoveIndexForward);
            }
            _ => {
                return;
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::new()
            .borders(Borders::all())
            .padding(ratatui::widgets::Padding {
                top: 2,
                bottom: 2,
                left: 4,
                right: 4,
            })
            .title_top(Line::from("Backspace").centered())
            .title_bottom(Line::from("Press <Esc> to exit").centered());
        let typing_test = self.store().typing_test.as_str();

        let inner_area = main_block.inner(area);
        main_block.render(area, buf);

        let spans: Vec<Span> = typing_test
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let style = if self.store().success_indices.contains(&i) {
                    ratatui::style::Style::default().fg(Color::White)
                } else if self.store().current_typing_index == i {
                    ratatui::style::Style::default()
                        .fg(Color::White)
                        .bg(Color::Rgb(100, 102, 105))
                        .add_modifier(Modifier::BOLD)
                } else if self.store().error_indices.contains(&i) {
                    ratatui::style::Style::default().fg(Color::Red)
                } else {
                    ratatui::style::Style::default().fg(Color::Rgb(100, 102, 105))
                };
                Span::styled(c.to_string(), style)
            })
            .collect();

        let text = Text::from(Line::from(spans));
        let p = Paragraph::new(text).wrap(ratatui::widgets::Wrap { trim: true });
        p.render(inner_area, buf);
    }
}

pub fn run(id: &str) -> Result<(), AppError> {
    println!("Running typing test {}...", id);
    let typing_tests_file = "./data/typing-tests/data.toml";
    let suite = TestSuite::<TomlLoader>::load(typing_tests_file)?;
    if let Some(typing_test) = suite.tests.into_iter().find(|test| test.id == id) {
        ratatui::run(|terminal| App::new(&typing_test.content).run(terminal))?;
        ratatui::restore();
        Ok(())
    } else {
        println!("Typing test with id '{}' not found.", id);
        Ok(())
    }
}
