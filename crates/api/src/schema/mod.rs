use const_vec::ConstVec;

use bevy_reflect::TypeInfo;

mod a_mod;
pub use a_mod::Mod;

use crate::ecs::system::Schedule;

#[derive(Debug, Clone, Copy)]
pub struct Schema {
    pub(crate) name: Option<&'static str>,
    pub(crate) types: ConstVec<fn() -> &'static TypeInfo, 1024>,
    pub(crate) resources: ConstVec<(fn() -> &'static TypeInfo, fn() -> Vec<u8>), 128>,
    pub(crate) schedules: ConstVec<(fn() -> &'static TypeInfo, Schedule), 128>,
}

impl Schema {
    pub const fn new() -> Self {
        Self {
            name: None,
            types: ConstVec::new(),
            resources: ConstVec::new(),
            schedules: ConstVec::new(),
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_check_size() {
        // Assume any size over 1MB is too big
        assert!(size_of::<Schema>() < 1024 * 1024);
    }
}
