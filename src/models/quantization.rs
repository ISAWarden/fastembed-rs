use serde::{Deserialize, Serialize};

/// Enum for quantization mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantizationMode {
    None,
    Static,
    Dynamic,
}

impl Default for QuantizationMode {
    fn default() -> Self {
        Self::None
    }
}
