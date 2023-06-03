use map_range::MapRange;
use nih_plug::prelude::*;
use std::sync::Arc;

struct CCMapRange {
    params: Arc<PluginParams>,
}

/// The [`Params`] derive macro gathers all of the information needed for the wrapper to know about
/// the plugin's parameters, persistent serializable fields, and nested parameter groups. You can
/// also easily implement [`Params`] by hand if you want to, for instance, have multiple instances
/// of a parameters struct for multiple identical oscillators/filters/envelopes.
#[derive(Params)]
struct PluginParams {
    /// The parameter's ID is used to identify the parameter in the wrapped plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.

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
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            cc: IntParam::new("CC", 0, IntRange::Linear { min: 0, max: 127 }),
            in_low: FloatParam::new("In Low", 0., FloatRange::Linear { min: 0., max: 1. }),
            in_high: FloatParam::new("In Low", 0., FloatRange::Linear { min: 0., max: 1. }),
            out_low: FloatParam::new("In Low", 0., FloatRange::Linear { min: 0., max: 1. }),
            out_high: FloatParam::new("In Low", 0., FloatRange::Linear { min: 0., max: 1. }),
        }
    }
}

impl Plugin for CCMapRange {
    const NAME: &'static str = "MIDCIRCUIT CC Map Range";
    const VENDOR: &'static str = "notblank00 (Igor Gunin)";
    // You can use `env!("CARGO_PKG_HOMEPAGE")` to reference the homepage field from the
    // `Cargo.toml` file here
    const URL: &'static str = "https://github.com/notblank00/midcircuit";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    // Setting this to `true` will tell the wrapper to split the buffer up into smaller blocks
    // whenever there are inter-buffer parameter changes. This way no changes to the plugin are
    // required to support sample accurate automation and the wrapper handles all of the boring
    // stuff like making sure transport and other timing information stays consistent between the
    // splits.
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    // This plugin doesn't need any special initialization, but if you need to do anything expensive
    // then this would be the place. State is kept around when the host reconfigures the
    // plugin. If we do need special initialization, we could implement the `initialize()` and/or
    // `reset()` methods

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

    // This can be used for cleaning up special resources like socket connections whenever the
    // plugin is deactivated. Most plugins won't need to do anything here.
    fn deactivate(&mut self) {}
}

// impl ClapPlugin for CCMapRange {
//     const CLAP_ID: &'static str = "com.midcircuit.maprange";
//     const CLAP_DESCRIPTION: Option<&'static str> = Some("A smoothed gain parameter example plugin");
//     const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
//     const CLAP_SUPPORT_URL: Option<&'static str> = None;
//     const CLAP_FEATURES: &'static [ClapFeature] = &[
//         ClapFeature::AudioEffect,
//         ClapFeature::Stereo,
//         ClapFeature::Mono,
//         ClapFeature::Utility,
//     ];
// }

impl Vst3Plugin for CCMapRange {
    const VST3_CLASS_ID: [u8; 16] = *b"MIDCRTCCMapRange";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

// nih_export_clap!(Gain);
nih_export_vst3!(CCMapRange);
