use crossterm::{event::{self, Event, KeyEvent, KeyEventKind}};
use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Block, Paragraph, Widget, Wrap}, DefaultTerminal};

use crate::{errors::AppError, testsuite::{TestSuite, TomlLoader}};

/// The `Action` enum defines the possible actions that can be dispatched to update the state of
/// the application.
enum Action {
    Exit,
}

/// The `Store` struct holds the state of the application, 
#[derive(Debug)]
struct Store {
    exit: bool,
    typing_test: String,
}

impl Store {
    fn new(test_str: &str) -> Self {
        Self {
            exit: false,
            typing_test: String::from(test_str),
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Exit => self.exit = true,
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
        Self {
            dispatcher,
       }
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
            Event::Key(key_event) => if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            crossterm::event::KeyCode::Esc => {
                self.dispatcher.dispatch(Action::Exit);
            }
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::bordered()
            .padding(ratatui::widgets::Padding { top: 2, bottom: 2, left: 4, right: 4 })
            .title_top(Line::from("Backspace").centered())
            .title_bottom(Line::from("Press <Esc> to exit").centered());
        // render each line of the typing test as a separate paragraph
        let typing_test = self.store().typing_test.as_str();
        Paragraph::new(typing_test.green())
            .wrap(Wrap { trim: true })
            .block(main_block)
            .render(area, buf);
}
}

pub fn run(id: &str) -> Result<(), AppError> {
    println!("Running typing test {}...", id);
    let typing_tests_file = "./data/typing-tests/data.toml";
    let suite = TestSuite::<TomlLoader>::load(typing_tests_file)?;
    if let Some(typing_test) = suite.tests.into_iter()
        .find(|test| test.id == id) {
        ratatui::run(|terminal| {
                App::new(&typing_test.content).run(terminal)
        })?;
        Ok(())
    } else {
        println!("Typing test with id '{}' not found.", id);
        Ok(())
    }
}
