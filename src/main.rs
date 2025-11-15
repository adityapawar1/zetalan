use std::io;
use app::App;

mod zetamac;
mod app;
mod p2p;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
