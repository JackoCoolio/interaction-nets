use crate::net::{
    id::IdAllocator,
    term::{Agent, AgentKind, Term},
};

pub struct RewriteContext {
    pub id_alloc: IdAllocator,
}

impl RewriteContext {
    pub fn create_port(&self) -> Term {
        Term::Port {
            id: self.id_alloc.create_id(),
        }
    }

    #[inline]
    pub fn create_wire(&self) -> (Term, Term) {
        (self.create_port(), self.create_port())
    }

    pub fn create_agent(&self, kind: AgentKind, ports: &[Term]) -> Term {
        Term::Agent(Agent {
            id: self.id_alloc.create_id(),
            kind,
            ports: ports.into(),
        })
    }
}
