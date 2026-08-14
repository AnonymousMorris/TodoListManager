use ratatui::widgets::{Block, Paragraph};
use serde::{Deserialize, Serialize};
use core::fmt;
use std::cmp::min;
use crate::command::Command;
use crate::todolist::TodoList;
use crate::config;
use crate::command::CommandPrompt;

#[derive(Serialize, Deserialize)]
pub struct App {
    pub mode: Mode,
    pub command_prompt: CommandPrompt,
    pub todolists: Vec<TodoList>,
    pub todolist_idx: Option<usize>,
    pub nominal_line_num: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    Insert, 
    Normal,
    Visual,
    Command,
}

impl fmt::Display for Mode {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            Mode::Insert => write!(f, "Insert Mode"),
            Mode::Normal => write!(f, "Normal Mode"),
            Mode::Visual => write!(f, "Visual Mode"),
            Mode::Command => write!(f, "Command Mode"),
        }
    }
}

impl App {
    pub fn new() -> App{
        App{
            mode: Mode::Normal,
            command_prompt: CommandPrompt::new(),
            todolists: vec![TodoList::new()],
            todolist_idx: None,
            nominal_line_num: 0,
        }
    }

    /////////////
    // Helpers
    /////////////
    
    pub fn current_todolist(&mut self) -> Option<&mut TodoList> {
        if let Some(idx) = self.todolist_idx {
            return Some(&mut self.todolists[idx]);
        }
        return None;
    }

    fn unselect_todolist(&mut self, idx: usize) {
        assert!(idx < self.todolists.len());
        self.todolists[idx].unselect(self.mode);
    }

    fn select_todolist(&mut self, idx: usize) {
        let len = self.todolists.len();
        if len == 0 {
            self.todolist_idx = None;
        }
        else {
            let new_idx = min(idx, len - 1);
            self.todolist_idx = Some(new_idx);
            self.todolists[new_idx].select(self.nominal_line_num, self.mode);
        }
    }

    ////////////////
    // Creation
    ////////////////

    pub fn create_todo(&mut self) -> Option<usize> {
        assert!(self.mode == Mode::Normal);
        if let Some(todolist) = self.current_todolist() {
            let pos = todolist.create_todo_below();
            todolist.select_todo(pos);
            self.toggle_editing();
            return Some(pos);
        }
        return None;
    }

    pub fn create_todolist(&mut self) -> usize {
        assert!(self.mode == Mode::Normal);
        let mut pos = 0;
        if let Some(idx) = self.todolist_idx {
            pos = idx + 1;
            self.unselect_todolist(idx);
        }
        self.todolists.push(TodoList::new());
        self.move_todolist(self.todolists.len() - 1, pos);
        self.select_todolist(pos);
        return pos
    }

    ////////////////
    // App Navigation
    ////////////////

    pub fn move_left (&mut self) {
        assert!(self.mode == Mode::Normal);
        if let Some(idx) = self.todolist_idx {
            self.unselect_todolist(idx);
            self.select_todolist(idx.saturating_sub(1));
        }
    }

    pub fn move_right (&mut self) {
        assert!(self.mode == Mode::Normal);
        if let Some(idx) = self.todolist_idx {
            let len = self.todolists.len();
            self.unselect_todolist(idx);
            self.select_todolist(min(idx + 1, len - 1));
        }
        else {
            self.todolist_idx = Some(0);
        }
    }

    pub fn move_up(&mut self) {
        let mode = self.mode;
        if let Some(todolist) = self.current_todolist() {
            todolist.move_selection_up(mode);
            self.nominal_line_num = todolist.todo_idx.unwrap_or(0);
        }
    }

    pub fn move_down(&mut self) {
        let mode = self.mode;
        if let Some(todolist) = self.current_todolist() {
            todolist.move_selection_down(mode);
            self.nominal_line_num = todolist.todo_idx.unwrap_or(0);
        }
    }

    ////////////////
    // Shuffling lists 
    ////////////////
    
    fn move_todolist(&mut self, a: usize, b: usize) {
        assert!(a < self.todolists.len());
        assert!(b < self.todolists.len());
        if a < b {
            for i in a..b {
                self.todolists.swap(i, i+1);
            }
        }
        else {
            for i in (b..a).rev() {
                self.todolists.swap(i, i+1);
            }
        }
    }

    pub fn move_todolist_left(&mut self) {
        if let Some(todolist_idx) = self.todolist_idx {
            if todolist_idx > 0 {
                self.move_todolist(todolist_idx, todolist_idx - 1);
                self.move_left();
            }
        }
    }

    pub fn move_todolist_right (&mut self) {
        if let Some(todolist_idx) = self.todolist_idx {
            if todolist_idx < self.todolists.len() - 1 {
                self.move_todolist(todolist_idx, todolist_idx + 1);
                self.move_right();
            }
        }
    }

    pub fn move_todo_up(&mut self) {
        let mode = self.mode;
        if let Some(todolist) = self.current_todolist() {
            todolist.move_todo_up(mode);
        }
    }

    pub fn move_todo_down(&mut self) {
        let mode = self.mode;
        if let Some(todolist) = self.current_todolist() {
            todolist.move_todo_down(mode);
        }
    }

    //////////////
    // Todo management
    //////////////

    pub fn delete_todolist(&mut self) {
        if let Some(idx) = self.todolist_idx{
            self.todolists.remove(idx);
            self.select_todolist(idx);

            let mode = self.mode;
            let line_num = self.nominal_line_num;
            if let Some(todolist) = self.current_todolist() {
                todolist.select(line_num, mode);
            }
        }
    }
    
    pub fn delete_todo(&mut self) {
        let mode = self.mode;
        if let Some(todolist) = self.current_todolist() {
            todolist.delete_todo(mode);
        }
        self.mode = Mode::Normal;
    }

    pub fn toggle_completed (&mut self) {
        let mode = self.mode;
        assert!(mode == Mode::Normal || mode == Mode::Visual);
        if let Some(todolist) = self.current_todolist() {
            todolist.toggle_completed(mode);
        }
    }

    ////////////////
    // Commands
    ////////////////

    // Returns true if app should exit
    pub fn execute(&mut self) -> bool {
        // We use a should save variable so that we can 
        // defer the save till end of function so we capture
        // the exit command mode state
        let mut should_save: bool = false;
        let mut should_exit: bool = false;
        if let Some(cmd) = self.command_prompt.parse() {
            match cmd {
                Command::Clean => {
                    self.clean();
                }
                Command::Save => {
                    should_save = true;
                }
                Command::Quit => {
                    should_exit = true;
                }
                Command::SaveAndQuit => {
                    should_save = true;
                    should_exit = true;
                }
            }
        }
        self.mode = Mode::Normal;
        if let Some(idx) = self.todolist_idx {
            self.select_todolist(idx);
        }
        if should_save {
            config::save(self);
        }
        return should_exit;
    }

    pub fn clean(&mut self) {
        let mode = self.mode;
        for todolist in &mut self.todolists {
            todolist.delete_completed_todos(mode);
        }
    }

    pub fn command_backspace(&mut self) {
        let mode = self.mode;
        assert!(mode == Mode::Command);
        self.command_prompt.value.pop();
    }

    pub fn command_char(&mut self, c: char) {
        let mode = self.mode;
        assert!(mode == Mode::Command);
        self.command_prompt.value.push(c);
    }

    pub fn toggle_command (&mut self) {
        match self.mode {
            Mode::Normal|Mode::Visual => {
                if let Some(idx) = self.todolist_idx {
                    self.unselect_todolist(idx);
                }
                self.command_prompt.select_command();
                self.mode = Mode::Command;
            },
            _ => (),
        }
    }

    /////////////////
    // Editing Mode
    /////////////////

    pub fn toggle_editing (&mut self) {
        let mode = self.mode;
        if let Some(todolist) = self.current_todolist() {
            match mode {
                Mode::Normal => {
                    todolist.start_editing(mode);
                    self.mode = Mode::Insert
                },
                Mode::Insert => {
                    todolist.stop_editing(mode);
                    self.mode = Mode::Normal;
                },
                _ => {},
            }
        }
    } 

    pub fn insert_backspace(&mut self) {
        let mode = self.mode;
        assert!(mode == Mode::Insert);
        let todolist = self.current_todolist().expect("A todolist must be selected if in Insert mode");
        todolist.insert_backspace(mode);
    }

    pub fn insert_char(&mut self, c: char) {
        let mode = self.mode;
        assert!(mode == Mode::Insert);
        let todolist = self.current_todolist().expect("A todolist must be selected if in Insert mode");
        todolist.insert_char(c, mode);
    }

    ////////////////
    // Visual Mode
    ////////////////
    
    pub fn toggle_visual (&mut self) {
        let mode = self.mode;
        match mode {
            Mode::Normal => {
                if let Some(todolist) = self.current_todolist() {
                    todolist.start_visual_selection(mode);
                    self.mode = Mode::Visual;
                }
            },
            Mode::Visual => {
                let todolist = self.current_todolist().expect("We can only enter visual mode if todolist exist");
                todolist.end_visual_selection(mode);
                self.mode = Mode::Normal;
            },
            Mode::Insert => {},
            Mode::Command => {},
        }
    }
}

use ratatui::prelude::*;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app_panes = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(4),
                Constraint::Fill(1),
                Constraint::Max(3),
            ])
            .split(area);

        // Header
        let title = "Todolist Manager";
        let mode_text = self.mode.to_string();
        let header = Paragraph::new(vec![
            Line::from(title), 
            Line::from(mode_text),
        ])
            .centered()
            .block(Block::bordered());
        header.render(app_panes[0], buf);

        // Todolists
        let len = self.todolists.len();
        let todolist_constraints: Vec<Constraint> = vec![Constraint::Max(40); len];
        let todolist_panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(todolist_constraints)
            .split(app_panes[1]);
        for (pane, todolist) in todolist_panes.iter().zip(&self.todolists) {
            todolist.render(*pane, buf);
        }

        // Command Prompt
        self.command_prompt.render(app_panes[2], buf);
    }
}
