#[derive(Debug, Clone)]
pub struct Port {
    pub name: Option<String>,
    pub(super) id: usize,
}

impl Port {
    /// Creates a new `Port`.
    pub fn new(id: usize) -> Port {
        Self { name: None, id }
    }
}
