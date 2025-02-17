use chordflow_music_theory::{note::generate_all_roots, quality::Quality};
use chordflow_shared::metronome::Metronome;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use strum::IntoEnumIterator;

use crate::{App, AppTab, DiatonicOption, ModeOption};

pub fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),  // Tabs
                Constraint::Min(10),    // Content Area
                Constraint::Length(10), // Content Area
            ]
            .as_ref(),
        )
        .split(f.area());

    let keymap_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Tabs
                Constraint::Percentage(50), // Content Area
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    // Tabs
    let tab_titles = AppTab::iter();
    let tabs = Tabs::new(
        tab_titles
            .map(|t| t.as_ref().to_string())
            .map(Span::from)
            .collect::<Vec<Span>>(),
    )
    .block(Block::default().borders(Borders::ALL).title(" Tabs "))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .select(AppTab::iter().position(|t| t == app.selected_tab).unwrap());

    f.render_widget(tabs, chunks[0]);

    f.render_widget(
        render_local_keymap(vec![
            "( q / Esc ) : Quit",
            "( ← / → ) : Navigate tabs",
            "( F1 ) : Apply configurations",
        ]),
        keymap_chunks[0],
    );

    // Content Area
    match app.selected_tab {
        AppTab::Mode => {
            render_mode_tab(f, chunks[1], app);
        }
        AppTab::Config => render_config_tab(app, f, chunks[1], keymap_chunks[1]),
        AppTab::Playback => render_playback_tab(app, f, chunks[1], keymap_chunks[1]),
    }
}

fn render_mode_tab(f: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let items: Vec<ListItem> = ModeOption::iter()
        .map(|mode| {
            let style = if mode == app.selected_mode {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(mode.as_ref().to_string(), style))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Mode "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, area);
}

fn render_config_tab(app: &App, f: &mut Frame, area: ratatui::layout::Rect, keymap_chunks: Rect) {
    match app.selected_mode {
        ModeOption::Fourths => render_fourths_config(f, area, app, keymap_chunks),
        ModeOption::Random => render_random_config(f, area, app, keymap_chunks),
        ModeOption::Custom => render_custom_config(f, area, app, keymap_chunks),
        ModeOption::Diatonic => render_diatonic_config(f, area, app, keymap_chunks),
    }
}

fn render_diatonic_config(f: &mut Frame, area: Rect, app: &App, keymap_chunks: Rect) {
    let all_roots = generate_all_roots();
    f.render_widget(
        render_local_keymap(vec![
            "( <TAB> ) : Select mode",
            "( ↑ / ↓ ) : Select root major",
        ]),
        keymap_chunks,
    );
    let items: Vec<ListItem> = DiatonicOption::iter()
        .map(|mode| {
            let style = if mode == app.diatonic_selected_option {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(mode.as_ref().to_string(), style))
        })
        .collect();

    let roots: Vec<ListItem> = all_roots
        .iter()
        .map(|note| {
            let style = if note == &app.diatonic_selected_root {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(format!("{}", note), style))
        })
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(10)].as_ref())
        .split(area);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Diatonic Progression "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let root_list = List::new(roots)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Root "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, chunks[0]);
    f.render_widget(root_list, chunks[1]);
}

fn render_custom_config(f: &mut Frame, area: Rect, app: &App, keymap_chunks: Rect) {
    f.render_widget(
        render_local_keymap(vec!["( Enter ) : Parse input"]),
        keymap_chunks,
    );
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(area);

    let input = Paragraph::new(
        Line::from(app.custom_input_buffer.clone()).style(Style::default().fg(Color::White)),
    )
    .block(
        Block::default()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .title(" Input "),
    );

    let progression = match app.custom_parsed_progression.clone() {
        Some(progression) => progression
            .chords
            .iter()
            .flat_map(|c| {
                vec![
                    Span::from(c.bars.to_string()).style(Style::default().fg(Color::Yellow)),
                    Span::from("x"),
                    Span::from(c.chord.to_string().to_string()).style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::from("  "),
                ]
            })
            .collect::<Vec<Span>>(),
        None => {
            vec![Span::from("Invalid progression".to_string())
                .style(Style::default().fg(Color::Red))]
        }
    };

    let progression_block = Paragraph::new(Line::from(progression.clone()))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Parsed Progression "),
        );

    f.render_widget(input, chunks[0]);
    f.render_widget(progression_block, chunks[1]);
}

fn render_random_config(f: &mut Frame, area: Rect, app: &App, keymap_chunks: Rect) {
    f.render_widget(
        render_local_keymap(vec![
            "( ↑ / ↓ ) : Select quality",
            "( <SPACE> ) : Toggle selection",
        ]),
        keymap_chunks,
    );
    let items: Vec<ListItem> = Quality::iter()
        .map(|quality| {
            let prefix = if app.random_selected_qualities.contains(&quality) {
                "[✔]"
            } else {
                "[ ]"
            };
            let style = if quality == app.random_qualities_cursor {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(
                format!("{} {}", prefix, quality.name()),
                style,
            ))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Qualities "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, area);
}

fn render_local_keymap(lst: Vec<&str>) -> List<'_> {
    let local_keys: Vec<ListItem> = lst.iter().map(|&key| ListItem::new(key)).collect();
    let local_key_list = List::new(local_keys)
        .block(Block::default().borders(Borders::ALL).title(" KeyMap "))
        .style(Style::default().fg(Color::Yellow));
    local_key_list
}

fn render_fourths_config(f: &mut Frame, area: Rect, app: &App, keymap_chunks: Rect) {
    f.render_widget(
        render_local_keymap(vec!["( ↑ / ↓ ) : Select quality"]),
        keymap_chunks,
    );

    let items: Vec<ListItem> = Quality::iter()
        .map(|selection| {
            let style = if selection == app.fourths_selected_quality {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(selection.name(), style))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Select Quality "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, area);
}

fn render_playback_tab(app: &App, f: &mut Frame, area: ratatui::layout::Rect, keymap_chunks: Rect) {
    f.render_widget(
        render_local_keymap(vec![
            "( ↑ / ↓ ) : Increase / Decrease BPM with 2",
            ("( r ) : Restart"),
        ]),
        keymap_chunks,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Min(10),
            ]
            .as_ref(),
        )
        .split(area);

    let metronome_display = generate_metronome_display(&app.metronome);

    let debug_text = format!(
        "Bar: {} Beat: {} Ended: {}",
        app.metronome.current_bar, app.metronome.current_beat, app.metronome.beat_timer.ended
    );

    let metronome_paragraph = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Speed: ".to_string(), Style::default()),
            Span::styled(
                format!("{} ", app.metronome.bpm),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled("BPM".to_string(), Style::default()),
        ])
        .alignment(Alignment::Left),
        Line::from(""),
        Line::from(metronome_display).alignment(Alignment::Center),
        #[cfg(debug_assertions)]
        Line::from(debug_text),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Metronome ")
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Center),
    );

    f.render_widget(metronome_paragraph, chunks[0]);

    let current_chord = Line::from(Span::styled(
        format!("Current chord: {}", app.practice_state.current_chord),
        Style::default().fg(Color::Cyan),
    ));

    let next_chord = Line::from(Span::styled(
        format!("Next chord: {}", app.practice_state.next_chord),
        Style::default(),
    ));

    let chord_paragraph = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Mode: ".to_string(), Style::default()),
            Span::styled(
                format!("{}", app.practice_state.mode),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        current_chord,
        next_chord,
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("🎵 Chords 🎵")
            .title_alignment(Alignment::Center),
    )
    .alignment(Alignment::Left);

    f.render_widget(chord_paragraph, chunks[1]);
}

fn generate_metronome_display(metronome: &Metronome) -> String {
    let mut metronome_display = String::new();

    for bar in 0..metronome.num_bars {
        if bar > 0 {
            metronome_display.push_str(" | "); // Separate bars
        }
        for tick in 0..metronome.num_beats {
            if bar < metronome.current_bar
                || (bar == metronome.current_bar && tick <= metronome.current_beat)
            {
                metronome_display.push('⬛');
            } else {
                metronome_display.push('⬜');
            }
        }
    }

    metronome_display
}

#[cfg(test)]
mod tests {

    use std::time::Instant;

    use super::*;

    #[test]
    fn test_generate_metronome_display() {
        let mut metronome = Metronome::new(100, 2, 4, Instant::now);
        assert_eq!(
            generate_metronome_display(&metronome),
            "⬛⬜⬜⬜ | ⬜⬜⬜⬜"
        );

        metronome.current_beat = 2;
        assert_eq!(
            generate_metronome_display(&metronome),
            "⬛⬛⬛⬜ | ⬜⬜⬜⬜"
        );
        metronome.current_bar = 1;
        assert_eq!(
            generate_metronome_display(&metronome),
            "⬛⬛⬛⬛ | ⬛⬛⬛⬜"
        );
    }
}
