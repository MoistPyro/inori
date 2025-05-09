use crate::model::*;
use ratatui::prelude::*;
use ratatui::style::Color::*;
use ratatui::style::Style;
mod artist_select_renderer;
pub mod library_renderer;
pub mod queue_renderer;
mod search_renderer;
mod status_renderer;
mod track_select_renderer;

#[derive(Clone)]
pub struct Theme {
    pub block_active: Style,
    pub field_album: Style,
    pub field_artistsort: Style,
    pub item_highlight_active: Style,
    pub item_highlight_inactive: Style,
    pub progress_bar_filled: Style,
    pub progress_bar_unfilled: Style,
    pub search_query_active: Style,
    pub search_query_inactive: Style,
    pub slash_span: Style,
    pub status_album: Style,
    pub status_artist: Style,
    pub status_paused: Style,
    pub status_playing: Style,
    pub status_stopped: Style,
    pub status_title: Style,
}
impl Theme {
    pub fn new() -> Self {
        Self {
            block_active: Style::default().fg(Red),
            field_album: Style::default().bold().italic().fg(Red),
            field_artistsort: Style::default().fg(DarkGray),
            item_highlight_active: Style::default().fg(Black).bg(White),
            item_highlight_inactive: Style::default().fg(Black).bg(DarkGray),
            progress_bar_filled: Style::default()
                .fg(LightYellow)
                .bg(Black)
                .add_modifier(Modifier::BOLD),
            progress_bar_unfilled: Style::default().fg(Black),
            search_query_active: Style::default().bg(White).fg(Black),
            search_query_inactive: Style::default().bg(DarkGray).fg(Black),
            slash_span: Style::default().fg(LightMagenta),
            status_album: Style::default().bold().italic().fg(Red),
            status_artist: Style::default().fg(Cyan),
            status_paused: Style::default().fg(LightRed),
            status_playing: Style::default().fg(LightGreen),
            status_stopped: Style::default().fg(Red),
            status_title: Style::default().bold(),
        }
    }
}

pub fn view(model: &mut Model, frame: &mut Frame) {
    // only &mut for ListState/TableState updating.
    // view function should be pure!

    let theme = model.config.theme.clone();
    match model.screen {
        Screen::Library => library_renderer::render(model, frame, &theme),
        Screen::Queue => queue_renderer::render(model, frame, &theme),
    }
}
