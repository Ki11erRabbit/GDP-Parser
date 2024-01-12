use std::fmt::Display;

/// Transforms a value prior to formatting it.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, is_macro::Is)]
#[repr(i8)]
pub enum ConversionFlag {
    /// No conversion
    None = -1, // CPython uses -1
    /// Converts by calling `str(<value>)`.
    Str = b's' as i8,
    /// Converts by calling `ascii(<value>)`.
    Ascii = b'a' as i8,
    /// Converts by calling `repr(<value>)`.
    Repr = b'r' as i8,
}

impl Display for ConversionFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionFlag::None => "-1".fmt(f),
            ConversionFlag::Ascii => "a".fmt(f),
            ConversionFlag::Str => "s".fmt(f),
            ConversionFlag::Repr => "r".fmt(f),
        }
    }
}

impl ConversionFlag {
    pub fn to_byte(&self) -> Option<u8> {
        match self {
            Self::None => None,
            flag => Some(*flag as u8),
        }
    }
    pub fn to_char(&self) -> Option<char> {
        Some(self.to_byte()? as char)
    }
}
