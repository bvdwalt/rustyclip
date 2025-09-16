use crate::{clipboard, store};
use chrono::Utc;
use store::ClipEntry;

pub fn add() -> anyhow::Result<()> {
    let text = clipboard::get_clipboard_text()?;
    let mut entries = store::load_entries()?;
    entries.push(ClipEntry {
        text,
        timestamp: Utc::now(),
    });
    store::save_entries(&entries)?;
    println!("Saved clipboard entry.");
    Ok(())
}

pub fn list() -> anyhow::Result<()> {
    let entries = store::load_entries()?;
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {}", i, entry.text.chars().take(50).collect::<String>());
    }
    Ok(())
}

pub fn get(index: usize) -> anyhow::Result<()> {
    let entries = store::load_entries()?;
    if let Some(entry) = entries.get(index) {
        println!("{}", entry.text);
    } else {
        eprintln!("No entry at index {}", index);
    }
    Ok(())
}

pub fn clear() -> anyhow::Result<()> {
    store::save_entries(&[])?;
    println!("History cleared.");
    Ok(())
}
