use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use tui_big_text::{BigText, PixelSize};

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
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.game_state.start_game();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(50))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event)
                    }
                    _ => {}
                };
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(area);

        let inner = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(outer[1]);

        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .padding(Padding::new(1, 1, 0, 0))
            .title(Line::from(" zetalan ".bold()).centered());

        let header_text = Paragraph::new(vec![Line::from(vec![
            Span::styled("Score: ", Style::new().bold()),
            Span::raw(self.game_state.score.to_string()),
            Span::raw("    "),
            Span::styled("Time: ", Style::new().bold()),
            Span::raw(format!("{}s", self.game_state.clock_time())),
        ])]);

        frame.render_widget(header_text.block(header_block), outer[0]);

        let problem_text = BigText::builder()
            .pixel_size(PixelSize::Full)
            .style(Style::new().blue())
            .lines(vec![
                format!("{} = {}", self.game_state.current_problem, self.user_input).into(),
            ])
            .build();

        let problem_area = inner[0];
        let vcenter = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Min(0),
                Constraint::Percentage(20),
            ])
            .split(problem_area);

        frame.render_widget(problem_text, vcenter[1]);

        let sidebar_block = Block::default()
            .borders(Borders::ALL)
            .title(" Info ".bold())
            .padding(Padding::new(1, 1, 1, 1));

        let sidebar_text = Paragraph::new("Press q to quit");

        frame.render_widget(sidebar_text.block(sidebar_block), inner[1]);
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
                if self.user_input.chars().count() < 7 {
                    self.user_input.push(value);
                    self.check_answer();
                }
            }
            KeyCode::Backspace => {
                self.user_input.pop();
                self.check_answer();
            }
            _ => {}
        }
    }
}
