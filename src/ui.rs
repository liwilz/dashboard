use ratatui::{
    Frame,
    layout::Alignment,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::model::Model;

pub fn view(model: &Model, f: &mut Frame) {
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
