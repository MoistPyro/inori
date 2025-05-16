use super::*;
use crate::model::*;
use crate::util::safe_add;
use crate::util::safe_subtract;
use event::KeyModifiers;
use nucleo_matcher::Matcher;
use proto::*;

pub mod library_handler;
pub mod queue_handler;

pub fn handle_vertical(msg: Vertical, selector: &mut impl Selector) {
    match selector.selected() {
        None => {
            if selector.len() != 0 {
                selector.set_selected(Some(0));
            }
        }
        Some(sel) => selector.set_selected(match msg {
            Vertical::Up => Some(safe_subtract(sel, 1, selector.len())),
            Vertical::Down => Some(safe_add(sel, 1, selector.len())),
            Vertical::Top => Some(0),
            Vertical::Bottom => {
                Some(safe_subtract(selector.len(), 1, selector.len()))
            }
        }),
    }
}

pub fn scroll_screenful(
    dir: Vertical,
    height: usize,
    selector: &mut impl Selector,
) {
    let len = selector.len();
    match dir {
        Vertical::Up => {
            selector.set_selected(Some(selector.offset()));
            selector.set_offset(safe_subtract(selector.offset(), height, len));
        }
        Vertical::Down => {
            let mut next = safe_add(selector.offset(), height, len);
            if next > 0 && next < safe_subtract(len, 1, len) {
                next = safe_subtract(next, 3, len);
                selector.set_offset(next);
            }
            selector.set_selected(Some(next));
        }
        _ => {}
    }
}

// TODO: Figure out a way to eliminate code duplication here
pub fn handle_search_k_tracksel(
    artist: &mut ArtistData,
    k: KeyEvent,
    matcher: &mut Matcher,
) -> Option<Message> {
    if k.modifiers.contains(KeyModifiers::CONTROL) {
        match k.code {
            // TODO: keep track of cursor and implement AEFB
            KeyCode::Char('u') => artist.search.query.clear(),
            KeyCode::Char('n') => {
                if let Some(Some(r)) = artist.selected_item().map(|i| i.rank) {
                    let idx = artist
                        .contents()
                        .iter()
                        .position(|i| i.rank == Some(r + 1));
                    if idx.is_some() {
                        artist.set_selected(idx)
                    }
                }
            }
            KeyCode::Char('p') => {
                if let Some(Some(r)) = artist.selected_item().map(|i| i.rank) {
                    if r > 0 {
                        artist.set_selected(
                            artist
                                .contents()
                                .iter()
                                .position(|i| i.rank == Some(r - 1)),
                        );
                    }
                }
            }
            _ => {}
        }
    } else {
        match k.code {
            KeyCode::Char(c) => artist.search.query.push(c),
            KeyCode::Backspace => {
                let _ = artist.search.query.pop();
            }
            KeyCode::Esc => {
                return Some(Message::LocalSearch(SearchMsg::End));
            }
            KeyCode::Enter => return Some(Message::Select),
            _ => {}
        }
    }
    artist.update_search(matcher);
    None
}

pub fn handle_search_k<T>(
    s: &mut impl Searchable<T>,
    k: KeyEvent,
    matcher: &mut Matcher,
    top_k: usize,
) -> Option<Message> {
    if k.modifiers.contains(KeyModifiers::CONTROL) {
        match k.code {
            // TODO: keep track of cursor and implement AEFB
            KeyCode::Char('u') => s.filter_mut().query.clear(),
            KeyCode::Char('n') => handle_vertical(Vertical::Down, s),
            KeyCode::Char('p') => handle_vertical(Vertical::Up, s),
            _ => {}
        }
    } else {
        match k.code {
            KeyCode::Char(c) => {
                s.filter_mut().query.push(c);
            }
            KeyCode::Backspace => {
                let _ = s.filter_mut().query.pop();
            }
            KeyCode::Esc => {
                return Some(Message::LocalSearch(SearchMsg::End));
            }
            KeyCode::Enter => return Some(Message::Select),
            _ => {}
        }
    }
    s.update_filter_cache(matcher, Some(top_k));
    s.watch_oob();
    None
}
