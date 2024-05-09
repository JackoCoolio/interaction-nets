use std::fmt::Debug;

use crate::map::AsHashKey;

pub use self::agent::{Agent, AgentKind};
pub use self::port::Port;

use super::connection::Connection;

mod agent;
mod port;

#[derive(Clone)]
pub enum Term {
    Agent(Agent),
    Port(Port),
}

impl AsHashKey for Term {
    type Key = usize;

    fn as_key(&self) -> Self::Key {
        match self {
            Self::Agent(Agent { id, .. }) => *id,
            Self::Port(Port { id, .. }) => *id,
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
            Self::Port(Port { id, .. }) => id,
        }
    }

    #[inline]
    pub fn connect(self, other: Term) -> Connection {
        Connection(self, other)
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
            Term::Port(Port {
                name: Some(name), ..
            }) => write!(f, "p_{name}"),
            Term::Port(Port { id, .. }) => write!(f, "p_{id}"),
            Term::Agent(agent) => write!(f, "{agent:?}"),
        }
    }
}

impl From<Agent> for Term {
    fn from(value: Agent) -> Self {
        Self::Agent(value)
    }
}

impl From<Port> for Term {
    fn from(value: Port) -> Self {
        Self::Port(value)
    }
}
