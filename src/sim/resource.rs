use crate::ant_settings::DEFAULT_RESOURCE_SIZE;

#[derive(Copy, Clone)]
pub struct Resource {
    resources_remaining: u8,
}
impl Default for Resource {
    fn default() -> Self {
        Resource {
            resources_remaining: DEFAULT_RESOURCE_SIZE,
        }
    }
}
impl Resource {
    pub(crate) fn consume(&mut self) -> Option<()> {
        if let Some(resources) = self.resources_remaining.checked_sub(1) {
            self.resources_remaining = resources;
            Some(())
        } else {
            None
        }
    }
    pub fn get_percentage_remaining(&self) -> f64 {
        self.resources_remaining as f64 / DEFAULT_RESOURCE_SIZE as f64
    }
}
