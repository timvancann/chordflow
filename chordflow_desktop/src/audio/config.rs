pub fn create_synth(soundfont_path: Option<PathBuf>) -> Synthesizer {
    let sample_rate = 44100;
    let settings = SynthesizerSettings::new(sample_rate);

    let sound_font = if let Some(path) = soundfont_path {
        let mut file = File::open(path).expect("Failed to open SoundFont file");
        SoundFont::new(&mut file).expect("Failed to load SoundFont")
    } else {
        let soundfont_bytes = include_bytes!("../assets/TimGM6mb.sf2");
        let mut cursor = Cursor::new(soundfont_bytes);
        SoundFont::new(&mut cursor).expect("Failed to load embedded SoundFont")
    };

    let sound_font = Arc::new(sound_font);
    Synthesizer::new(&sound_font, &settings).expect("Failed to create synthesizer")
}
