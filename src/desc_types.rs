use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiChannel {
    Global,
    Home,
    Hybrid,
    Local,
    Main,
    Channel { channel_id: String },
    Antenna { antenna_id: String },
}
