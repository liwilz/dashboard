// --- Event -> Message ---

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
