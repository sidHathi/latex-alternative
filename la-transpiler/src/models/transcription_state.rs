use crate::enums::TranscriptionMode;

#[derive(Debug)]
pub struct TranscriptionState {
    pub prev_modes: Vec<TranscriptionMode>,
    pub mode: TranscriptionMode,
    pub indentation: u8,
    pub paren_depth: i8,
}