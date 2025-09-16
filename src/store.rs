use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};

const STORE_FILE: &str = "rustyclip.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ClipEntry {
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

pub fn load_entries() -> anyhow::Result<Vec<ClipEntry>> {
    if !std::path::Path::new(STORE_FILE).exists() {
        return Ok(vec![]);
    }

    let mut file = OpenOptions::new().read(true).open(STORE_FILE)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let entries: Vec<ClipEntry> = serde_json::from_str(&content)?;
    Ok(entries)
}

pub fn save_entries(entries: &[ClipEntry]) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(entries)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(STORE_FILE)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
