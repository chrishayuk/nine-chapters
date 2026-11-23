//! The transcription, compiled in: each problem of chapter 8 and each
//! commentary that works on a board.

include!(concat!(env!("OUT_DIR"), "/corpus.rs"));

/// A page by name or problem number: `1`, `8.1`, `no-01`, `liu-hui-18-old`.
pub fn find(key: &str) -> Option<(&'static str, &'static str)> {
    let k = key.trim().trim_start_matches("8.");
    if let Ok(n) = k.parse::<u32>() {
        let name = format!("no-{:02}", n);
        return CORPUS.iter().find(|(id, _)| *id == name).copied();
    }
    CORPUS.iter().find(|(id, _)| *id == k).copied()
}
