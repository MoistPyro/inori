use mpd::Song;
use mpd::Status;
use std::time::Duration;

pub fn safe_add(idx: usize, k: usize, length: usize) -> usize {
    if length == 0 {
        return idx;
    }
    if idx + k >= length {
        return length - 1;
    }
    idx + k
}

pub fn safe_subtract(idx: usize, k: usize, length: usize) -> usize {
    if length == 0 || idx == 0 {
        return idx;
    }
    if k >= idx {
        return 0;
    }
    idx - k
}

pub fn song_album(s: &Song) -> Option<&String> {
    Some(&s.tags.iter().find(|t| t.0 == "Album")?.1)
}

pub fn format_time(d: Duration) -> String {
    let total = d.as_secs();
    let m = total / 60;
    let s = total % 60;
    if m > 59 {
        format!("{}:{:02}:{:02}", m / 60, m % 60, s)
    } else {
        format!("{}:{:02}", m, s)
    }
}

pub fn format_progress(s: &Status) -> String {
    if let (Some(e), Some(d)) = (s.elapsed, s.duration) {
        format!("{}/{}", format_time(e), &format_time(d))
    } else {
        String::new()
    }
}
pub fn song_to_str(song: &Song) -> String {
    let mut out = String::new();
    if let Some(title) = &song.title {
        out.push_str(title);
    }
    if let Some(artist) = &song.artist {
        out.push(' ');
        out.push_str(artist);
    }
    if let Some(album) = song_album(song) {
        out.push(' ');
        out.push_str(album);
    }
    out
}
