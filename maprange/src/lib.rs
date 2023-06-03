use map_range::MapRange;
use nih_plug::prelude::*;
use std::sync::Arc;

struct CCMapRange {
    params: Arc<PluginParams>,
}

#[derive(Params)]
struct PluginParams {
    #[id = "cc"]
    pub cc: IntParam,

    #[id = "in_low"]
    pub in_low: FloatParam,

    #[id = "in_high"]
    pub in_high: FloatParam,

    #[id = "out_low"]
    pub out_low: FloatParam,

    #[id = "out_high"]
    pub out_high: FloatParam,
}

impl Default for CCMapRange {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            cc: IntParam::new("CC", 0, IntRange::Linear { min: 0, max: 127 }),
            in_low: FloatParam::new("In Low", 0., FloatRange::Linear { min: 0., max: 1. }),
            in_high: FloatParam::new("In High", 0., FloatRange::Linear { min: 0., max: 1. }),
            out_low: FloatParam::new("Out Low", 0., FloatRange::Linear { min: 0., max: 1. }),
            out_high: FloatParam::new("Out High", 0., FloatRange::Linear { min: 0., max: 1. }),
        }
    }
}

impl Plugin for CCMapRange {
    const NAME: &'static str = "MIDCIRCUIT CC Map Range";
    const VENDOR: &'static str = "notblank00 (Igor Gunin)";

    const URL: &'static str = "https://github.com/notblank00/midcircuit";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // this plugin doesn't use audio
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::MidiCC {
                    timing,
                    channel,
                    cc,
                    value,
                } => context.send_event(NoteEvent::MidiCC {
                    timing,
                    channel,
                    cc,
                    value: if cc as i32 == self.params.cc.value() {
                        value.map_range(
                            self.params.in_low.value()..self.params.in_high.value(),
                            self.params.out_low.value()..self.params.out_high.value(),
                        )
                    } else {
                        value
                    },
                }),
                _ => context.send_event(event),
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for CCMapRange {
    const CLAP_ID: &'static str = "com.midcircuit.maprange";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("MIDI CC Map Range utility");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for CCMapRange {
    const VST3_CLASS_ID: [u8; 16] = *b"MIDCRTCCMapRange";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(CCMapRange);
nih_export_vst3!(CCMapRange);
