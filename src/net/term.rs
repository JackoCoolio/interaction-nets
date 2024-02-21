use std::fmt::Debug;

use crate::map::AsHashKey;

use super::connection::Connection;

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
    pub id: usize,
    pub kind: AgentKind,
    pub ports: Box<[Term]>,
}

impl Agent {
    pub fn ports_array<const N: usize>(self) -> Result<[Term; N], usize> {
        let array_boxed: Box<[Term; N]> =
            self.ports.try_into().map_err(|b: Box<[Term]>| b.len())?;
        Ok(*array_boxed)
    }

    pub fn new_eraser(id: usize) -> Self {
        Self {
            id,
            kind: AgentKind::Eraser,
            ports: [].into(),
        }
    }

    pub fn new_duplicator(id: usize, port_a: Term, port_b: Term) -> Self {
        Self::new_2_arity(id, AgentKind::Duplicator, port_a, port_b)
    }

    pub fn new_constructor(id: usize, port_a: Term, port_b: Term) -> Self {
        Self::new_2_arity(id, AgentKind::Constructor, port_a, port_b)
    }

    fn new_2_arity(id: usize, kind: AgentKind, port_a: Term, port_b: Term) -> Self {
        Self {
            id,
            kind,
            ports: [port_a, port_b].into(),
        }
    }
}

impl Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}_{}(", self.kind, self.id)?;

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

#[derive(Clone)]
pub enum Term {
    Agent(Agent),
    Port { id: usize },
}

impl AsHashKey for Term {
    type Key = usize;

    fn as_key(&self) -> Self::Key {
        match self {
            Self::Agent(Agent { id, .. }) => *id,
            Self::Port { id } => *id,
        }
    }
}

impl std::borrow::Borrow<usize> for Term {
    fn borrow(&self) -> &usize {
        self.id()
    }
}

impl Term {
    pub fn id(&self) -> &usize {
        match self {
            Self::Agent(Agent { id, .. }) => id,
            Self::Port { id } => id,
        }
    }

    #[inline]
    pub fn connect(self, other: Term) -> Connection {
        Connection(self, other)
    }

    #[inline]
    pub fn new_port(id: usize) -> Term {
        Self::Port { id }
    }
}

impl std::hash::Hash for Term {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(*self.id());
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Term {}
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Port { id } => write!(f, "p_{id}"),
            Term::Agent(agent) => write!(f, "{agent:?}"),
        }
    }
}

impl From<Agent> for Term {
    fn from(value: Agent) -> Self {
        Self::Agent(value)
    }
}
