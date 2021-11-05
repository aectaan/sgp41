#[derive(Debug, Copy, Clone)]
/// List of SGP41 sensor commands.
pub enum Command {
    ExecuteConditioning,
    MeasureRawSignals,
    ExecuteSelfTest,
    TurnHeaterOff,
    GetSerialNumber,
    SoftReset,
}

impl Command {
    /// Returns command hex code and measurement delay in ms
    pub fn as_tuple(self) -> (u16, u32) {
        match self {
            Self::ExecuteConditioning => (0x2612, 50),
            Self::MeasureRawSignals => (0x2619, 50),
            Self::ExecuteSelfTest => (0x280E, 320),
            Self::TurnHeaterOff => (0x3615, 1000),
            Self::GetSerialNumber => (0x3682, 1000),
            Self::SoftReset => (0x0006, 1000),
        }
    }
}
