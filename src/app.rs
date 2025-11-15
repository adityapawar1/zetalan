use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::zetamac::GameState;

#[derive(Debug)]
pub struct App {
    game_state: GameState,
    user_input: String,
    current_screen: Screen,
    exit: bool,
}

#[derive(Debug)]
enum Screen {
    Menu,
    Lobby,
    Game,
    Settings,
}

impl Default for App {
    fn default() -> App {
        App {
            game_state: GameState::new(),
            user_input: String::new(),
            current_screen: Screen::Game,
            exit: false
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn check_answer(&mut self) {
        if self.game_state.is_correct(&self.user_input) {
            self.game_state.next_problem();
            self.user_input = String::new();
            self.game_state.score += 1;
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(value) => {
                self.user_input.push(value);
                self.check_answer();
            },
            KeyCode::Backspace => {
                self.user_input.pop();
                self.check_answer();
            },
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" ZetaLAN ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let problem_text = Text::from(
            format!("{} = {}", self.game_state.current_problem, self.user_input).yellow(),
        );

        Paragraph::new(problem_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
