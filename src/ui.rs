// --- Message ---

enum Message {
    Quit,
}

// --- Update ---

fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Quit => model.running_state = RunningState::Done,
    }
}

// --- View ---

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
