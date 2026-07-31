use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Alignment,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    action::Action,
    components::{Component, weather::fetch::fetch_weather},
};

mod fetch;

pub enum WeatherIcon {
    Sunny,
    PartlyCloudy,
    Cloudy,
    LightRain,
    Rain,
    Thunder,
    Snow,
}

impl WeatherIcon {
    pub fn glyph(&self) -> &'static str {
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

impl WeatherIcon {
    pub fn from_code(code: u32) -> Self {
        match code {
            // Clear / Sunny
            1000 => WeatherIcon::Sunny,

            // Partly cloudy
            1003 => WeatherIcon::PartlyCloudy,

            // Cloudy / overcast / haze / dust / smoke / fog family
            1006 | 1009 | 1012 | 1015 | 1018 | 1021 | 1024 | 1027 | 1030 | 1033 | 1036 | 1039
            | 1042 | 1045 | 1048 | 1135 | 1147 => WeatherIcon::Cloudy,

            // Light rain / drizzle
            1063 | 1072 | 1150 | 1153 | 1168 | 1180 | 1183 | 1198 | 1240 => WeatherIcon::LightRain,

            // Heavier rain / sleet / ice pellets
            1171 | 1186 | 1189 | 1192 | 1195 | 1201 | 1204 | 1207 | 1237 | 1243 | 1246 | 1249
            | 1252 | 1261 | 1264 => WeatherIcon::Rain,

            // Thunder
            1087 | 1273 | 1276 | 1279 | 1282 => WeatherIcon::Thunder,

            // Snow
            1066 | 1069 | 1114 | 1117 | 1210 | 1213 | 1216 | 1219 | 1222 | 1225 | 1255 | 1258 => {
                WeatherIcon::Snow
            }

            // Fallback for anything unmapped
            _ => WeatherIcon::Cloudy,
        }
    }
}

pub struct Weather {
    pub icon: WeatherIcon,
    pub high: i32, // e.g. 20.2°C stored as 202
    pub low: i32,
}

impl Weather {
    fn fetch(&mut self) -> color_eyre::Result<()> {
        dotenvy::dotenv().ok();
        let api_key = std::env::var("WEATHERAPI_KEY")?;
        let data = fetch_weather(&api_key, "Birmingham")?;

        self.icon = data.icon;
        self.high = data.high;
        self.low = data.low;
        Ok(())
    }
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            icon: WeatherIcon::Sunny,
            high: 290,
            low: 172,
        }
    }
}

impl Component for Weather {
    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        if key.code == KeyCode::Char('r') {
            return Ok(Some(Action::WeatherRefresh));
        }
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        if let Action::WeatherRefresh = action {
            self.fetch()?;
        }
        Ok(None)
    }

    fn draw(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
    ) -> color_eyre::Result<()> {
        let block = Block::default().title("Weather").borders(Borders::ALL);

        let high = self.high as f32 / 10.0;
        let low = self.low as f32 / 10.0;

        let text = Text::from(vec![
            Line::from(self.icon.glyph()),
            Line::from(format!("High: {high:.1}°C")),
            Line::from(format!("Low: {low:.1}°C")),
        ]);

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
        Ok(())
    }
}
