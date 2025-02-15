#[derive(Default)]
struct ScreenState {
    // screen states
    exit: bool,
    tab: Tab,
    fourths: Quality,
    fourths_cursor: Quality,
    random: Vec<Quality>,
    random_cursor: usize,
    custom_input_buffer: String,
}

struct App {
    screen_state: ScreenState,
    practice_state: PracticState,
    metronome: Metronome,
    error_messsage: Option<String>,
    sink: Sink,
    synth: Synth,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.metronome.start();
        play(
            &mut self.synth,
            &self.sink,
            self.practice_state.current_chord,
            self.metronome.duration_per_bar,
            self.metronome.num_beats,
        );
        while !self.screen_state.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.update();
            self.handle_events()?;
        }
        Ok(())
    }

    fn update(&mut self) {
        self.metronome.tick();
        if self.metronome.bar_timer.ended && self.metronome.current_bar == 0 {
            self.metronome.reset();
            self.practice_state.next_chord();
        }
        if self.metronome.bar_timer.ended {
            play(
                &mut self.synth,
                &self.sink,
                self.practice_state.current_chord,
                self.metronome.duration_per_bar as u64,
                self.metronome.num_beats,
            );
        }
    }

    fn metronome_display(&self) -> Paragraph<'_> {
        let mut metronome_display = String::new();

        for bar in 0..self.metronome.num_bars {
            let current_tick = self.metronome.current_bar * self.metronome.num_beats
                + self.metronome.current_beat_step;

            if bar > 0 {
                metronome_display.push_str(" | "); // Separate bars
            }
            for tick in 0..self.metronome.num_beats {
                let this_tick = tick + bar * self.metronome.num_beats;
                if this_tick <= current_tick {
                    metronome_display.push_str("⬜"); // Inactive tick
                } else {
                    metronome_display.push_str("⬛"); // Active tick
                }
            }
        }

        let metronome_paragraph = Paragraph::new(metronome_display)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 🎵 Metronome 🎵")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center);

        metronome_paragraph
    }

    fn chords_display(&self) -> Paragraph<'_> {
        let current_chord = Line::from(Span::styled(
            format!("🎸 Current chord: {}", self.practice_state.current_chord),
            Style::default().fg(Color::Cyan),
        ));

        let next_chord = Line::from(Span::styled(
            format!("🎼 Next chord: {}", self.practice_state.next_chord),
            Style::default(),
        ));

        let chord_paragraph = Paragraph::new(vec![current_chord, next_chord])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎵 Chords 🎵")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Left);

        chord_paragraph
    }

    fn practice_mode_display(&self) -> Paragraph<'_> {
        let bpm_display = vec![
            Line::from(vec![
                Span::styled(format!("🎚  Speed: "), Style::default()),
                Span::styled(
                    format!("{} ", self.metronome.bpm),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(format!(" BPM"), Style::default()),
            ]),
            Line::from(vec![
                Span::styled(format!("🎛  Mode: "), Style::default()),
                Span::styled(
                    format!("{}", self.practice_state.mode),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
        ];
        let bpm_paragraph = Paragraph::new(bpm_display)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎵 Tempo 🎵")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Left);
        bpm_paragraph
    }

    fn keys_display(&self) -> List<'_> {
        let key_bindings = vec![
            "( q ) : Quit",
            "( ↑ / ↓ ) : Navigate modes",
            "( Enter ) : Select mode",
            "( j / k ) : Navigate quality",
            "( Space ) : Toggle quality",
            "( → / ← ) : Increase / Decrease bars per chord",
            "( [ / ] ) : Decrease / Increase time feel",
            "( - / =  ) : Decrease / Increase BPM with 2",
        ];

        let key_items: Vec<ListItem> = key_bindings.iter().map(|&key| ListItem::new(key)).collect();
        let key_list = List::new(key_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Key Bindings "),
            )
            .style(Style::default().fg(Color::Yellow));
        key_list
    }

    fn draw(&self, frame: &mut Frame) {
        let size = frame.area();
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font
            .convert(&format!("{}", self.practice_state.current_chord))
            .unwrap();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Metronome
                Constraint::Length(4), // Chords
                Constraint::Length(4), // BPM
                Constraint::Min(0),    // Instructions
            ])
            .split(size);

        frame.render_widget(self.metronome_display(), chunks[0]);
        frame.render_widget(self.chords_display(), chunks[1]);
        frame.render_widget(self.practice_mode_display(), chunks[2]);

        //frame.render_widget(self.keys_display(), bottom_chunks[0]); // Left: Key List
        let tabs = Tabs::new(Tab::all().iter().map(|t| t.name()))
            .block(Block::bordered().title("Tabs"))
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(2)
            .divider(symbols::DOT)
            .padding("->", "<-");
        frame.render_widget(tabs, chunks[3]);
    }

    fn modal_display(&self, f: &mut Frame) {
        let size = f.area();
        let block = Block::default()
            .title("Mode Selection")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray));
        let modal_area = centered_rect(size, 50, 50);
        Clear.render(modal_area, f.buffer_mut());
        f.render_widget(block, modal_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(modal_area);

        let mode_options = vec!["Fourths", "Random", "Custom"];
        let mode_list = List::new(mode_options.iter().map(|m| ListItem::new(*m)))
            .block(Block::default().title("Modes").borders(Borders::ALL))
            .style(
                Style::default().fg(if self.screen_state == State::Modal(ModalState::Main) {
                    Color::Green
                } else {
                    Color::White
                }),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(
            mode_list,
            chunks[0],
            &mut ListState::default().with_selected(Some(self.mode_selection.selected_mode)),
        );

        let subchoice_block = Block::default().title("Subchoice").borders(Borders::ALL);

        match self.mode_selection.selected_mode {
            0 => {
                let quality_options = vec!["Major", "Minor", "Augmented", "Diminished"];
                let quality_list = List::new(quality_options.iter().map(|q| ListItem::new(*q)))
                    .block(subchoice_block)
                    .style(Style::default().fg(
                        if self.screen_state == State::Modal(ModalState::Secondary) {
                            Color::Green
                        } else {
                            Color::White
                        },
                    ))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    );
                f.render_stateful_widget(
                    quality_list,
                    chunks[1],
                    &mut ListState::default()
                        .with_selected(Some(self.mode_selection.selected_quality)),
                );
            }
            1 => {
                let quality_options = vec!["Major", "Minor", "Augmented", "Diminished"];
                let items = quality_options
                    .iter()
                    .enumerate()
                    .map(|(i, q)| {
                        let selected = self.mode_selection.selected_qualities.contains(&i);
                        let indicator = if selected { "[✔] " } else { "[ ] " };
                        ListItem::new(format!("{}{}", indicator, q))
                    })
                    .collect::<Vec<_>>();
                let list = List::new(items)
                    .block(subchoice_block)
                    .style(Style::default().fg(
                        if self.screen_state == State::Modal(ModalState::Secondary) {
                            Color::Green
                        } else {
                            Color::White
                        },
                    ))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    );
                f.render_widget(list, chunks[1]);
            }
            3 => {
                let input = Paragraph::new(self.mode_selection.input_buffer.clone())
                    .block(subchoice_block)
                    .style(Style::default().fg(Color::White));
                f.render_widget(input, chunks[1]);
            }
            _ => {}
        }
    }

    fn handle_practice_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.screen_state.exit = true;
            }
            KeyCode::Char('=') => {
                self.metronome.bpm += 2;
            }
            KeyCode::Char('-') => {
                self.metronome.bpm -= 2;
            }
            KeyCode::Left => {
                self.metronome.num_bars = self.metronome.num_bars.saturating_sub(1);
            }
            KeyCode::Right => {
                self.metronome.num_bars += 1;
            }
            KeyCode::Char('[') => {
                self.metronome.num_beats = self.metronome.num_beats.saturating_sub(1);
            }
            KeyCode::Char(']') => {
                self.metronome.num_beats += 1;
            }
            _ => {}
        }
    }
    fn handle_modal_main_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen_state = State::Practice;
            }
            KeyCode::Up => {
                self.mode_selection.selected_mode = if self.mode_selection.selected_mode == 0 {
                    2
                } else {
                    self.mode_selection.selected_mode - 1
                };
            }
            KeyCode::Down => {
                self.mode_selection.selected_mode = (self.mode_selection.selected_mode + 1) % 3;
            }
            KeyCode::Enter => {
                self.screen_state = State::Modal(ModalState::Secondary);
            }
            _ => {}
        }
    }

    fn handle_modal_secondary_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.screen_state = State::Modal(ModalState::Main);
            }
            KeyCode::Up => {
                self.mode_selection.selected_quality = if self.mode_selection.selected_quality == 0
                {
                    2
                } else {
                    self.mode_selection.selected_quality - 1
                };
            }
            KeyCode::Down => {
                self.mode_selection.selected_quality =
                    (self.mode_selection.selected_quality + 1) % 3;
            }
            KeyCode::Char(' ') => {}
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                self.handle_practice_keys(key);
                match self.screen_state.tab {
                    Tab::Fourths => self.handle_practice_keys(key),
                    Tab::Random => self.handle_modal_main_keys(key),
                    Tab::Custom => self.handle_modal_secondary_keys(key),
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
        Ok(())
    }

    fn new(cli: Cli, sink: Sink, synth: Synth) -> Self {
        Self {
            screen_state: ScreenState::default(),
            practice_state: PracticState::default(),
            metronome: Metronome::new(cli.bpm, cli.bars_per_chord, cli.ticks_per_bar),
            error_messsage: None,
            sink,
            synth,
        }
    }
}
