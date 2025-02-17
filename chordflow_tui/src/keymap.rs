use chordflow_audio::audio::play;
use chordflow_music_theory::{
    note::generate_all_roots,
    quality::Quality,
    scale::{Scale, ScaleType},
};
use chordflow_shared::{
    mode::Mode,
    progression::{Progression, ProgressionChord},
    DiatonicOption, ModeOption,
};
use crossterm::event::{KeyCode, KeyEvent};

use crate::{App, AppTab};

pub fn handle_keys(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Left => {
            app.selected_tab = AppTab::from_repr(app.prev_item::<AppTab>(app.selected_tab)).unwrap()
        }
        KeyCode::Right => {
            app.selected_tab = AppTab::from_repr(app.next_item::<AppTab>(app.selected_tab)).unwrap()
        }
        KeyCode::Esc => app.exit = true,
        KeyCode::Char('q') => app.exit = true,
        KeyCode::F(1) => {
            let has_changed = match app.selected_mode {
                ModeOption::Fourths => app
                    .practice_state
                    .set_mode(Mode::Fourths(app.fourths_selected_quality)),
                ModeOption::Random => app
                    .practice_state
                    .set_mode(Mode::Random(app.random_selected_qualities.clone())),
                ModeOption::Custom => app
                    .practice_state
                    .set_mode(Mode::Custom(app.custom_parsed_progression.clone())),
                ModeOption::Diatonic => app.practice_state.set_mode(Mode::Diatonic(
                    Scale::new(app.diatonic_selected_root, ScaleType::Diatonic),
                    app.diatonic_selected_option,
                )),
            };
            if let Mode::Custom(Some(p)) = &app.practice_state.mode {
                let current_idx = (app.practice_state.next_progression_chord_idx + p.chords.len()
                    - 1)
                    % p.chords.len();
                app.metronome.num_bars = p.chords[current_idx].bars;
            }
            if has_changed {
                app.metronome.reset();
                app.metronome.reset_timers();
                app.audio.sink.stop();
                play(
                    &mut app.audio.synth,
                    &app.audio.sink,
                    app.practice_state.current_chord,
                    app.metronome.duration_per_bar,
                    app.metronome.num_beats,
                );
            }
        }
        _ => {}
    }
    match app.selected_tab {
        AppTab::Playback => match key.code {
            KeyCode::Char(' ') => app.metronome.toggle(),
            KeyCode::Up => app.metronome.increase_bpm(2),
            KeyCode::Down => app.metronome.decrease_bpm(2),
            KeyCode::Char('r') => {
                app.practice_state.reset();
                app.metronome.reset();
                app.metronome.reset_timers();
                app.audio.sink.stop();
                play(
                    &mut app.audio.synth,
                    &app.audio.sink,
                    app.practice_state.current_chord,
                    app.metronome.duration_per_bar,
                    app.metronome.num_beats,
                );
            }
            _ => {}
        },

        AppTab::Mode => match key.code {
            KeyCode::Up => {
                app.selected_mode =
                    ModeOption::from_repr(app.prev_item::<ModeOption>(app.selected_mode)).unwrap()
            }
            KeyCode::Down => {
                app.selected_mode =
                    ModeOption::from_repr(app.next_item::<ModeOption>(app.selected_mode)).unwrap()
            }
            _ => {}
        },
        AppTab::Config => match app.selected_mode {
            ModeOption::Fourths => match key.code {
                KeyCode::Up => {
                    app.fourths_selected_quality =
                        Quality::from_repr(app.prev_item::<Quality>(app.fourths_selected_quality))
                            .unwrap();
                }
                KeyCode::Down => {
                    app.fourths_selected_quality =
                        Quality::from_repr(app.next_item::<Quality>(app.fourths_selected_quality))
                            .unwrap();
                }
                _ => {}
            },
            ModeOption::Random => match key.code {
                KeyCode::Up => {
                    app.random_qualities_cursor =
                        Quality::from_repr(app.prev_item::<Quality>(app.random_qualities_cursor))
                            .unwrap();
                }
                KeyCode::Down => {
                    app.random_qualities_cursor =
                        Quality::from_repr(app.next_item::<Quality>(app.random_qualities_cursor))
                            .unwrap();
                }
                KeyCode::Char(' ') => {
                    if app
                        .random_selected_qualities
                        .contains(&app.random_qualities_cursor)
                    {
                        if app.random_selected_qualities.len() > 1 {
                            app.random_selected_qualities
                                .retain(|&x| x != app.random_qualities_cursor);
                        }
                    } else {
                        app.random_selected_qualities
                            .push(app.random_qualities_cursor);
                    }
                }
                _ => {}
            },
            ModeOption::Diatonic => match key.code {
                KeyCode::Down => {
                    let all_roots = generate_all_roots();
                    let position = all_roots
                        .iter()
                        .position(|&x| x == app.diatonic_selected_root);
                    let next_position = (position.unwrap() + 1) % all_roots.len();

                    app.diatonic_selected_root = all_roots[next_position];
                }
                KeyCode::Up => {
                    let all_roots = generate_all_roots();
                    let position = all_roots
                        .iter()
                        .position(|&x| x == app.diatonic_selected_root);
                    let next_position = if position.unwrap() == 0 {
                        all_roots.len() - 1
                    } else {
                        position.unwrap() - 1
                    };
                    app.diatonic_selected_root = all_roots[next_position];
                }
                KeyCode::Tab => {
                    app.diatonic_selected_option = DiatonicOption::from_repr(
                        app.next_item::<DiatonicOption>(app.diatonic_selected_option),
                    )
                    .unwrap();
                }
                _ => {}
            },
            ModeOption::Custom => match key.code {
                KeyCode::Enter => {
                    let progression = ProgressionChord::from_str(app.custom_input_buffer.clone());
                    app.custom_parsed_progression = match progression {
                        Ok(progression) => Some(Progression {
                            chords: progression,
                        }),
                        _ => None,
                    }
                }
                KeyCode::Backspace => {
                    app.custom_input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    app.custom_input_buffer.push(c);
                }
                _ => {}
            },
        },
    }
}
