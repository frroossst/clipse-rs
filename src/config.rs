use serde::{Deserialize, Serialize};

use crate::clipboard::ClipBoard;


#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClipConfig {
    pub clipboard: ClipBoard,
}
