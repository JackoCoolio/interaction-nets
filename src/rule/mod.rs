mod builtin;
pub mod context;
pub mod rulebook;

use self::context::RewriteContext;

use super::net::{connection::Connection, term::Agent};
use builtin::Builtin;

pub struct RewriteResult {
    pub new_connections: Vec<Connection>,
}

impl RewriteResult {
    pub fn empty() -> Self {
        Self {
            new_connections: Vec::new(),
        }
    }
}

impl From<Vec<Connection>> for RewriteResult {
    fn from(new_connections: Vec<Connection>) -> Self {
        Self { new_connections }
    }
}

type RewriteRule = dyn Fn(&RewriteContext, Agent, Agent) -> RewriteResult;

pub enum Rule {
    Builtin(Builtin),
    Dynamic(Box<RewriteRule>),
}

impl Rule {
    pub fn rewrite(&self, ctx: &RewriteContext, a: Agent, b: Agent) -> RewriteResult {
        match self {
            Self::Builtin(builtin) => builtin.rewrite(ctx, a, b),
            Self::Dynamic(f) => f(ctx, a, b),
        }
    }
}

//     fn rewrite(&self, left: Agent, right: Agent) -> RewriteResult;
