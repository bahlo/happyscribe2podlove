use std::io;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Word {
    text: String,
    #[serde(rename = "type")]
    typ: String, // This should be an enum but whatever
    data_start: f64,
    data_end: f64,
    confidence: f64,
}

#[derive(Deserialize, Debug)]
struct Entry {
    speaker: String,
    words: Vec<Word>,
    data_start: f64,
    data_end: f64,
}

fn format_time(time: f64) -> String {
    let time = time as i64;
    let milliseconds = (time % 1000) as u32;
    let seconds = time % 60;
    let minutes = (time / 60) % 60;
    let hours = time / 3600;
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        hours, minutes, seconds, milliseconds
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json: Vec<Entry> = serde_json::from_reader(io::stdin())?;

    println!("WEBVTT\n");

    for entry in json {
        let text = entry
            .words
            .iter()
            .map(|w| w.text.trim().to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!(
            "{} --> {}",
            format_time(entry.data_start),
            format_time(entry.data_end)
        );
        println!("<v {}>{}\n", entry.speaker, text);
    }

    Ok(())
}
