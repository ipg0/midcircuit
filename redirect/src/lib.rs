use nih_plug::prelude::*;
use std::sync::Arc;

struct CCRedirect {
    params: Arc<PluginParams>,
}

#[derive(Params)]
struct PluginParams {
    #[id = "cc_from"]
    pub cc_from: IntParam,

    #[id = "cc_to"]
    pub cc_to: IntParam,
}

impl Default for CCRedirect {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            cc_from: IntParam::new("CC From", 0, IntRange::Linear { min: 0, max: 127 }),
            cc_to: IntParam::new("CC To", 0, IntRange::Linear { min: 0, max: 127 }),
        }
    }
}

impl Plugin for CCRedirect {
    const NAME: &'static str = "MIDCIRCUIT CC Redirect";
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
                } => {
                    if cc as i32 == self.params.cc_from.value() {
                        context.send_event(NoteEvent::MidiCC {
                            timing,
                            channel,
                            self.params.cc_to.value(),
                            value,
                        })
                    }
                }
                _ => context.send_event(event),
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

// impl ClapPlugin for CCRedirect {
//     const CLAP_ID: &'static str = "com.midcircuit.maprange";
//     const CLAP_DESCRIPTION: Option<&'static str> = Some("MIDI CC Invert utility");
//     const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
//     const CLAP_SUPPORT_URL: Option<&'static str> = None;
//     const CLAP_FEATURES: &'static [ClapFeature] = &[
//         ClapFeature::Utility,
//     ];
// }

impl Vst3Plugin for CCRedirect {
    const VST3_CLASS_ID: [u8; 16] = *b"MIDCRTCCRedirect";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

// nih_export_clap!(CCRedirect);
nih_export_vst3!(CCRedirect);
