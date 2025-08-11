pub const VOLUME_MAPPING: kira::Mapping<kira::Decibels> = kira::Mapping {
    input_range: (0.0, 1.0),
    output_range: (kira::Decibels::SILENCE, kira::Decibels::IDENTITY),
    easing: kira::Easing::OutPowf(3.0), // more realistic volume scaling
};
