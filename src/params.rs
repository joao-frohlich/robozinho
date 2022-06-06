use crate::tool::ToolType;

pub struct Params {
    pub items_quantity: Vec<(ToolType, usize)>,
}

impl Params {
    pub fn new(items_quantity: Vec<(ToolType, usize)>) -> Self {
        Self { items_quantity }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(vec![
            (ToolType::Battery, 20),
            (ToolType::WeldingArm, 10),
            (ToolType::SuctionPump, 8),
            (ToolType::CoolingDevice, 6),
            (ToolType::PneumaticArm, 4),
        ])
    }
}
