# 🎸 ChordFlow

ChordFlow is a TUI (Terminal User Interface) tool designed to help guitarists/musicians practice improvisation and master the guitar neck by providing dynamic chord progressions with a built-in metronome.

## ✨ Features

- 🎵 Metronome with Custom Sounds – Supports SoundFont-based metronome ticks.
- 🔄 Random Chord Generation – Generate new chords every bar to improve improvisation skills.
- 📊 Visual Progress Bar – Displays the current beat and bar progress.
- 🎼 Real-Time Chord Display – Shows the current and upcoming chord.
- ⚙️  Customizability – Users can supply their own SoundFont for metronome ticks and chord sounds.
- 🖥️ Cross-Platform – Works on macOS, and Windows.

## 📦 Installation

1. Build from Source

  ```bash
  git clone https://github.com/timvancann/chordflow
  cd chordflow
  cargo build --release
  ```

2. Grab the latest [release](https://github.com/timvancann/chordflow/releases)

## 🚀 Usage

```bash
./chordflow --help

Usage: chordflow [OPTIONS]

Options:
      --bpm <INT>              BPM (Beats per minute) [default: 100]
  -b, --bars-per-chord <INT>   Number of bars per chord [default: 2]
  -t, --ticks-per-bar <INT>    Number of beats per bar [default: 4]
  -s, --soundfont <SOUNDFONT>  Soundfont file path
  -h, --help                   Print help
 ```

## 🏗️ Roadmap

- [ ] Fix Linux release
- [ ] Add more scales (e.g. melodic minor)
- [ ] Better feedback and UI on custom progressions
- [ ] Allow dynamically update the number of beats per bar
- [ ] Use [Dioxux](https://dioxuslabs.com/) to create a GUI native app

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

1. Fork the repo
2. Create a new branch (git checkout -b feature-name)
3. Commit changes (git commit -m "Added cool feature")
4. Push to branch (git push origin feature-name)
5. Open a pull request
