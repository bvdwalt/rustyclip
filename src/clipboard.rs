use arboard::Clipboard;

pub fn get_clipboard_text() -> anyhow::Result<String> {
    let mut clipboard = Clipboard::new()?;
    let text = clipboard.get_text()?;
    Ok(text)
}
