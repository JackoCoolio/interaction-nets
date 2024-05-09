#![allow(unused)]

use std::rc::Rc;

use map::ConnectionMap;
use net::{connection::Connection, term::Term};

use rule::rulebook::Rulebook;

use crate::{
    net::{
        id::IdAllocator,
        term::{Agent, Port},
    },
    rule::context::RewriteContext,
};

mod ast;
mod map;
mod net;
mod rule;

struct Symbol {
    ident: String,
    ref_count: u16,
}

struct Binding {
    symbol: Rc<Symbol>,
    outer: Option<Rc<Binding>>,
}

impl Binding {
    pub fn find_symbol(&self, ident: &str) -> Option<Rc<Symbol>> {
        if self.symbol.ident == ident {
            return Some(Rc::clone(&self.symbol));
        }

        let outer = Option::as_ref(&self.outer)?;

        outer.find_symbol(ident)
    }
}

enum Expression {
    Application {
        function: Box<Expression>,
        argument: Box<Expression>,
    },
    Lambda {
        arg: String,
        body: Box<Expression>,
    },
}

struct Application {
    function: Box<Expression>,
    argument: Box<Expression>,
}

impl Expression {}

fn main() {
    let id_alloc = IdAllocator::new();

    let a_port = Port::new(id_alloc.create_id());
    let a = Agent::new_constructor(id_alloc.create_id(), a_port.clone(), a_port);

    let c_port = Port::new(id_alloc.create_id());
    let c = Agent::new_constructor(id_alloc.create_id(), c_port.clone(), c_port);

    let ed_port = Port::new(id_alloc.create_id());
    let out_port = Port::new(id_alloc.create_id());
    let e = Agent::new_constructor(id_alloc.create_id(), ed_port.clone(), out_port);

    let d = Agent::new_constructor(id_alloc.create_id(), e, ed_port);

    let b = Agent::new_constructor(id_alloc.create_id(), c, d);

    let connections = vec![Term::from(a).connect(b.into())];

    let runtime = Runtime::new(connections, Rulebook::default(), id_alloc);

    let net: Vec<_> = runtime.normalize().into_iter().collect();

    println!("Connections ({}):\n", net.len());
    for connection in net {
        println!("{:?}", connection);
    }
}

enum Action {
    Reduce(usize),
}

struct Runtime {
    // connections should only be Port=Agent, or Agent=Agent; never Agent=Port.
    // we should also be able to avoid Port=Port
    connections: ConnectionMap<Term, Term>,
    action_stack: Vec<Action>,
    rulebook: Rulebook,
    ctx: RewriteContext,
}

impl Runtime {
    pub fn new(
        connections: impl IntoIterator<Item = Connection>,
        rulebook: Rulebook,
        id_alloc: IdAllocator,
    ) -> Self {
        let mut runtime = Self {
            connections: ConnectionMap::<_, _>::new(),
            action_stack: Vec::new(),
            rulebook,
            ctx: RewriteContext { id_alloc },
        };

        for Connection(left, right) in connections {
            runtime.push_connection(left, right);
        }

        runtime
    }

    fn push_connection(&mut self, a: Term, b: Term) {
        match (&a, &b) {
            (Term::Agent(_), Term::Agent(_)) => {
                self.action_stack.push(Action::Reduce(*a.id()));
                self.connections.insert(a, b);
            }
            (Term::Port { .. }, Term::Port { .. }) => {
                // TODO: reduce
                self.connections.insert(a, b);
            }
            (Term::Port { .. }, Term::Agent(_)) => {
                self.connections.insert(a, b);
            }
            (Term::Agent(_), Term::Port { .. }) => self.push_connection(b, a),
        };
    }

    pub fn normalize(mut self) -> impl IntoIterator<Item = (Term, Term)> {
        while let Some(action) = self.action_stack.pop() {
            match action {
                Action::Reduce(id) => {
                    let (left, right) = self
                        .connections
                        .remove_by_left_key(&id)
                        .expect("invalid runtime state: action stack had invalid term ID");

                    let Term::Agent(left) = left else {
                        panic!("invalid runtime state: reduce action pointed to a port");
                    };

                    let Term::Agent(right) = right else {
                        panic!("invalid runtime state: reduce action pointed to a port");
                    };

                    let result = self.rulebook.rewrite(&self.ctx, left, right);

                    for Connection(left, right) in result.new_connections {
                        self.push_connection(left, right);
                    }
                }
            }
        }

        self.connections
    }
}
