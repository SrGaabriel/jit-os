use alloc::string::String;

use crate::module::ModuleId;

pub struct Unique {
    pub id: usize,
    pub module_id: ModuleId,
    pub display_name: Option<String>,
}

impl Unique {
    pub fn new(id: usize, module_id: ModuleId, display_name: Option<String>) -> Self {
        Self {
            id,
            module_id,
            display_name,
        }
    }
    
    pub fn unnamed(id: usize, module_id: ModuleId) -> Self {
        Self {
            id,
            module_id,
            display_name: None,
        }
    }
}
