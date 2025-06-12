mod platform;

// TODO: OfflineAudioContext

pub trait AudioNode {
    #[doc(hidden)]
    fn raw(&self) -> platform::AudioNodeRef<'_>;

    fn connect<'a>(&self, to: &'a dyn AudioNode) -> &'a dyn AudioNode {
        platform::connect(self.raw(), to.raw());
        to
    }

    /// Disconnects all outgoing connections from the AudioNode.
    fn disconnect(&self) {
        platform::disconnect(self.raw());
    }
}

pub struct AudioParam(platform::AudioParam);

impl AudioParam {
    pub fn set_value(&self, value: f32) {
        self.0.set_value(value);
    }

    pub fn linear_ramp_to_value_at_time(&self, value: f32, end_time: f64) {
        self.0.linear_ramp_to_value_at_time(value, end_time);
    }

    pub fn exponential_ramp_to_value_at_time(&self, value: f32, end_time: f64) {
        self.0.exponential_ramp_to_value_at_time(value, end_time);
    }

    pub fn cancel_scheduled_changes(&self, cancel_time: f64) {
        self.0.cancel_scheduled_changes(cancel_time);
    }

    pub fn value(&self) -> f32 {
        self.0.value()
    }
}

pub struct AudioContext(platform::AudioContext);

impl AudioContext {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self(platform::AudioContext::new()))
    }

    pub fn destination(&self) -> AudioDestinationNode {
        AudioDestinationNode(self.0.destination())
    }

    pub fn listener(&self) -> AudioListener {
        AudioListener(self.0.listener())
    }

    pub async fn decode(&self, data: Vec<u8>) -> anyhow::Result<AudioBuffer> {
        Ok(AudioBuffer(self.0.decode(data).await?))
    }

    pub fn current_time(&self) -> f64 {
        self.0.current_time()
    }
}

pub struct AudioListener(platform::AudioListener);

impl AudioListener {
    pub fn set_position(&self, pos: [f32; 3]) {
        self.0.set_position(pos);
    }
    pub fn set_orientation(&self, forward: [f32; 3], up: [f32; 3]) {
        self.0.set_orientation(forward, up);
    }
}

pub struct AudioDestinationNode(platform::AudioDestinationNode);

impl AudioNode for AudioDestinationNode {
    fn raw(&self) -> platform::AudioNodeRef<'_> {
        self.0.get_ref()
    }
}

pub struct GainNode(platform::GainNode);

impl GainNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(platform::GainNode::new(&context.0))
    }

    pub fn gain(&self) -> AudioParam {
        AudioParam(self.0.gain())
    }
}

impl AudioNode for GainNode {
    fn raw(&self) -> platform::AudioNodeRef<'_> {
        self.0.get_ref()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub enum DistanceModel {
    Linear,
    #[default]
    Inverse,
    Exponential,
}

pub struct PannerNode(platform::PannerNode);

impl PannerNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(platform::PannerNode::new(&context.0))
    }

    pub fn set_distance_model(&mut self, model: DistanceModel) {
        self.0.set_distance_model(model);
    }

    pub fn set_position(&mut self, pos: [f32; 3]) {
        self.0.set_position(pos);
    }

    pub fn set_ref_distance(&mut self, ref_distance: f64) {
        self.0.set_ref_distance(ref_distance);
    }

    pub fn set_max_distance(&mut self, max_distance: f64) {
        self.0.set_max_distance(max_distance);
    }
}

impl AudioNode for PannerNode {
    fn raw(&self) -> platform::AudioNodeRef<'_> {
        self.0.get_ref()
    }
}

#[derive(Clone)]
pub struct AudioBuffer(platform::AudioBuffer);

impl AudioBuffer {
    pub fn duration(&self) -> f64 {
        self.0.duration()
    }
}

pub struct AudioBufferSourceNode(platform::AudioBufferSourceNode);

impl AudioBufferSourceNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(platform::AudioBufferSourceNode::new(&context.0))
    }

    pub fn start_with_offset(&mut self, offset: f64) {
        self.0.start_with_offset(offset);
    }

    pub fn stop(&mut self) {
        self.0.stop();
    }

    pub fn stop_at(&mut self, when: f64) {
        self.0.stop_at(when);
    }

    pub fn set_loop(&mut self, looped: bool) {
        self.0.set_loop(looped);
    }

    pub fn set_buffer(&mut self, buffer: AudioBuffer) {
        self.0.set_buffer(buffer.0);
    }

    pub fn playback_rate(&self) -> AudioParam {
        AudioParam(self.0.playback_rate())
    }

    pub fn position(&self) -> f64 {
        self.0.position()
    }
}

impl AudioNode for AudioBufferSourceNode {
    fn raw(&self) -> platform::AudioNodeRef<'_> {
        self.0.get_ref()
    }
}

/// Biquad filter types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiquadFilterType {
    /// Allows frequencies below the cutoff frequency to pass through and
    /// attenuates frequencies above the cutoff. (12dB/oct rolloff)
    Lowpass,
    /// Frequencies above the cutoff frequency are passed through, but
    /// frequencies below the cutoff are attenuated. (12dB/oct rolloff)
    Highpass,
    /// Allows a range of frequencies to pass through and attenuates the
    /// frequencies below and above this frequency range.
    Bandpass,
    /// Allows all frequencies through, except for a set of frequencies.
    Notch,
    /// Allows all frequencies through, but changes the phase relationship
    /// between the various frequencies.
    Allpass,
    /// Allows all frequencies through, but adds a boost (or attenuation) to
    /// a range of frequencies.
    Peaking,
    /// Allows all frequencies through, but adds a boost (or attenuation) to
    /// the lower frequencies.
    Lowshelf,
    /// Allows all frequencies through, but adds a boost (or attenuation) to
    /// the higher frequencies.
    Highshelf,
}

impl serde::Serialize for BiquadFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{self}").serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for BiquadFilterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Incorrect biquad filter type")]
pub struct IncorrectBiquadFilterType;

impl std::str::FromStr for BiquadFilterType {
    type Err = IncorrectBiquadFilterType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "lowpass" => Self::Lowpass,
            "highpass" => Self::Highpass,
            "bandpass" => Self::Bandpass,
            "notch" => Self::Notch,
            "allpass" => Self::Allpass,
            "peaking" => Self::Peaking,
            "lowshelf" => Self::Lowshelf,
            "highshelf" => Self::Highshelf,
            _ => return Err(IncorrectBiquadFilterType),
        })
    }
}

impl std::fmt::Display for BiquadFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BiquadFilterType::Lowpass => "lowpass",
            BiquadFilterType::Highpass => "highpass",
            BiquadFilterType::Bandpass => "bandpass",
            BiquadFilterType::Notch => "notch",
            BiquadFilterType::Allpass => "allpass",
            BiquadFilterType::Peaking => "peaking",
            BiquadFilterType::Lowshelf => "lowshelf",
            BiquadFilterType::Highshelf => "highshelf",
        };
        s.fmt(f)
    }
}

pub struct BiquadFilterNode(platform::BiquadFilterNode);

impl BiquadFilterNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(platform::BiquadFilterNode::new(&context.0))
    }

    pub fn frequency(&self) -> AudioParam {
        AudioParam(self.0.frequency())
    }

    pub fn gain(&self) -> AudioParam {
        AudioParam(self.0.gain())
    }

    pub fn q(&self) -> AudioParam {
        AudioParam(self.0.q())
    }

    pub fn detune(&self) -> AudioParam {
        AudioParam(self.0.detune())
    }

    pub fn set_type(&mut self, r#type: crate::BiquadFilterType) {
        self.0.set_type(r#type);
    }
}

impl AudioNode for BiquadFilterNode {
    fn raw(&self) -> platform::AudioNodeRef<'_> {
        self.0.get_ref()
    }
}
