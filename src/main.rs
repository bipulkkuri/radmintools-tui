use std::io;

use crossterm::event::{self, Event, KeyCode};

use base64::{decode_config, encode_config, URL_SAFE};
use md5;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{Block, List, ListState, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

/// App holds the state of the application
#[derive(Debug, Default)]
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// text
    message: String,
    /// index
    id: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

const ITEMS: [&str; 7] = [
    "0. MD5 HASH",
    "1. Base64 Encode",
    "2. Base64 Decode",
    "3. Base64-EncodeURL",
    "4. Base64-DecodeURL",
    "5. SAML-Encode",
    "6. SAML-Decode",
];

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
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
        // println!("{:?}", list_state.selected());
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
        //println!("{:?}", list_state.selected());
        //self.messages.push(self.input.value().into());
        self.message.push_str(self.input.value().into());
        if let Some(i) = list_state.selected() {
            // Perform some logic based on the selected index (e.g., updating an item).
            // println!("Selected index: {}", i);
            self.id = i;
        }

        // self.input.reset();
    }

    fn render(&mut self, frame: &mut Frame, list_state: &mut ListState) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
        let [top, main] = vertical.areas(frame.area());
        let [left, middle, right] = horizontal.areas(main);

        let title = Text::from_iter([Span::from("Admin ToolBox,Press q to exit e to start editing.Press Esc to stop editing, Enter to record the message").bold()]);
        frame.render_widget(title.centered(), top);

        // self.render_help_message(frame, left);

        self.render_admin_list(frame, left, list_state);
        self.render_input(frame, middle);
        self.render_messages(frame, right);
    }

    /// Render a list.
    fn render_admin_list(&self, frame: &mut Frame, area: Rect, list_state: &mut ListState) {
        let list = List::new(ITEMS)
            .style(Color::White)
            .block(Block::bordered().title("Pick"))
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, list_state);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_messages(&mut self, frame: &mut Frame, area: Rect) {
        let message = self.message.to_string();
        let mut process_msg = String::new();
        let mut title_msg = String::from("Output");
        if !message.is_empty() {
            if self.id == 0 {
                process_msg = self.compute_md5(message);
                title_msg.clear();
                title_msg.push_str(" MD5");

                self.message.clear();
            } else if self.id == 1 {
                process_msg = self.base64_encode(message.as_bytes());
                title_msg.clear();
                title_msg.push_str(" base64_encode");
            } else if self.id == 2 {
                // Base64 Decode logic (as an example, you could modify or add logic for other options)
                match self.base64_decode(&message) {
                    Ok(decoded) => {
                        process_msg = String::from_utf8_lossy(&decoded).to_string();
                        title_msg.clear();
                        title_msg.push_str(" base64_decode");
                        // Decode and display result
                    }
                    Err(_) => {
                        process_msg = "Error in Base64 decoding".to_string(); // Error message if decoding fails
                    }
                }
                self.message.clear();
            } else if self.id == 3 {
                process_msg = self.base64_url_encode(message.as_bytes());
                title_msg.clear();
                title_msg.push_str(" base64_url_encode");
                self.message.clear();
            } else if self.id == 4 {
                match self.base64_url_decode(&message) {
                    Ok(decoded) => {
                        process_msg = String::from_utf8_lossy(&decoded).to_string();
                        title_msg.clear();
                        title_msg.push_str(" base64_url_decode");
                    }
                    Err(_) => {
                        process_msg = "Error in Base64 URL decoding".to_string();
                    }
                }
                self.message.clear();
            } else {
                process_msg = message;
            };
        };

        let out_message = Paragraph::new(process_msg)
            .style(Color::White)
            .block(Block::bordered().title(title_msg))
            .scroll((0, 0))
            .wrap(Wrap { trim: true });
        // println!("selected {:?}", self.id);
        frame.render_widget(out_message, area);
    }

    /// Computes the MD5 hash of a string and returns the hash as a hexadecimal string.
    fn compute_md5(&self, input: String) -> String {
        let hash = md5::compute(input);
        // Return the hash as a hexadecimal string
        format!("{:x}", hash)
    }

    fn base64_encode(&self, input: &[u8]) -> String {
        base64::encode(input)
    }

    fn base64_decode(&self, input: &str) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(input)
    }
    // Base64 URL encoding
    fn base64_url_encode(&self, input: &[u8]) -> String {
        encode_config(input, URL_SAFE)
    }
    // Base64 URL decoding
    fn base64_url_decode(&self, input: &str) -> Result<Vec<u8>, base64::DecodeError> {
        decode_config(input, URL_SAFE)
    }
}
