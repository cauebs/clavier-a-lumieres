pub type ColorRGB = (u8, u8, u8);

pub const CHROMATIC_RAINBOW: [ColorRGB; 12] = [
    (0xff, 0x00, 0x00),
    (0xff, 0x80, 0x00),
    (0xff, 0xff, 0x00),
    (0x80, 0xff, 0x00),
    (0x00, 0xff, 0x00),
    (0x00, 0xff, 0x80),
    (0x00, 0xff, 0xff),
    (0x00, 0x80, 0xff),
    (0x00, 0x00, 0xff),
    (0x80, 0x00, 0xff),
    (0xff, 0x00, 0xff),
    (0xff, 0x00, 0x80),
];

pub const RAINBOW_OF_FIFTHS: [ColorRGB; 12] = [
    (0x80, 0xff, 0x00), // A
    (0xff, 0x00, 0xff), // A#
    (0x00, 0xff, 0x80), // B
    (0xff, 0x00, 0x00), // C
    (0x00, 0x80, 0xff), // C#
    (0xff, 0xff, 0x00), // D
    (0x80, 0x00, 0xff), // D#
    (0x00, 0xff, 0x00), // E
    (0xff, 0x00, 0x80), // F
    (0x00, 0xff, 0xff), // F#
    (0xff, 0x80, 0x00), // G
    (0x00, 0x00, 0xff), // G#
];
