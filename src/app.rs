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

pub struct Model {
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
