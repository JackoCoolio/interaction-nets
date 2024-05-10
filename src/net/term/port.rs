use std::fmt::Debug;

#[derive(Clone)]
pub struct Port {
    pub name: Option<String>,
    pub(super) id: usize,
}

impl Port {
    /// Creates a new `Port`.
    pub fn new(id: usize) -> Port {
        Self { name: None, id }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Debug for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self {
                id,
                name: Some(name),
            } => write!(f, "p<{},{}>", id, name),
            Self { id, .. } => write!(f, "p_<{}>", id),
        }
    }
}
