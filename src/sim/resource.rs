use crate::ant_settings::DEFAULT_RESOURCE_SIZE;

/// A tile that ants will target, as it contains a "useful" resource
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
    /// Used for when an ant "consumes" part of a resource
    ///
    /// Will reduce the resources remaining by one,
    ///
    /// Returns the amount of remaining resources, or None, if is is depleted
    ///
    /// # Example
    /// ```
    /// use Ants::sim::Resource;
    ///
    /// let resource = Resource::default();
    ///
    /// while let Some(amount) = resource.consume(){
    ///     println!("Amount: {}", amount);            
    /// }
    /// ```
    pub(crate) fn consume(&mut self) -> Option<u8> {
        if let Some(resources) = self.resources_remaining.checked_sub(1) {
            self.resources_remaining = resources;
            Some(self.resources_remaining)
        } else {
            None
        }
    }
    /// Returns the percentage amount of resource left, from the default starting amount
    pub fn get_percentage_remaining(&self) -> f64 {
        self.resources_remaining as f64 / DEFAULT_RESOURCE_SIZE as f64
    }
}
