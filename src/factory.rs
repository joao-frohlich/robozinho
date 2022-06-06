use crate::tool::ToolType;

#[derive(Clone, Copy)]
pub struct Factory {
    needed_tool: Option<ToolType>,
    quantity: usize,
}

impl Factory {
    pub fn new(needed_tool: Option<ToolType>, quantity: usize) -> Self {
        Self {
            needed_tool,
            quantity,
        }
    }
}
