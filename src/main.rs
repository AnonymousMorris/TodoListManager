use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    }, ExecutableCommand
};

use todolist_manager::{
    app::{App, Mode}, 
    ui, config
};

use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result};

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
    loop{
        let _ = terminal.draw(|f| {ui::ui(f, &app);});
        
        if event::poll(std::time::Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Normal => {
                        if key.kind == KeyEventKind::Press{
                            match key.code {
                                KeyCode::Char('q') => {
                                    config::save(&app);
                                    break;
                                }, 
                                KeyCode::Char('x') => {
                                    app.toggle_completed();
                                },
                                KeyCode::Char('d') => {
                                    app.delete_todo();
                                },
                                KeyCode::Char('o') => {
                                    app.create_todo();
                                },
                                KeyCode::Char('n') => {
                                    app.create_todolist();
                                },
                                KeyCode::Char(':') => {
                                    app.toggle_command();
                                },
                                KeyCode::Char('D') => {
                                    // TODO: add a warning to warn user about deleting a todolist
                                    app.delete_todolist();
                                },
                                KeyCode::Char('i') => {
                                    app.toggle_editing();
                                }
                                KeyCode::Char('v') => {
                                    app.toggle_visual();
                                }
                                KeyCode::Char('V') => {
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
                    Mode::Visual => {
                        if key.kind == KeyEventKind::Press{
                            match key.code {
                                KeyCode::Char('q') => {
                                    config::save(&app);
                                    break;
                                }, 
                                KeyCode::Char('x') => {
                                    app.toggle_completed();
                                },
                                KeyCode::Char('d') => {
                                    app.delete_todo();
                                },
                                KeyCode::Char(':') => {
                                    app.toggle_command();
                                },
                                KeyCode::Char('v') => {
                                    app.toggle_visual();
                                }
                                KeyCode::Char('V') => {
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
                                KeyCode::Char('J') => {
                                    app.move_todo_down();
                                },
                                KeyCode::Char('K') => {
                                    app.move_todo_up();
                                },
                                _ => {},
                            }
                        }
                    },
                    Mode::Insert => {
                        if key.kind == KeyEventKind::Press{
                            if key.modifiers == event::KeyModifiers::CONTROL && key.code==KeyCode::Char('['){
                                app.toggle_editing();
                            }
                            else {
                                match key.code {
                                    KeyCode::Esc => {
                                        app.toggle_editing();
                                    },
                                    KeyCode::Enter => {
                                        app.toggle_editing();
                                    },
                                    KeyCode::Backspace => {
                                        app.insert_backspace();
                                    },
                                    KeyCode::Char(val) => {
                                        app.insert_char(val);
                                    },
                                    _ => {}
                                }
                            }
                        }
                    },
                    Mode::Command => {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => {
                                    let should_exit: bool = app.execute();
                                    if should_exit {
                                        break;
                                    }
                                },
                                KeyCode::Backspace => {
                                    app.command_backspace();
                                },
                                KeyCode::Char(val) => {
                                    app.command_char(val);
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
