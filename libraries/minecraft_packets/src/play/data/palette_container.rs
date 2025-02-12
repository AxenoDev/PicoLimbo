use protocol::prelude::*;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct PaletteContainer {
    bits_per_entry: u8, // Should always be 0 for Single valued palette
    palette: VarInt,    // Single valued palette
    /// Present but of length 0 when Bits Per Entry is 0.
    data: LengthPaddedVec<i64>,
}

impl PaletteContainer {
    pub fn void() -> Self {
        Self {
            bits_per_entry: 0,
            palette: VarInt::default(),
            data: Vec::new().into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum PaletteContainerError {
    #[error("error while decoding a palette")]
    EncodeError,
    #[error("invalid palette container error")]
    Infallible,
}

impl From<std::convert::Infallible> for PaletteContainerError {
    fn from(_: std::convert::Infallible) -> Self {
        PaletteContainerError::Infallible
    }
}

impl From<LengthPaddedVecEncodeError> for PaletteContainerError {
    fn from(_: LengthPaddedVecEncodeError) -> Self {
        PaletteContainerError::EncodeError
    }
}

impl EncodePacketField for PaletteContainer {
    type Error = PaletteContainerError;

    fn encode(&self, bytes: &mut Vec<u8>) -> Result<(), Self::Error> {
        self.bits_per_entry.encode(bytes)?;
        self.palette.encode(bytes)?;
        self.data.encode(bytes)?;
        Ok(())
    }
}
