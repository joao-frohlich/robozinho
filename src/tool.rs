use bevy::prelude::Component;

#[derive(Clone, Copy)]
pub enum ToolType {
    Battery,
    WeldingArm,
    SuctionPump,
    CoolingDevice,
    PneumaticArm,
}

#[derive(Clone, Copy, Component)]
pub struct Tool {
    pub x: usize,
    pub y: usize,
    pub tool_type: Option<ToolType>,
}

impl Tool {
    pub fn new(x: usize, y: usize, tool_type: Option<ToolType>) -> Self {
        Self {x, y, tool_type}
    }
}