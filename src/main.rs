mod utils;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;
use utils::{base64, hex, html, json, md5};

const ITEMS: [&str; 10] = [
    "0. MD5 HASH",
    "1. Base64 Encode",
    "2. Base64 Decode",
    "3. Base64-EncodeURL",
    "4. Base64-DecodeURL",
    "5. Pretty Json",
    "6. HTML Encode",
    "7. HTML Decode",
    "8. String to Hex",
    "9. Hex to String",
];

#[derive(Debug, Default)]
struct App {
    input: Input,
    input_mode: InputMode,
    message: String,
    id: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let mut list_state = ListState::default();
        list_state.select_first();
        loop {
            terminal.draw(|frame| self.render(frame, &mut list_state))?;
            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => self.start_editing(),
                        KeyCode::Down => list_state.select_next(),
                        KeyCode::Up => list_state.select_previous(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => self.push_message(&mut list_state),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                }
            }
        }
    }

    fn start_editing(&mut self) {
        self.input.reset();
        self.input_mode = InputMode::Editing
    }

    fn stop_editing(&mut self) {
        self.input.reset();
        self.message.clear();
        self.id = 0;
        self.message = String::new();
        self.input_mode = InputMode::Normal;
    }

    fn push_message(&mut self, list_state: &mut ListState) {
        self.message.push_str(self.input.value().into());
        if let Some(i) = list_state.selected() {
            self.id = i;
        }
    }

    fn render(&mut self, frame: &mut Frame, list_state: &mut ListState) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
        let [top, main] = vertical.areas(frame.area());
        let [left, middle, right] = horizontal.areas(main);

        let title = Text::from_iter([Span::from("Admin ToolBox,Press q to exit e to start editing.Press Esc to stop editing, Enter to process the input").bold()]);
        frame.render_widget(title.centered(), top);

        self.render_admin_list(frame, left, list_state);
        self.render_input(frame, middle);
        self.render_messages(frame, right);
    }

    fn render_admin_list(&self, frame: &mut Frame, area: Rect, list_state: &mut ListState) {
        let list = List::new(ITEMS)
            .style(Color::White)
            // .block(Block::bordered().title(" Pick operation using UP/DOWN ↑↓ "))
            .block(
                Block::default()
                    //.style(Style::default().bg(Color::Black).fg(Color::Black))
                    .title(" Pick operation using UP/DOWN ↑↓ ")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded), //  .border_style(Style::default().bg(Color::White).fg(Color::Black)),
            )
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, list_state);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            //  .block(Block::bordered().title(" Input"));
            .block(
                Block::default()
                    //.style(Style::default().bg(Color::Black).fg(Color::Black))
                    .title(" Input")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded), //  .border_style(Style::default().bg(Color::White).fg(Color::Black)),
            );
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_messages(&mut self, frame: &mut Frame, area: Rect) {
        let message = self.message.to_string();
        let mut process_msg = String::new();
        let mut title_msg = String::from(" Output");
        self.process_input(message, &mut process_msg, &mut title_msg);
        let mut style = Style::default();
        if !process_msg.is_empty() {
            style = Color::LightCyan.into();
        }
        let out_message = Paragraph::new(process_msg)
            .style(style)
            //.block(Block::bordered().title(title_msg))
            .block(
                Block::default()
                    //.style(Style::default().bg(Color::Black).fg(Color::Black))
                    .title(title_msg)
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded), //  .border_style(Style::default().bg(Color::White).fg(Color::Black)),
            )
            .scroll((0, 0))
            .wrap(Wrap { trim: true });

        frame.render_widget(out_message, area);
    }

    fn process_input(&mut self, message: String, process_msg: &mut String, title_msg: &mut String) {
        if !message.is_empty() {
            title_msg.clear();
            match self.id {
                0 => {
                    *process_msg = md5::compute_md5(message.clone()).to_string();
                    title_msg.push_str(" MD5");
                }
                1 => {
                    *process_msg = base64::base64_encode_std(message.as_bytes());
                    title_msg.push_str(" base64_encode");
                }
                2 => match base64::base64_decode_std(&message) {
                    Ok(decoded) => {
                        *process_msg = String::from_utf8_lossy(&decoded).to_string();
                        title_msg.push_str(" base64_decode");
                    }
                    Err(_) => {
                        *process_msg = "Error in Base64 decoding".to_string();
                    }
                },
                3 => {
                    *process_msg = base64::base64_encode(message.as_bytes());
                    title_msg.push_str(" base64_url_encode");
                }
                4 => match base64::base64_decode(&message) {
                    Ok(decoded) => {
                        *process_msg = String::from_utf8_lossy(&decoded).to_string();
                        title_msg.push_str(" base64_url_decode");
                    }
                    Err(_) => {
                        *process_msg = "Error in Base64 URL decoding".to_string();
                    }
                },
                5 => {
                    match json::pretty_json_from_string(&message) {
                        Ok(pretty_json) => {
                            *process_msg = pretty_json;
                        }
                        Err(_) => {
                            *process_msg = "Error in JSON parsing".to_string();
                        }
                    }
                    title_msg.push_str(" pretty_json");
                }
                6 => {
                    *process_msg = html::encode_html_string(message);
                    title_msg.push_str(" HTML Encode");
                }
                7 => {
                    *process_msg = html::decode_html_string(message);
                    title_msg.push_str(" HTML Decode");
                }
                8 => {
                    *process_msg = hex::string_to_hex(&message);
                    title_msg.push_str(" String to Hex");
                }
                9 => {
                    *process_msg = hex::hex_to_string(&message);
                    title_msg.push_str(" Hex to String");
                }
                _ => {
                    *process_msg = message;
                }
            };
            self.message.clear();
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}
