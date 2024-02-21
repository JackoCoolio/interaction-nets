use std::collections::BTreeMap;

use super::{builtin::Builtin, context::RewriteContext, RewriteResult, Rule};
use crate::net::{
    connection::Connection,
    term::{Agent, AgentKind},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActivePairPattern(AgentKind, AgentKind);

impl ActivePairPattern {
    pub fn new(a: AgentKind, b: AgentKind) -> Self {
        if a < b {
            Self(a, b)
        } else {
            Self(b, a)
        }
    }

    pub fn from_agents(a: &Agent, b: &Agent) -> Self {
        let a = a.kind;
        let b = b.kind;

        Self::new(a, b)
    }

    #[inline]
    pub fn pattern(&self) -> (&AgentKind, &AgentKind) {
        (&self.0, &self.1)
    }
}

pub struct Rulebook {
    /// Map from agent kinds to rewrite rule.
    map: BTreeMap<ActivePairPattern, Rule>,
}

impl Rulebook {
    pub fn add_rule(&mut self, pattern: ActivePairPattern, rule: Rule) -> &mut Self {
        self.map.insert(pattern, rule);

        self
    }

    pub fn rewrite(&self, ctx: &RewriteContext, left: Agent, right: Agent) -> RewriteResult {
        let Some(rule) = self.map.get(&ActivePairPattern::from_agents(&left, &right)) else {
            eprintln!(
                "warn: no rewrite rule for {:?} <-> {:?}",
                left.kind, right.kind
            );

            return vec![Connection::from_agents(left, right)].into();
        };

        rule.rewrite(ctx, left, right)
    }
}

impl Default for Rulebook {
    fn default() -> Self {
        let rules = BTreeMap::<_, _>::from_iter(
            Builtin::all()
                .into_iter()
                .map(|builtin| (builtin.pattern(), Rule::Builtin(builtin))),
        );

        Self { map: rules }
    }
}
