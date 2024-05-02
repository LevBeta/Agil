/// DEfines the type of [`Instrument`]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum InstrumentKind {
    Spot,
    Perpetual,
}

impl Default for InstrumentKind {
    fn default() -> Self {
        Self::Spot
    }
}
