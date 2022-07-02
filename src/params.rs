use crate::tool::ToolType;

pub struct Params {
    pub items_quantity: Vec<(ToolType, usize)>,
    pub factories_needs: Vec<(ToolType, usize)>,
    pub agent_radius: usize,
    pub h_factor: i32,
    pub g_factor: i32,
    pub input_idx: usize,
}

impl Params {
    pub fn new(
        items_quantity: Vec<(ToolType, usize)>,
        factories_needs: Vec<(ToolType, usize)>,
        agent_radius: usize,
        h_factor: i32,
        g_factor: i32,
        input_idx: usize,
    ) -> Self {
        Self {
            items_quantity,
            factories_needs,
            agent_radius,
            h_factor,
            g_factor,
            input_idx,
        }
    }
}
