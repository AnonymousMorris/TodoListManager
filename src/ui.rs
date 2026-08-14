use crate::app::App;
use ratatui::prelude::*;
pub fn ui (f: &mut Frame, app: & App) {
    f.render_widget(app, f.area());
}
