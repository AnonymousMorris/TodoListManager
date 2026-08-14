use ratatui::prelude::*;
use ratatui::widgets::BorderType;
use ratatui::widgets::{Paragraph, Block};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommandPrompt {
    pub value: String,
    pub selected: bool,
}

pub enum Command {
    Clean,
    Save,
    Quit,
    SaveAndQuit,
}

impl CommandPrompt {
    pub fn new() -> CommandPrompt {
        return CommandPrompt {
            value: String::new(),
            selected: false,
        }
    }

    pub fn parse(&mut self) -> Option<Command> {
        self.selected = false;
        match std::mem::take(&mut self.value).as_str() {
            ":clean" => return Some(Command::Clean),
            ":w" => return Some(Command::Save),
            ":q" => return Some(Command::Quit),
            ":wq" => return Some(Command::SaveAndQuit),
            _ => {},
        }
        return None;
    }

    pub fn select_command(&mut self) {
        self.selected = true;
        self.value = String::from(":");
    }
}

impl Widget for &CommandPrompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let command_block = Block::bordered().border_type(BorderType::Plain);
        let mut cursor = Span::raw("");
        if self.selected {
            // command_block.border_type(BorderType::);
            cursor = Span::from(" ").bg(Color::White)
        }
        let cmd_line = Line::from(vec![Span::from(&self.value), cursor]);
        let component = Paragraph::new(cmd_line).left_aligned().block(command_block);

        Widget::render(component, area, buf);
    }
}
