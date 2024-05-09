use std::fmt::Debug;

use super::Term;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgentKind {
    Eraser,
    Duplicator,
    Constructor,
    Dynamic(usize),
}

impl Debug for AgentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eraser => write!(f, "Eraser"),
            Self::Duplicator => write!(f, "Duplicator"),
            Self::Constructor => write!(f, "Constructor"),
            Self::Dynamic(id) => write!(f, "Dynamic[{id}]"),
        }
    }
}

#[derive(Clone)]
pub struct Agent {
    name: Option<String>,
    pub id: usize,
    pub kind: AgentKind,
    pub ports: Box<[Term]>,
}

impl Agent {
    /// Annotates the `Agent` with a `name`.
    pub fn name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
    }

    pub fn ports_array<const N: usize>(self) -> Result<[Term; N], usize> {
        let array_boxed: Box<[Term; N]> =
            self.ports.try_into().map_err(|b: Box<[Term]>| b.len())?;
        Ok(*array_boxed)
    }

    pub fn new(id: usize, kind: AgentKind, ports: impl Into<Box<[Term]>>) -> Self {
        Self {
            name: None,
            id,
            kind,
            ports: ports.into(),
        }
    }

    pub fn new_eraser(id: usize) -> Self {
        Self {
            name: None,
            id,
            kind: AgentKind::Eraser,
            ports: [].into(),
        }
    }

    pub fn new_duplicator(id: usize, port_a: impl Into<Term>, port_b: impl Into<Term>) -> Self {
        Self::new_2_arity(id, AgentKind::Duplicator, port_a.into(), port_b.into())
    }

    pub fn new_constructor(id: usize, port_a: impl Into<Term>, port_b: impl Into<Term>) -> Self {
        Self::new_2_arity(id, AgentKind::Constructor, port_a.into(), port_b.into())
    }

    fn new_2_arity(
        id: usize,
        kind: AgentKind,
        port_a: impl Into<Term>,
        port_b: impl Into<Term>,
    ) -> Self {
        Self {
            name: None,
            id,
            kind,
            ports: [port_a.into(), port_b.into()].into(),
        }
    }
}

impl Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}_", self.kind)?;

        if let Some(name) = &self.name {
            write!(f, "{}(", name)?;
        } else {
            write!(f, "{}(", self.id)?;
        }

        let mut ports_iter = self.ports.iter();
        if let Some(port) = ports_iter.next() {
            write!(f, "{port:?}")?;

            for port in ports_iter {
                write!(f, ", {port:?}")?;
            }
        }

        write!(f, ")")
    }
}
