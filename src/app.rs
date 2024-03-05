use serde::{Deserialize, Serialize};
use core::fmt;
use std::cmp;

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub num: usize,
    pub todos: Vec<Todo>,
}
#[derive(Serialize, Deserialize)]
pub struct App {
    pub currently_editing: Option<CurrentlyEditing>,
    pub mode: Mode,
    pub todos: TodoList,
    pub line_num: Option<usize>,
    pub visual_begin: Option<usize>,
}
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub selected: bool,
    pub value: String,
    pub completed: bool, 
    pub description: String,
    pub editing: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrentlyEditing {
    List,
    Description,
}
#[derive(Serialize, Deserialize)]
pub enum Mode {
    Insert, 
    Normal,
    Visual,
}
impl fmt::Display for Mode {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        match self {
            Mode::Insert => write!(f, "Insert Mode"),
            Mode::Normal => write!(f, "Normal Mode"),
            Mode::Visual => write!(f, "Visual Mode"),
        }
    }
}
impl Todo {
    fn new() -> Todo {
        Todo{
            selected: false,
            value: String::new(),
            completed: false,
            description: String::new(),
            editing: false,
        }
    }
}
impl TodoList{
    fn new() -> TodoList {
        TodoList{
            num: 0,
            todos: Vec::new(),
        }
    }
    pub fn add_todo(&mut self, todo: Todo, pos:usize) {
        if pos > self.todos.len() {
            panic!("Position is out of bounds");
        }
        self.num += 1;
        self.todos.push(todo);
        self.move_todo(self.todos.len()-1, pos);
    }
    pub fn delete(&mut self, pos: usize) {
        if pos < self.todos.len() {
            self.num -= 1;
            self.move_todo(pos, self.todos.len() - 1);
            self.todos.pop();
        }
    }
    // moves the element at index a to index b while keeping everything else in place
    pub fn move_todo(&mut self, a: usize, b: usize) {
        if a >= self.todos.len() || b >= self.todos.len(){
            panic!("tried to move two indexes which were out of bound");
        }
        if a == b {
            return;
        }
        if a < b {
            for i in a..b {
                self.todos.swap(i, i+1);
            }
        }
        else {
            for i in (b..a).rev() { 
                self.todos.swap(i, i+1);
            }
        }
    }
}
impl App {
    pub fn new() -> App{
        App{
            currently_editing: Some(CurrentlyEditing::List),
            mode: Mode::Normal,
            todos: TodoList::new(),
            line_num: None,
            visual_begin: None,
        }
    }
    pub fn add_todo(&mut self) {
        if let Some(line_num) = self.line_num{
            self.todos.add_todo(Todo::new(), line_num);
            self.refresh_normal_selection();
            self.toggle_editing();
        }
        else {
            self.todos.add_todo(Todo::new(), 0);
            self.line_num = Some(0);
            self.refresh_normal_selection();
            self.toggle_editing();
        }
    }
    pub fn move_up(&mut self) {
        if let Some(line_num) = self.line_num{
            if line_num > 0 {
                self.todos.move_todo(line_num, cmp::max(0, line_num.saturating_sub(1)));
                self.line_num = Some(line_num - 1);
                self.refresh_normal_selection();
            }
        }
    }
    pub fn move_down(&mut self) {
        if let Some(line_num) = self.line_num{
            if line_num < self.todos.num - 1 {
                self.todos.move_todo(line_num, cmp::min(self.todos.num.saturating_sub(1), line_num.saturating_add(1)));
                self.line_num = Some(line_num + 1);
                self.refresh_normal_selection();
            }
        }
    }
    pub fn visual_move_up(&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(visual_begin) = self.visual_begin{
                let a = cmp::min(line_num, visual_begin);
                let b = cmp::max(line_num, visual_begin);
                if a > 0 {
                    self.todos.move_todo(a-1, b);
                    self.line_num = Some(line_num - 1);
                    self.visual_begin = Some(visual_begin - 1);
                }
            }
        }
    }
    pub fn visual_move_down(&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(visual_begin) = self.visual_begin{
                let a = cmp::min(line_num, visual_begin);
                let b = cmp::max(line_num, visual_begin);
                if b < self.todos.num - 1 {
                    self.todos.move_todo(b+1, a);
                    self.line_num = Some(line_num + 1);
                    self.visual_begin = Some(visual_begin + 1);
                }
            }
        }
    }
    pub fn delete(&mut self) {
        let mut i = 0;
        while i < self.todos.todos.len() {
            if self.todos.todos[i].selected {
                self.todos.delete(i);
                continue;
            }
            i += 1;
        }
        self.todos.num = self.todos.todos.len();
        if self.todos.num == 0 {
            self.line_num = None;
        }
        if let Some(line_num) = self.line_num {
            self.line_num = Some(cmp::min(self.todos.num - 1, line_num));
        }
        self.mode = Mode::Normal;
        self.refresh_normal_selection();
    }
    pub fn toggle_editing (&mut self) {
        match self.mode {
            Mode::Normal => {
                self.mode = Mode::Insert;
                self.toggle_todo_editing();
            },
            Mode::Insert => {
                self.mode = Mode::Normal;
                self.toggle_todo_editing();
            },
            Mode::Visual => {},
        }
    } 
    pub fn toggle_visual (&mut self) {
        match self.mode {
            Mode::Visual => {
                self.mode = Mode::Normal;
                self.visual_begin = None;
            },
            Mode::Normal => {
                self.mode = Mode::Visual;
                if let Some(line_num) = self.line_num{
                    self.visual_begin = Some(line_num);
                }
            },
            Mode::Insert => {}
        }
    }
    pub fn toggle_todo_select (&mut self) {
        if let Some(line_num) = self.line_num {
            self.todos.todos[line_num].selected ^= true;
        }
    }
    pub fn refresh_normal_selection (&mut self) {
        if let Some(line_num) = self.line_num {
            for i in 0..self.todos.num {
                if i == line_num{
                    self.todos.todos[line_num].selected = true;
                }
                else {
                    self.todos.todos[i].selected = false;
                }
            }
        }
        else {
            for i in 0..self.todos.num{
                self.todos.todos[i].selected = false;
            }
        }
    }
    pub fn refresh_visual_selection (&mut self) {
        if let Some(visual_begin) = self.visual_begin {
            if let Some(line_num) = self.line_num {
                let a = cmp::min(visual_begin, line_num);
                let b = cmp:: max(visual_begin, line_num);
                for i in 0..self.todos.num {
                    if a <= i && i <= b {
                        self.todos.todos[i].selected = true;
                    }
                    else {
                        self.todos.todos[i].selected = false;
                    }
                }
            }
        }
    }
    pub fn toggle_todo_editing(&mut self) {
        if let Some(line_num) = self.line_num {
            self.todos.todos[line_num].editing = !self.todos.todos[line_num].editing;
        }
        else{
            // todo implement for multiple lists
        }
    }
    pub fn toggle_current_view(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::List) => {
                self.currently_editing = Some(CurrentlyEditing::Description);
            }
            Some(CurrentlyEditing::Description) => {
                self.currently_editing = Some(CurrentlyEditing::List);
            }
            None => {
                self.currently_editing = Some(CurrentlyEditing::List);
            }
        }
    }
    pub fn toggle_completetion (&mut self) {
        for todo in &mut self.todos.todos {
            if todo.selected {
                todo.completed = !todo.completed;
            }
        }
    }
}
