use serde::{Deserialize, Serialize};
use core::fmt;
use std::cmp;

use crate::config;

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub title: String,
}
#[derive(Serialize, Deserialize)]
pub struct App {
    pub current_todolist: Option<usize>,
    pub mode: Mode,
    pub todolists: Vec<TodoList>,
    pub line_num: Option<usize>,
    pub visual_begin: Option<usize>,
    pub command: Command,
}
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub selected: bool,
    pub value: String,
    pub completed: bool, 
    pub description: String,
    pub editing: bool,
}
#[derive(Serialize, Deserialize)]
pub struct Command {
    pub value: String,
}
#[derive(Serialize, Deserialize, PartialEq)]
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
impl Command {
    fn new() -> Command {
        return Command {value: String::new()};
    }
}
impl TodoList{
    fn new() -> TodoList {
        TodoList{
            todos: Vec::new(),
            title: String::from("Todo List"),
        }
    }
    pub fn add_todo(&mut self, todo: Todo, pos:usize) {
        if pos > self.todos.len() {
            panic!("Position is out of bounds");
        }
        self.todos.push(todo);
        self.move_todo(self.todos.len()-1, pos);
    }
    pub fn delete(&mut self, pos: usize) {
        if pos < self.todos.len() {
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
            mode: Mode::Normal,
            line_num: None,
            visual_begin: None,
            current_todolist: Some(0),
            todolists: vec![TodoList::new()],
            command: Command::new(),
        }
    }
    pub fn current_todolist(&mut self) -> Option<&mut TodoList> {
        if let Some(current_todolist) = self.current_todolist {
            return Some(&mut self.todolists[current_todolist]);
        }
        None
    }
    pub fn add_todo(&mut self) {
        if let Some(line_num) = self.line_num{
            if let Some(todolist) = self.current_todolist(){
                todolist.add_todo(Todo::new(), line_num + 1);
                self.move_down();
                self.refresh_normal_selection();
                self.toggle_editing();
            }
        }
        else {
            if let Some(todolist) = self.current_todolist(){
                todolist.add_todo(Todo::new(), 0);
                self.line_num = Some(0);
                self.refresh_normal_selection();
                self.toggle_editing();
            }
        }
    }
    pub fn add_todolist(&mut self) {
        self.todolists.push(TodoList::new());
        if let Some(todolist_idx) = self.current_todolist{
            self.move_todolist(self.todolists.len() - 1, todolist_idx + 1);
            self.current_todolist = Some(todolist_idx + 1);
        }
        else {
            self.current_todolist = Some(0);
        }
    }
    pub fn move_todolist(&mut self, a: usize, b: usize) {
        if a >= self.todolists.len() || b >= self.todolists.len() {
            panic!("tried to swap a todolist to out of bound");
        }
        if a == b {
            return;
        }
        if a < b {
            for i in a..b{
                self.todolists.swap(i, i+1);
            }
        }
        else {
            for i in (b..a).rev() {
                self.todolists.swap(i, i+1);
            }
        }
    }
    pub fn refresh_line_num (&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(todolist) = self.current_todolist(){
                if todolist.todos.len() == 0 {
                    self.line_num = None;
                }
                else {
                    self.line_num = Some( cmp::min(line_num, todolist.todos.len() - 1) );
                }
            }
        }
    }
    pub fn move_left (&mut self) {
        if let Some(todolist_idx) = self.current_todolist {
            if todolist_idx > 0 {
                self.toggle_selection();
                self.current_todolist = Some(todolist_idx - 1);
                self.refresh_line_num();
                self.toggle_selection();
            }
        }
    }
    pub fn move_right (&mut self) {
        if let Some(todolist_idx) = self.current_todolist {
            if todolist_idx < self.todolists.len() - 1 {
                self.toggle_selection();
                self.current_todolist = Some(todolist_idx + 1);
                self.refresh_line_num();
                self.toggle_selection();
            }
        }
        else {
            if self.todolists.len() > 0 {
                self.current_todolist = Some(0);
                self.refresh_line_num();
                self.toggle_selection();
            }
        }
    }
    pub fn move_todolist_left(&mut self) {
        if let Some(todolist_idx) = self.current_todolist {
            if todolist_idx > 0 {
                print!("debug {}, {}", todolist_idx, todolist_idx - 1);
                self.move_todolist(todolist_idx, todolist_idx - 1);
                self.move_left();
            }
        }
    }
    pub fn move_todolist_right (&mut self) {
        if let Some(todolist_idx) = self.current_todolist {
            if todolist_idx < self.todolists.len() - 1 {
                print!("debug {}, {}", todolist_idx, todolist_idx + 1);
                self.move_todolist(todolist_idx, todolist_idx + 1);
                self.move_right();
            }
        }
    }
    pub fn move_up(&mut self) {
        if let Some(line_num) = self.line_num{
            if line_num > 0 {
                self.line_num = Some(line_num - 1);
            }
            else if line_num == 0 {
                self.line_num = None;
            }
        }
        self.refresh_normal_selection();
    }
    pub fn move_down(&mut self) {
        if let Some(line_num) = self.line_num{
            if let Some(todolist) = self.current_todolist() {
                if line_num < todolist.todos.len() - 1 {
                    self.line_num = Some(line_num + 1);
                }
            }
        }
        else {
            if let Some(todolist) = self.current_todolist() {
                if todolist.todos.len() > 0 {
                    self.line_num = Some(0);
                }
            }
        }
        self.refresh_normal_selection();
    }
    pub fn move_todo_up(&mut self) {
        if let Some(line_num) = self.line_num{
            if let Some(todolist) = self.current_todolist() {
                if line_num > 0 {
                    todolist.move_todo(line_num, cmp::max(0, line_num.saturating_sub(1)));
                    self.line_num = Some(line_num - 1);
                    self.refresh_normal_selection();
                }
            }
        }
    }
    pub fn move_todo_down(&mut self) {
        if let Some(line_num) = self.line_num{
            if let Some(todolist) = self.current_todolist() {
                if line_num < todolist.todos.len()- 1 {
                    todolist.move_todo(line_num, cmp::min(todolist.todos.len().saturating_sub(1), line_num.saturating_add(1)));
                    self.line_num = Some(line_num + 1);
                    self.refresh_normal_selection();
                }
            }
        }
    }
    pub fn visual_move_up(&mut self) {
        if let Some(line_num) = self.line_num{
            if line_num > 0 {
                self.line_num = Some(line_num - 1);
                self.refresh_visual_selection();
            }
        }
    }
    pub fn visual_move_down(&mut self) {
        if let Some(line_num) = self.line_num{
            if let Some(todolist) = self.current_todolist() {
                if line_num < todolist.todos.len() - 1 {
                    self.line_num = Some(line_num + 1);
                    self.refresh_visual_selection();
                }
            }
        }
        else {
            if let Some(todolist) = self.current_todolist() {
                if todolist.todos.len() > 0 {
                    self.line_num = Some(0);
                    self.refresh_visual_selection();
                }
                if self.visual_begin.is_none() {
                    self.visual_begin = self.line_num;
                }
            }
        }
        
    }
    pub fn visual_move_todo_up(&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(visual_begin) = self.visual_begin{
                if let Some(todolist) = self.current_todolist() {
                    let a = cmp::min(line_num, visual_begin);
                    let b = cmp::max(line_num, visual_begin);
                    if a > 0 {
                        todolist.move_todo(a-1, b);
                        self.line_num = Some(line_num - 1);
                        self.visual_begin = Some(visual_begin - 1);
                    }
                }
            }
        }
    }
    pub fn visual_move_todo_down(&mut self) {
        if let Some(visual_begin) = self.visual_begin{
            if let Some(line_num) = self.line_num {
                if let Some(todolist) = self.current_todolist() {
                    let a = cmp::min(line_num, visual_begin);
                    let b = cmp::max(line_num, visual_begin);
                    if b < todolist.todos.len() - 1 {
                        todolist.move_todo(b+1, a);
                        self.line_num = Some(line_num + 1);
                        self.visual_begin = Some(visual_begin + 1);
                    }
                }
            }
        }
    }
    pub fn delete_todolist(&mut self) {
        if let Some(todolist_idx) = self.current_todolist{
            self.move_todolist(todolist_idx, self.todolists.len()-1);
            self.todolists.pop();
            if self.todolists.len() == 0 {
                self.current_todolist = None;
            }
            else {
                self.current_todolist = Some(cmp::min(self.todolists.len() -1, todolist_idx));
            }
        }
    }
    pub fn delete(&mut self) {
        let mut line_num = self.line_num;
        if let Some(todolist) = self.current_todolist() {
            let mut i = 0;
            while i < todolist.todos.len() {
                if todolist.todos[i].selected {
                    todolist.delete(i);
                    continue;
                }
                i += 1;
            }
            if todolist.todos.len() == 0 {
                line_num = None;
            }
            if let Some(num) = line_num {
                line_num = Some(cmp::min(todolist.todos.len() - 1, num));
            }
        }
        self.line_num = line_num;
        self.mode = Mode::Normal;
        self.refresh_normal_selection();
    }
    pub fn execute(&mut self) {
        match &self.command.value as &str {
            ":w" => {
                config::save(self);
                self.command.value = String::new();
            },
            ":clean" => {
                self.clean();
                self.command.value = String::new();
            },
            _ => (),
        }
        self.mode = Mode::Normal;
    }
    pub fn clean(&mut self) {
        for todolist in &mut self.todolists {
            let mut i = 0;
            while i < todolist.todos.len() {
                if todolist.todos[i].completed {
                    todolist.delete(i);
                    continue;
                }
                i += 1;
            }
        }
        self.refresh_line_num();
        self.mode = Mode::Normal;
        self.command.value = String::new();
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
            _ => {},
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
            Mode::Insert => {},
            Mode::Command => {},
        }
    }
    pub fn toggle_command (&mut self) {
        match self.mode {
            Mode::Normal => {
                self.mode = Mode::Command;
                self.command.value = String::from(":");
            },
            Mode::Visual => {
                self.mode = Mode::Command;
                self.command.value = String::from(":");
            }
            _ => (),
        }
    }
    pub fn refresh_normal_selection (&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(todolist) = self.current_todolist() {
                for i in 0..todolist.todos.len() {
                    if i == line_num{
                        todolist.todos[line_num].selected = true;
                    }
                    else {
                        todolist.todos[i].selected = false;
                    }
                }
            }
        }
        else {
            if let Some(todolist) = self.current_todolist() {
                for i in 0..todolist.todos.len() {
                    todolist.todos[i].selected = false;
                }
            }
        }
    }
    pub fn refresh_visual_selection (&mut self) {
        if let Some(visual_begin) = self.visual_begin {
            if let Some(line_num) = self.line_num {
                if let Some(todolist) = self.current_todolist() {
                    let a = cmp::min(visual_begin, line_num);
                    let b = cmp:: max(visual_begin, line_num);
                    for i in 0..todolist.todos.len() {
                        if a <= i && i <= b {
                            todolist.todos[i].selected = true;
                        }
                        else {
                            todolist.todos[i].selected = false;
                        }
                    }
                }
            }
        }
    }
    pub fn toggle_todo_editing(&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(todolist) = self.current_todolist() {
                todolist.todos[line_num].editing ^= true;
            }
        }
        else{
            // todo implement for multiple lists
        }
    }
    pub fn toggle_completetion (&mut self) {
        if let Some(todolist) = self.current_todolist() {
            for todo in &mut todolist.todos {
                if todo.selected {
                    todo.completed = !todo.completed;
                }
            }
        }
    }
    pub fn toggle_selection (&mut self) {
        if let Some(line_num) = self.line_num {
            if let Some(todolist) = self.current_todolist() {
                todolist.todos[line_num].selected ^= true;
            }
        }
    }
}
