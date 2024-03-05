use app::Todo;
use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    }, ExecutableCommand
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};
mod app;
mod ui;
use crate::{
    app::App,
    ui::ui
};

fn main() -> Result<()> {
    let mut out = stdout();
    let _ = execute!(
        out,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        )
    );
    out.execute(EnterAlternateScreen);
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();
    let todo1 = Todo {
        selected: false,
        value: String::from("1. hello morris"),
        completed: true,
        description: String::from("hi"),
        editing: false,
    };
    let todo2 = Todo {
        selected: false,
        value: String::from("2. finish this app?"),
        completed: false,
        description: String::from("hi"),
        editing: false,
    };
    let todo3 = Todo {
        selected: false,
        value: String::from("3. finish this app?"),
        completed: true,
        description: String::from("hi"),
        editing: false,
    };
    app.todos.add_todo(todo1, 0);
    app.todos.add_todo(todo2, 1);
    app.todos.add_todo(todo3, 2);
    app.toggle_completetion();
    app.todos.move_todo(2, 0);
    app.toggle_todo_editing();
    loop{
        let _ = terminal.draw(|f| {ui(f, &app);});
        
        if event::poll(std::time::Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                match app.mode {
                    app::Mode::Normal => {
                        if key.kind == KeyEventKind::Press{
                            if key.modifiers == event::KeyModifiers::SHIFT {
                                match key.code {
                                    KeyCode::Char('J') => {
                                        app.move_down();
                                    },
                                    KeyCode::Char('K') => {
                                        app.move_up();
                                    },
                                    _ => {},
                                }
                            }
                            match key.code {
                                KeyCode::Char('q') => {
                                    break;
                                }, 
                                KeyCode::Char('x') => {
                                    app.toggle_completetion();
                                },
                                KeyCode::Char('d') => {
                                    app.delete();
                                },
                                KeyCode::Char('a') => {
                                    app.add_todo();
                                },
                                KeyCode::Char('i') => {
                                    app.toggle_editing();
                                }
                                KeyCode::Char('v') => {
                                    app.toggle_visual();
                                }
                                KeyCode::Char('j') => {
                                    if let Some(line_num) = app.line_num{
                                        if line_num < app.todos.num - 1 {
                                            app.line_num = Some(line_num + 1);
                                        }
                                    }
                                    else {
                                        if app.todos.num > 0 {
                                            app.line_num = Some(0);
                                        }
                                    }
                                    app.refresh_normal_selection();
                                }
                                KeyCode::Char('k') => {
                                    if let Some(line_num) = app.line_num{
                                        if line_num > 0 {
                                            app.line_num = Some(line_num - 1);
                                        }
                                        else if line_num == 0 {
                                            app.line_num = None;
                                        }
                                    }
                                    app.refresh_normal_selection();
                                }
                                KeyCode::Char('J') => {
                                    app.move_down();
                                },
                                KeyCode::Char('K') => {
                                    app.move_up();
                                },
                                _ => {},
                            }
                        }
                    },
                    app::Mode::Visual => {
                        if key.kind == KeyEventKind::Press{
                            match key.code {
                                KeyCode::Char('x') => {
                                    app.toggle_completetion();
                                },
                                KeyCode::Char('v') => {
                                    app.toggle_visual();
                                    app.refresh_normal_selection();
                                }
                                KeyCode::Char('d') => {
                                    app.delete();
                                }
                                KeyCode::Char('q') => {
                                    break;
                                }
                                KeyCode::Char('j') => {
                                    if let Some(line_num) = app.line_num{
                                        if line_num < app.todos.num - 1 {
                                            app.line_num = Some(line_num + 1);
                                            app.refresh_visual_selection();
                                        }
                                    }
                                    else {
                                        if app.todos.num > 0 {
                                            app.line_num = Some(0);
                                            app.refresh_visual_selection();
                                        }
                                        if app.visual_begin.is_none() {
                                            app.visual_begin = app.line_num;
                                        }
                                    }
                                }
                                KeyCode::Char('k') => {
                                    if let Some(line_num) = app.line_num{
                                        if line_num > 0 {
                                            app.line_num = Some(line_num - 1);
                                            app.refresh_visual_selection();
                                        }
                                    }
                                }
                                KeyCode::Char('J') => {
                                    app.visual_move_down();
                                }
                                KeyCode::Char('K') => {
                                    app.visual_move_up();
                                }

                                _ => {},
                            }
                        }
                    },
                    app::Mode::Insert => {
                        if key.kind == KeyEventKind::Press{
                            if key.modifiers == event::KeyModifiers::CONTROL && key.code==KeyCode::Char('['){
                                app.toggle_editing();
                            }
                            match key.code {
                                KeyCode::Esc => {
                                    app.toggle_editing();
                                },
                                KeyCode::Enter => {
                                    app.toggle_editing();
                                },
                                KeyCode::Backspace => {
                                    if let Some(todo_idx) = app.line_num{
                                        app.todos.todos[todo_idx].value.pop();
                                    }
                                },
                                KeyCode::Char(val) => {
                                    if let Some(todo_idx) = app.line_num{
                                        app.todos.todos[todo_idx].value.push(val);
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                }
            }
        }
    }
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = stdout().execute(PopKeyboardEnhancementFlags);
    disable_raw_mode()?;
    Ok(())
}
