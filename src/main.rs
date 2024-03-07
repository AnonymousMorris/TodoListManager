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
mod config;
use crate::{
    app::App,
    ui::ui,
};

fn main() -> Result<()> {
    let mut out = stdout();
    let _ = execute!(
        out,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        )
    );
    let _ = stdout().execute(EnterAlternateScreen);
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = match config::retrieve() {
        Err(_) => App::new(),
        Ok(app) => app,
    };
    app.toggle_todo_editing();
    loop{
        let _ = terminal.draw(|f| {ui(f, &app);});
        
        if event::poll(std::time::Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                match app.mode {
                    app::Mode::Normal => {
                        if key.kind == KeyEventKind::Press{
                            match key.code {
                                KeyCode::Char('q') => {
                                    config::save(&app);
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
                                KeyCode::Char('n') => {
                                    app.add_todolist();
                                },
                                KeyCode::Char(':') => {
                                    app.toggle_command();
                                },
                                KeyCode::Char('D') => {
                                    // add a warning to warn user about deleting a todolist
                                    app.delete_todolist();
                                },
                                KeyCode::Char('i') => {
                                    app.toggle_editing();
                                }
                                KeyCode::Char('v') => {
                                    app.toggle_visual();
                                }
                                KeyCode::Char('s') => {
                                    config::save(&app);
                                }
                                KeyCode::Char('j') => {
                                    app.move_down();
                                }
                                KeyCode::Char('k') => {
                                    app.move_up();
                                }
                                KeyCode::Char('h') => {
                                    app.move_left();
                                }
                                KeyCode::Char('l') => {
                                    app.move_right();
                                }
                                KeyCode::Char('J') => {
                                    app.move_todo_down();
                                },
                                KeyCode::Char('K') => {
                                    app.move_todo_up();
                                },
                                KeyCode::Char('H') => {
                                    app.move_todolist_left();
                                },
                                KeyCode::Char('L') => {
                                    app.move_todolist_right();
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
                                KeyCode::Char(':') => {
                                    app.toggle_command();
                                },
                                KeyCode::Char('d') => {
                                    app.delete();
                                }
                                KeyCode::Char('q') => {
                                    config::save(& app);
                                    break;
                                }
                                KeyCode::Char('j') => {
                                    app.visual_move_down();
                                }
                                KeyCode::Char('k') => {
                                    app.visual_move_up();
                                }
                                KeyCode::Char('J') => {
                                    app.visual_move_todo_down();
                                }
                                KeyCode::Char('K') => {
                                    app.visual_move_todo_up();
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
                                        if let Some(todolist) = app.current_todolist() {
                                            todolist.todos[todo_idx].value.pop();
                                        }
                                    }
                                    else {
                                        if let Some(todolist) = app.current_todolist(){
                                            todolist.title.pop();
                                        }
                                    }
                                },
                                KeyCode::Char(val) => {
                                    if let Some(todo_idx) = app.line_num{
                                        if let Some(todolist) = app.current_todolist() {
                                            todolist.todos[todo_idx].value.push(val);
                                        }
                                    }
                                    else {
                                        if let Some(todolist) = app.current_todolist() {
                                            todolist.title.push(val);
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                    app::Mode::Command => {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => {
                                    app.execute();
                                },
                                KeyCode::Backspace => {
                                    app.command.value.pop();
                                },
                                KeyCode::Char(val) => {
                                    app.command.value.push(val);
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = stdout().execute(PopKeyboardEnhancementFlags);
    disable_raw_mode()?;
    Ok(())
}
