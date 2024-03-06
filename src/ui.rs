use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Wrap};
pub fn ui (f: &mut Frame, app: & App) {
    let head = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(5),
            Constraint::Fill(1),
        ])
        .split(f.size());
    let mut constraints: Vec<Constraint> = vec![];
    constraints = app.todolists.iter().map( |_| Constraint::Max(40)).collect();
    let pane = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(head[1]);
    let title = render_title(app);
    f.render_widget(title, head[0]);

    let mut list: Vec<Paragraph> = vec![];
    for i in 0..app.todolists.len() {
        list.push(render_list(app, i));
    }
    for i in 0 ..app.todolists.len() {
        f.render_widget(&list[i], pane[i]);
    }
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
    let block = Block::bordered().title_top(Line::raw("My Todos").centered());
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
