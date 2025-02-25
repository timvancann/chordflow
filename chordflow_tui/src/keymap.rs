use chordflow_audio::audio::AudioCommand;
use chordflow_music_theory::{note::generate_all_roots, quality::Quality};
use chordflow_shared::{
    metronome::{calculate_duration_per_bar, MetronomeCommand},
    mode::update_mode_from_state,
    progression::{Progression, ProgressionChord},
    DiatonicOption, ModeOption,
};
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crate::{sync_metronome_bars, App, AppTab};

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
            let has_changed = update_mode_from_state(
                &app.selected_mode,
                &mut app.practice_state,
                &app.config_state,
            );

            sync_metronome_bars(app);
            if has_changed {
                let _ = app.tx_metronome.send(MetronomeCommand::Reset);
                app.current_bar = 0;
                app.current_tick = 0;
                let _ = app.tx_audio.send(AudioCommand::PlayChord((
                    app.practice_state.current_chord,
                    calculate_duration_per_bar(app.bpm, app.ticks_per_bar).duration_per_bar,
                    app.ticks_per_bar,
                )));
            }
        }
        _ => {}
    }
    match app.selected_tab {
        AppTab::Playback => match key.code {
            KeyCode::Up => {
                app.bpm += 2;
                let _ = app.tx_metronome.send(MetronomeCommand::IncreaseBpm(2));
                sync_metronome_bars(app);
            }
            KeyCode::Down => {
                app.bpm -= 2;
                let _ = app.tx_metronome.send(MetronomeCommand::DecreaseBpm(2));
                sync_metronome_bars(app);
            }
            KeyCode::Char('r') => {
                app.practice_state.reset();
                app.current_bar = 0;
                app.current_tick = 0;
                let _ = app.tx_metronome.send(MetronomeCommand::Reset);
                let _ = app.tx_audio.send(AudioCommand::PlayChord((
                    app.practice_state.current_chord,
                    calculate_duration_per_bar(app.bpm, app.ticks_per_bar).duration_per_bar,
                    app.ticks_per_bar,
                )));
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
                    app.config_state.fourths_selected_quality = Quality::from_repr(
                        app.prev_item::<Quality>(app.config_state.fourths_selected_quality),
                    )
                    .unwrap();
                }
                KeyCode::Down => {
                    app.config_state.fourths_selected_quality = Quality::from_repr(
                        app.next_item::<Quality>(app.config_state.fourths_selected_quality),
                    )
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
                        .config_state
                        .random_selected_qualities
                        .contains(&app.random_qualities_cursor)
                    {
                        if app.config_state.random_selected_qualities.len() > 1 {
                            app.config_state
                                .random_selected_qualities
                                .retain(|&x| x != app.random_qualities_cursor);
                        }
                    } else {
                        app.config_state
                            .random_selected_qualities
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
                        .position(|&x| x == app.config_state.diatonic_root);
                    let next_position = (position.unwrap() + 1) % all_roots.len();

                    app.config_state.diatonic_root = all_roots[next_position];
                }
                KeyCode::Up => {
                    let all_roots = generate_all_roots();
                    let position = all_roots
                        .iter()
                        .position(|&x| x == app.config_state.diatonic_root);
                    let next_position = if position.unwrap() == 0 {
                        all_roots.len() - 1
                    } else {
                        position.unwrap() - 1
                    };
                    app.config_state.diatonic_root = all_roots[next_position];
                }
                KeyCode::Tab => {
                    app.config_state.diatonic_option = DiatonicOption::from_repr(
                        app.next_item::<DiatonicOption>(app.config_state.diatonic_option),
                    )
                    .unwrap();
                }
                _ => {}
            },
            ModeOption::Custom => match key.code {
                KeyCode::Enter => {
                    let progression =
                        ProgressionChord::from_string(app.custom_input_buffer.clone());
                    app.config_state.progression = match progression {
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
