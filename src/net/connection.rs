use std::fmt::Debug;

use super::term::{Agent, Term};

pub struct Connection(pub Term, pub Term);

impl Connection {
    #[inline]
    pub fn left(&self) -> &Term {
        &self.0
    }

    #[inline]
    pub fn right(&self) -> &Term {
        &self.1
    }

    #[inline]
    pub fn id(&self) -> usize {
        *self.left().id()
    }

    pub fn from_agents(left: Agent, right: Agent) -> Self {
        Self(Term::Agent(left), Term::Agent(right))
    }

    pub fn is_active_pair(&self) -> bool {
        matches!(self, Connection(Term::Agent(_), Term::Agent(_)))
    }
}

impl From<(Term, Term)> for Connection {
    fn from(value: (Term, Term)) -> Self {
        Self(value.0, value.1)
    }
}

impl Eq for Connection {}
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.left() == other.left() && self.right() == other.right())
            || (self.left() == other.right() && self.right() == other.left())
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = {:?}", self.left(), self.right())
    }
}
