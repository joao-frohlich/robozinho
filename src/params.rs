use crate::tool::ToolType;

pub struct Params {
    pub items_quantity: Vec<(ToolType, usize)>,
    pub factories_needs: Vec<(ToolType, usize)>,
    pub agent_radius: usize,
}

impl Params {
    pub fn new(
        items_quantity: Vec<(ToolType, usize)>,
        factories_needs: Vec<(ToolType, usize)>,
        agent_radius: usize,
    ) -> Self {
        Self {
            items_quantity,
            factories_needs,
            agent_radius,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self::new(
            vec![
                (ToolType::Battery, 20),
                (ToolType::WeldingArm, 10),
                (ToolType::SuctionPump, 8),
                (ToolType::CoolingDevice, 6),
                (ToolType::PneumaticArm, 4),
            ],
            vec![
                (ToolType::Battery, 8),
                (ToolType::WeldingArm, 5),
                (ToolType::SuctionPump, 2),
                (ToolType::CoolingDevice, 5),
                (ToolType::PneumaticArm, 2),
            ],
            4,
        )
    }
}
