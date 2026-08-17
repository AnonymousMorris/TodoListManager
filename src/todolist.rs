use serde::{Deserialize, Serialize};
use crate::todo::Todo;
use crate::app::Mode;
use std::cmp::{min, max};


#[derive(Serialize, Deserialize)]
struct VisualRange {
    pub start: usize,
    pub end: usize,
}

impl VisualRange {
    pub fn new(start: usize, end: usize) -> VisualRange {
        return Self {
            start, 
            end,
        }
    }

    pub fn lower(&self) -> usize {
        return min(self.start, self.end);
    }

    pub fn uppser(&self) -> usize {
        return max(self.start, self.end);
    }

    fn contains(&self, idx: usize) -> bool {
        let lower = min(self.start, self.end);
        let upper = max(self.start, self.end);
        return (lower..=upper).contains(&idx);
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub title: String,
    todos: Vec<Todo>,
    pub todo_idx: Option<usize>,
    visual_range: VisualRange,
    pub editing_title: bool,
    pub selected: bool,
}

impl TodoList{
    pub fn new() -> TodoList {
        TodoList{
            title: String::from("Todo List"),
            todos: Vec::new(),
            todo_idx: None,
            visual_range: VisualRange::new(0, 0),
            editing_title: false,
            selected: false,
        }
    }

    fn current_todo(&mut self) -> Option<&mut Todo> {
        if let Some(idx) = self.todo_idx {
            return Some(&mut self.todos[idx]);
        }
        return None;
    }
    
    pub fn create_todo_below(&mut self) -> usize {
        let len = self.todos.len();
        let mut pos = 0;
        if let Some(idx) = self.todo_idx {
            self.unselect_todo(idx);
            pos = idx + 1;
        }
        let todo = Todo::new(len);
        self.todos.push(todo);
        self.move_todo(len, pos);
        return pos;
    }

    fn swap_todo(&mut self, a: usize, b: usize) {
        assert!(a < self.todos.len());
        assert!(b < self.todos.len());
        assert!(a != b);
        self.todos.swap(a, b);
        self.todos[a].todo_idx = a;
        self.todos[b].todo_idx = b;
    }

    pub fn select(&mut self, idx: usize, mode: Mode) {
        assert!(mode == Mode::Normal || mode == Mode::Visual);
        self.selected = true;
        self.select_todo(idx);
    }
    
    pub fn unselect(&mut self, mode: Mode) {
        assert!(mode == Mode::Normal || mode == Mode::Visual);
        self.selected = false;
        if let Some(idx) = self.todo_idx {
            self.unselect_todo(idx);
        }
    }

    pub fn select_todo(&mut self, idx: usize) {
        let len = self.todos.len();
        if len == 0 {
            self.todo_idx = None;
            return;
        }
        let new_idx = min(idx, len - 1);
        self.todo_idx = Some(new_idx);
        self.todos[new_idx].selected = true;
    }

    pub fn unselect_todo(&mut self, idx: usize) {
        assert!(idx < self.todos.len());
        self.todos[idx].selected = false;
    }

    // moves the element at index a to index b while keeping everything else in place
    pub fn move_todo(&mut self, a: usize, b: usize) {
        assert!(a < self.todos.len());
        assert!(b < self.todos.len());
        if a == b {
            return;
        }
        if a < b {
            for i in a..b {
                self.swap_todo(i, i+1);
            }
        }
        else {
            for i in (b..a).rev() {
                self.swap_todo(i, i+1);
            }
        }
    }

    pub fn move_selection_up(&mut self, mode: Mode) {
        assert!(mode == Mode::Normal || mode == Mode::Visual);
        match mode {
            Mode::Normal => {
                if let Some(idx) = self.todo_idx {
                    self.unselect_todo(idx);
                    if idx > 0 {
                        self.select_todo(idx.saturating_sub(1));
                    }
                    else {
                        self.todo_idx = None;
                    }
                }
            }
            Mode::Visual => {
                let idx = self.todo_idx.expect("todo must be selected in visual mode");
                let new_idx = idx.saturating_sub(1);
                self.visual_range.end = new_idx;
                self.select_todo(new_idx);
                if !self.visual_range.contains(idx) {
                    self.unselect_todo(idx);
                }
            }
            _ => unreachable!("move_selection_up only supports Normal and Visual modes"),
        }
    }

    pub fn move_selection_down(&mut self, mode: Mode) {
        let len = self.todos.len();
        match mode {
            Mode::Normal => {
                if let Some(idx) = self.todo_idx {
                    self.unselect_todo(idx);
                    self.select_todo(min(idx + 1, len - 1));
                }
                else {
                    self.select_todo(0);
                }
            }
            Mode::Visual => {
                let idx = self.todo_idx.expect("a todo must be selected in visual mode");
                let new_idx = min(idx + 1, len - 1);
                self.visual_range.end = new_idx;
                self.select_todo(new_idx);
                if !self.visual_range.contains(idx) {
                    self.unselect_todo(idx);
                }
            }
            _ => unreachable!("move_selection_down only supports Normal and Visual modes")
        }
    }

    pub fn move_todo_up(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => {
                if let Some(idx) = self.todo_idx {
                    self.swap_todo(idx, idx.saturating_sub(1));
                    self.select_todo(idx.saturating_sub(1));
                }
            }
            Mode::Visual => {
                let lower = self.visual_range.lower();
                let upper = self.visual_range.uppser();
                if lower > 0 {
                    self.move_todo(lower - 1, upper);
                    self.visual_range.start -= 1;
                    self.visual_range.end -= 1;
                }
            }
            _ => unreachable!("move only supports Normal and Visual Mode")
        }
    }

    pub fn move_todo_down(&mut self, mode: Mode) {
        let len = self.todos.len();
        match mode {
            Mode::Normal => {
                if let Some(idx) = self.todo_idx {
                    if idx < (self.todos.len() - 1) {
                        self.swap_todo(idx, min(idx + 1, len - 1));
                        self.select_todo(min(idx + 1, len - 1));
                    }
                }
            }
            Mode::Visual => {
                let lower = self.visual_range.lower();
                let upper = self.visual_range.uppser();
                if upper < len - 1 {
                    self.move_todo(upper + 1, lower);
                    self.visual_range.start += 1;
                    self.visual_range.end += 1;
                }
            }
            _ => unreachable!("move only supports Normal and Visual Mode")
        }
    }

    pub fn delete_todo(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => {
                if let Some(idx) = self.todo_idx {
                    self.todos.remove(idx);
                    self.select_todo(idx);
                }
            }
            Mode::Visual => {
                let lower = self.visual_range.lower();
                let upper = self.visual_range.uppser();
                assert!(upper < self.todos.len());
                for i in (lower..=upper).rev() {
                    self.todos.remove(i);
                }
                self.select_todo(lower);
            }
            _ => unreachable!("delete only support Normal and Visual Mode")
        }
    }

    pub fn delete_completed_todos(&mut self, mode: Mode) {
        assert!(mode == Mode::Command);
        let len = self.todos.len();
        for i in (0..len).rev() {
            if self.todos[i].completed {
                self.todos.remove(i);
            }
        }
    }

    pub fn start_visual_selection(&mut self, mode: Mode) {
        assert!(mode == Mode::Normal);
        if let Some(idx) = self.todo_idx {
            self.visual_range.start = idx;
            self.visual_range.end = idx;
        }
    }
    
    pub fn end_visual_selection(&mut self, mode: Mode) {
        assert!(mode == Mode::Visual);
        let idx = self.visual_range.end;
        let lower = self.visual_range.lower();
        let upper = self.visual_range.uppser();
        self.todo_idx = Some(idx);
        for i in lower..=upper {
            self.unselect_todo(i);
        }
        self.select_todo(idx);
    }

    pub fn start_editing(&mut self, mode: Mode) {
        assert!(mode == Mode::Normal);
        if let Some(todo) = self.current_todo() {
            todo.editing = true;
        }
        else {
            self.editing_title = true;
        }
    }

    pub fn stop_editing(&mut self, mode: Mode) {
        assert!(mode == Mode::Insert);
        if self.editing_title {
            self.editing_title = false;
        }
        else {
            let todo = self.current_todo().expect("a todo must be select in insert mode if the todolist title isn't");
            todo.editing = false;
        }
    }

    pub fn toggle_completed(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => {
                if let Some(todo) = self.current_todo() {
                    todo.completed ^= true;
                }
            }
            Mode::Visual => {
                let lower = self.visual_range.lower();
                let upper = self.visual_range.uppser();
                for i in lower..=upper {
                    self.todos[i].completed ^= true;
                }
            }
            _ => unreachable!("toggling completed only support normal and visual mode")
        }
    }

    pub fn insert_backspace(&mut self, mode: Mode) {
        assert!(mode == Mode::Insert);
        if self.editing_title {
            self.title.pop();
        }
        else {
            let todo = self.current_todo().expect("Editing title must be set to true if insert mode and no todo is selected");
            todo.value.pop();
        }
    }

    pub fn insert_char(&mut self, c: char, mode: Mode) {
        assert!(mode == Mode::Insert);
        if self.editing_title {
            self.title.push(c);
        }
        else {
            let todo = self.current_todo().expect("Editing title must be set to true if insert mode and no todo is selected");
            todo.value.push(c);
        }
    }
}

use ratatui::prelude::*;
use ratatui::widgets::{List, ListItem};
use ratatui::widgets::{Block};
impl Widget for &TodoList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Todolist Title
        let title = Span::from(self.title.as_str());
        let mut cursor = Span::raw("");
        if self.selected && self.editing_title {
            cursor = Span::from(" ").bg(Color::White);
        }
        let title_style = if self.todo_idx.is_none() {
            Color::Yellow
        } else{
            Color::White
        };
        let block_title = Line::from_iter([title, cursor])
            .style(title_style)
            .centered();

        // Todo items
        let todo_items = self.todos.iter().map(|todo| {
            let status = if todo.completed { " [x] " } else { " [ ] " };

            let mut content_style = Style::default();
            let mut span_style = Style::default();

            if todo.completed {
                content_style = content_style.add_modifier(Modifier::CROSSED_OUT);
            }

            if todo.editing {
                content_style = content_style.add_modifier(Modifier::UNDERLINED);
            }

            if todo.selected && !todo.editing {
                // Ghostty does not support blinking as of Aug 14, 2026. 
                span_style = span_style.add_modifier(Modifier::SLOW_BLINK);
                span_style = span_style.yellow();
            }

            // We append a 0 width invisible character so the
            // textwrap library does not trim our spaces
            const WORD_JOINER: char = '\u{2060}';
            let display_value = if todo.editing {
                format!("{}{WORD_JOINER}", todo.value)
            } else {
                todo.value.clone()
            };

            // Creating text wrapping through the textwrap library
            let border_width = 2;
            let status_width = 5;
            let cursor_width = 1;
            let width = area.width.saturating_sub(border_width + status_width + cursor_width) as usize;
            let wrapped = textwrap::wrap(&display_value, width.max(1));
            let last = wrapped.len().saturating_sub(1);
            let lines = wrapped.into_iter()
                .enumerate()
                .map(|(i, text)| {
                    let prefix = if i == 0 { status } else { "     " };
                    let cursor = if todo.editing && i == last {
                        Span::from(" ").bg(Color::White)
                    } 
                    else {
                        Span::raw("")
                    };

                    Line::from(vec![
                        Span::raw(prefix),
                        Span::styled(text.into_owned(), content_style),
                        cursor,
                    ])
                    
                })
            .collect::<Vec<_>>();
            ListItem::new(lines).style(span_style)
        });

        // Todolist UI Component
        let todolist_color = if self.selected {Color::Yellow} else {Color::White};
        let todolist_block = Block::bordered().title(block_title).border_style(todolist_color);
        let todo_list = List::new(todo_items).block(todolist_block);

        Widget::render(todo_list, area, buf);
    }
}

