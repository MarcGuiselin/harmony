/// Similar to bevy's Component
pub trait Component
where
    Self: common::HasStableId + bitcode::Encode + Decode,
{
    fn get_local_component_id() -> u32;
}

pub trait Decode
where
    Self: for<'a> bitcode::Decode<'a>,
{
}
impl<T> Decode for T where T: for<'a> bitcode::Decode<'a> {}
