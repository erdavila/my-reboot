use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::win_api::types;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DeviceId {
    pub adapterId: types::LUID,
    pub id: u32,
}

impl From<(types::LUID, u32)> for DeviceId {
    fn from((adapterId, id): (types::LUID, u32)) -> Self {
        DeviceId { adapterId, id }
    }
}

impl Hash for DeviceId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.adapterId.hash(state);
        self.id.hash(state);
    }
}
