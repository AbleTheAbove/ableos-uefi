pub struct Cpu {}

pub enum DisplayType {
    Standard,
    VRHeadset,
}

pub struct Hardware {
    display: DisplayType,
}
