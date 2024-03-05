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
    let pane = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(head[1]);
    let title = render_title(app);
    // let hello_world = render_hello_world(app);
    let list = render_list(app);
    f.render_widget(title, head[0]);
    //f.render_widget( hello_world, pane[0]);
    f.render_widget(list, pane[0])
}
fn render_title(app: &App) -> Paragraph {
    let title_block = Block::bordered();
    let title_style = Style::new();
    let title_text = vec![Line::styled("ToDo List Manager", title_style), Line::from(app.mode.to_string())];
    let title = Paragraph::new(title_text).centered().block(title_block);
    title
}
fn render_list(app: &App) -> Paragraph {
    let mut text = Vec::new();
    let block = Block::bordered().title_top(Line::raw("My Todos").centered());
    for todo in &app.todos.todos {
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
