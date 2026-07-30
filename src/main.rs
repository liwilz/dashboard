use std::io::stdout;

use color_eyre::Result;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

use crate::{
    event::handle_event,
    model::{Model, RunningState},
    ui::view,
    update::update,
};

mod event;
mod message;
mod model;
mod ui;
mod update;

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        terminal.draw(|f| view(&model, f))?;
        if let Some(msg) = handle_event()? {
            update(&mut model, msg);
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
