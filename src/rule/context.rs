use crate::net::{
    id::IdAllocator,
    term::{Agent, AgentKind, Port, Term},
};

pub struct RewriteContext {
    pub id_alloc: IdAllocator,
}

impl RewriteContext {
    pub fn new(id_alloc: IdAllocator) -> Self {
        Self { id_alloc }
    }

    pub fn create_port(&self) -> Term {
        Term::Port(Port::new(self.id_alloc.create_id()))
    }

    #[inline]
    pub fn create_wire(&self) -> (Term, Term) {
        (self.create_port(), self.create_port())
    }

    pub fn create_agent(&self, kind: AgentKind, ports: &[Term]) -> Term {
        Term::Agent(Agent::new(self.id_alloc.create_id(), kind, ports))
    }
}
