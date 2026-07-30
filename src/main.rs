use std::io::stdout;

use color_eyre::Result;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand,
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    layout::Alignment,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

enum WeatherIcon {
    Sunny,
    PartlyCloudy,
    Cloudy,
    LightRain,
    Rain,
    Thunder,
    Snow,
}

impl WeatherIcon {
    fn glyph(&self) -> &'static str {
        match self {
            WeatherIcon::Sunny => "☀",
            WeatherIcon::PartlyCloudy => "⛅",
            WeatherIcon::Cloudy => "☁",
            WeatherIcon::LightRain => "🌦",
            WeatherIcon::Rain => "🌧",
            WeatherIcon::Thunder => "⛈",
            WeatherIcon::Snow => "❄",
        }
    }
}

#[derive(Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Running,
    Done,
}

struct Model {
    icon: WeatherIcon,
    high: i32, // e.g. 20.2°C stored as 202
    low: i32,
    running_state: RunningState,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            icon: WeatherIcon::Sunny,
            high: 290,
            low: 172,
            running_state: RunningState::default(),
        }
    }
}

enum Message {
    Quit,
}

fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Quit => model.running_state = RunningState::Done,
    }
}

fn view(model: &Model, f: &mut Frame) {
    let block = Block::default().title("Weather").borders(Borders::ALL);

    let high = model.high as f32 / 10.0;
    let low = model.low as f32 / 10.0;

    let text = Text::from(vec![
        Line::from(model.icon.glyph()),
        Line::from(format!("High: {high:.1}°C")),
        Line::from(format!("Low: {low:.1}°C")),
    ]);

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, f.area());
}

fn handle_event() -> Result<Option<Message>> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(None);
        }
        return Ok(match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        });
    }
    Ok(None)
}

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
