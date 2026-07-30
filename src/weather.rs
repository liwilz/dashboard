use color_eyre::Result;
use serde::Deserialize;

use crate::model::WeatherIcon;

#[derive(Deserialize)]
struct ForecastResponse {
    forecast: Forecast,
}

#[derive(Deserialize)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize)]
struct ForecastDay {
    day: Day,
}

#[derive(Deserialize)]
struct Day {
    maxtemp_c: f64,
    mintemp_c: f64,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    code: u32,
}

pub struct WeatherData {
    pub icon: WeatherIcon,
    pub high: i32,
    pub low: i32,
}

fn fetch_forecast(api_key: &str, location: &str) -> Result<ForecastResponse> {
    let url =
        format!("https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={location}&days=1");
    let response = reqwest::blocking::get(url)?.json::<ForecastResponse>()?;
    Ok(response)
}

pub fn fetch_weather(api_key: &str, location: &str) -> Result<WeatherData> {
    let response = fetch_forecast(api_key, location)?;

    let day = &response
        .forecast
        .forecastday
        .first()
        .ok_or_else(|| color_eyre::eyre::eyre!("no forecast day returned"))?
        .day;

    Ok(WeatherData {
        icon: WeatherIcon::from_code(day.condition.code),
        high: (day.maxtemp_c * 10.0).round() as i32,
        low: (day.mintemp_c * 10.0).round() as i32,
    })
}
