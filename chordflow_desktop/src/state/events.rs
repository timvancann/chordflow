

pub struct MetronomeCommandChannel {
    pub sender: Sender<MetronomeCommand>,
    pub receiver: Receiver<MetronomeCommand>,
}
pub static METRONOME_COMMAND_CHANNEL: LazyLock<MetronomeCommandChannel> = LazyLock::new(||
    {
    let (sender, receiver) = unbounded();
    MetronomeCommandChannel { sender, receiver }
});

pub enum AudioEvent {
    BarComplete(usize),
    CycleComplete,
    Tick(usize),
}
pub struct MetronomeEventChannel {
    pub sender: Sender<MetronomeEvent>,
    pub receiver: Receiver<MetronomeEvent>,
}
pub static METRONOME_EVENT_CHANNEL: LazyLock<MetronomeEventChannel> = LazyLock::new(|| {
    let (sender, receiver) = unbounded();
    MetronomeEventChannel { sender, receiver }
});

pub enum AudioCommand {
    PlayChord(Chord),
}
pub struct AudioCommandChannel {
    pub sender: Sender<AudioCommand>,
    pub receiver: Receiver<AudioCommand>,
}
pub static AUDIO_COMMAND_CHANNEL: LazyLock<AudioCommandChannel> = LazyLock::new(|| {
    let (sender, receiver) = unbounded();
    AudioCommandChannel { sender, receiver }
});
