use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SwapVersion {
    pub version: u32,
}

impl Default for SwapVersion {
    fn default() -> Self {
        Self {
            version: legacy_swap_version(),
        }
    }
}

impl SwapVersion {
    pub(crate) fn is_legacy(&self) -> bool {
        self.version == legacy_swap_version()
    }
}

impl From<u32> for SwapVersion {
    fn from(version: u32) -> Self {
        Self { version }
    }
}

pub const fn legacy_swap_version() -> u32 {
    1
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TakerRequest {
    pub base: String,
    pub rel: String,
    #[serde(default)] // Ensures SwapVersion::default() is used when deserializing
    #[serde(skip_serializing_if = "SwapVersion::is_legacy")]
    pub swap_version: SwapVersion,
}
