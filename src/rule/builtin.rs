use crate::net::{
    connection::Connection,
    term::{Agent, AgentKind, Term},
};

use super::{context::RewriteContext, rulebook::ActivePairPattern, RewriteResult};

pub(super) enum Builtin {
    EraEra,
    CtrCtr,
    DupDup,
    DupEra,
    CtrEra,
    CtrDup,
}

impl Builtin {
    pub fn all() -> Vec<Self> {
        use Builtin::*;

        vec![EraEra, CtrCtr, DupDup, DupEra, CtrEra, CtrDup]
    }

    pub fn pattern(&self) -> ActivePairPattern {
        use AgentKind::*;
        use Builtin::*;

        match self {
            EraEra => ActivePairPattern::new(Eraser, Eraser),
            CtrCtr => ActivePairPattern::new(Constructor, Constructor),
            DupDup => ActivePairPattern::new(Duplicator, Duplicator),
            DupEra => ActivePairPattern::new(Duplicator, Eraser),
            CtrEra => ActivePairPattern::new(Constructor, Eraser),
            CtrDup => ActivePairPattern::new(Constructor, Duplicator),
        }
    }

    #[inline] // inline because it's only used in one place
    pub fn rewrite(&self, ctx: &RewriteContext, a: Agent, b: Agent) -> RewriteResult {
        /*
         * Agents passed into this function are sorted by their AgentKind.
         */

        match self {
            Self::EraEra => {
                ctx.id_alloc.retire_id(a.id);
                ctx.id_alloc.retire_id(b.id);

                RewriteResult::empty()
            }
            Self::CtrCtr | Self::DupDup => {
                ctx.id_alloc.retire_id(a.id);
                ctx.id_alloc.retire_id(b.id);

                let [a0, a1] = a.ports_array().unwrap();
                let [b0, b1] = b.ports_array().unwrap();

                RewriteResult {
                    new_connections: vec![Connection(a0, b0), Connection(a1, b1)],
                }
            }
            Self::CtrDup => {
                let ctr = a;
                let dup = b;
                assert!(ctr.kind == AgentKind::Constructor);
                assert!(ctr.ports.len() == 2);
                assert!(dup.kind == AgentKind::Duplicator);
                assert!(dup.ports.len() == 2);

                let [ctr_a_in, ctr_b_in] = ctr.ports_array().unwrap();
                let [dup_a_in, dup_b_in] = dup.ports_array().unwrap();

                // NOTE THE SWAP!
                let ctr_a_out = dup_a_in;
                let ctr_b_out = dup_b_in;
                let dup_a_out = ctr_a_in;
                let dup_b_out = ctr_b_in;

                let w0 @ (ctr_a_dup_b_ctr, ctr_a_dup_b_dup) = &ctx.create_wire();
                let w1 @ (ctr_a_dup_a_ctr, ctr_a_dup_a_dup) = &ctx.create_wire();
                let w2 @ (ctr_b_dup_b_ctr, ctr_b_dup_b_dup) = &ctx.create_wire();
                let w3 @ (ctr_b_dup_a_ctr, ctr_b_dup_a_dup) = &ctx.create_wire();

                let mut new_connections: Vec<_> = [w0, w1, w2, w3]
                    .into_iter()
                    .map(Clone::clone)
                    .map(Into::into)
                    .collect();

                // agents
                let ctr_a = Agent::new_constructor(
                    ctx.id_alloc.create_id(),
                    ctr_a_dup_a_ctr.clone(),
                    ctr_a_dup_b_ctr.clone(),
                );

                let ctr_b = Agent::new_constructor(
                    ctx.id_alloc.create_id(),
                    ctr_b_dup_a_ctr.clone(),
                    ctr_b_dup_b_ctr.clone(),
                );

                let dup_a = Agent::new_duplicator(
                    ctx.id_alloc.create_id(),
                    ctr_a_dup_a_dup.clone(),
                    ctr_b_dup_a_dup.clone(),
                );

                let dup_b = Agent::new_duplicator(
                    ctx.id_alloc.create_id(),
                    ctr_a_dup_b_dup.clone(),
                    ctr_b_dup_b_dup.clone(),
                );

                new_connections.push(Term::Agent(ctr_a).connect(ctr_a_out));
                new_connections.push(Term::Agent(ctr_b).connect(ctr_b_out));
                new_connections.push(Term::Agent(dup_a).connect(dup_a_out));
                new_connections.push(Term::Agent(dup_b).connect(dup_b_out));

                RewriteResult { new_connections }
            }
            Self::DupEra | Self::CtrEra => handle_dup_or_ctr_to_era(a, b),
        }
    }
}

fn handle_dup_or_ctr_to_era(dup_or_ctr: Agent, era: Agent) -> RewriteResult {
    assert!(dup_or_ctr.ports.len() == 2);
    assert!(era.kind == AgentKind::Eraser);
    assert!(era.ports.len() == 0);

    // we can reuse these IDs
    let era_a_id = dup_or_ctr.id;
    let era_b_id = era.id;

    let [a, b] = dup_or_ctr.ports_array().unwrap();

    let era_a = Agent::new_eraser(era_a_id);
    let era_b = Agent::new_eraser(era_b_id);

    let new_connections = vec![Term::Agent(era_a).connect(a), Term::Agent(era_b).connect(b)];

    RewriteResult { new_connections }
}
