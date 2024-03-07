use crate::app::App;
use crate::app::Mode;
use ratatui::prelude::*;
use ratatui::widgets::BorderType;
use ratatui::widgets::{Paragraph, Block, Wrap};
pub fn ui (f: &mut Frame, app: & App) {
    let head = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(5),
            Constraint::Fill(1),
            Constraint::Max(3),
        ])
        .split(f.size());
    let constraints: Vec<Constraint> = app.todolists.iter().map( |_| Constraint::Max(40)).collect();
    let pane = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(head[1]);
    let title = render_title(app);
    let command = render_command(app);
    f.render_widget(title, head[0]);
    f.render_widget(command, head[2]);

    let mut list: Vec<Paragraph> = vec![];
    for i in 0..app.todolists.len() {
        list.push(render_list(app, i));
    }
    for i in 0 ..app.todolists.len() {
        f.render_widget(&list[i], pane[i]);
    }
}
fn render_command(app: &App) -> Paragraph {
    let command_block = Block::bordered().border_type(BorderType::Plain);
    let cursor = if app.mode == Mode::Command {Span::from(" ").bg(Color::White)} else { Span::raw("") };
    let command_line = Line::from(vec![Span::from(& app.command.value), cursor]);
    return Paragraph::new(command_line).left_aligned().block(command_block);
}
fn render_title(app: &App) -> Paragraph {
    let title_block = Block::bordered();
    let title_style = Style::new();
    let title_text = vec![Line::styled("ToDo List Manager", title_style), Line::from(app.mode.to_string())];
    let title = Paragraph::new(title_text).centered().block(title_block);
    title
}
fn render_list(app: &App, todolist_idx: usize) -> Paragraph {
    let mut text = Vec::new();
    let mut cursor = Span::raw("");
    if let Some(idx) = app.current_todolist{
        if idx == todolist_idx && app.mode == Mode::Insert && app.line_num == None{
            cursor = Span::from(" ").bg(Color::White); 
        }
    }
    let block_line = Line::from(vec![Span::raw(& app.todolists[todolist_idx].title), cursor]);
    let mut block = Block::bordered().title_top(block_line).title_alignment(Alignment::Center);
    if let Some(idx) = app.current_todolist{
        if idx == todolist_idx {
            block = block.border_style(Style::new().yellow());
        }
    }
    let todolist = &app.todolists[todolist_idx];
    for todo in &todolist.todos {
        let status_string = if todo.completed {" [x] "} else {" [ ] "};
        let mut todo_style = Style::new();
        if todo.completed {
            todo_style = todo_style.add_modifier(Modifier::CROSSED_OUT);
        }
        if todo.editing {
            todo_style = todo_style.add_modifier(Modifier::UNDERLINED);
        }
        let todo_span = Span::raw(& todo.value).style(todo_style);
        let cursor = if todo.editing {Span::from(" ").bg(Color::White)} else {Span::raw("")};
        let todo_line = Line::from(vec![Span::raw(status_string), todo_span, cursor]);
        if todo.selected && !todo.editing{
            text.push(todo_line.style(Style::new().add_modifier(Modifier::SLOW_BLINK)));
        }
        else{
            text.push(todo_line);
        }
    }
    let list = Paragraph::new(text).left_aligned().wrap(Wrap{trim: false}).block(block);
    
    list
}
