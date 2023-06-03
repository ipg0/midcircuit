use nih_plug::prelude::*;
use std::sync::Arc;

struct CCSuppress {
    params: Arc<PluginParams>,
}

#[derive(Params)]
struct PluginParams {
    #[id = "cc"]
    pub cc: IntParam,
}

impl Default for CCSuppress {
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
        }
    }
}

impl Plugin for CCSuppress {
    const NAME: &'static str = "MIDCIRCUIT CC Suppress";
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
                    timing: _,
                    channel: _,
                    cc,
                    value: _,
                } => {
                    if cc as i32 != self.params.cc.value() {
                        context.send_event(event)
                    }
                }
                _ => context.send_event(event),
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

// impl ClapPlugin for CCSuppress {
//     const CLAP_ID: &'static str = "com.midcircuit.maprange";
//     const CLAP_DESCRIPTION: Option<&'static str> = Some("MIDI CC Invert utility");
//     const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
//     const CLAP_SUPPORT_URL: Option<&'static str> = None;
//     const CLAP_FEATURES: &'static [ClapFeature] = &[
//         ClapFeature::Utility,
//     ];
// }

impl Vst3Plugin for CCSuppress {
    const VST3_CLASS_ID: [u8; 16] = *b"MIDCRTCCSuppress";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

// nih_export_clap!(CCSuppress);
nih_export_vst3!(CCSuppress);
