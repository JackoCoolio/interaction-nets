#![allow(unused)]

use std::rc::Rc;

use map::ConnectionMap;
use net::{connection::Connection, term::Term};

use rule::rulebook::Rulebook;

use crate::{
    net::{
        id::IdAllocator,
        term::{Agent, AgentKind, Port},
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
    use AgentKind::*;

    let ctx = RewriteContext::new(IdAllocator::new());

    let a_port = ctx.create_port().with_name("a_port");
    let a = ctx
        .create_agent(Constructor, &[a_port.clone(), a_port])
        .with_name("a");

    let c_port = ctx.create_port().with_name("c_port");
    let c = ctx
        .create_agent(Constructor, &[c_port.clone(), c_port])
        .with_name("c");

    let ed_port = ctx.create_port().with_name("ed_port");
    let out_port = ctx.create_port().with_name("out_port");
    let e = ctx
        .create_agent(Constructor, &[ed_port.clone(), out_port])
        .with_name("e");

    let d = ctx.create_agent(Constructor, &[e, ed_port]).with_name("d");

    let b = ctx.create_agent(Constructor, &[c, d]).with_name("b");

    let connections = vec![a.connect(b)];

    let runtime = Runtime::new(connections, Rulebook::default(), ctx);

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
        ctx: RewriteContext,
    ) -> Self {
        let mut runtime = Self {
            connections: ConnectionMap::<_, _>::new(),
            action_stack: Vec::new(),
            rulebook,
            ctx,
        };

        for Connection(left, right) in connections {
            runtime.push_connection(left, right);
        }

        runtime
    }

    fn push_connection(&mut self, left: Term, right: Term) {
        println!("current map state:");
        self.connections.dump();
        println!("pushing {:?} <> {:?}", left, right);
        match (left, right) {
            (left @ Term::Agent(_), right @ Term::Agent(_)) => {
                println!("\t* agent<>agent, pushing reduce to stack");
                self.action_stack.push(Action::Reduce(*left.id()));
                self.connections.insert(left, right).unwrap();
            }
            (left @ Term::Port { .. }, right @ Term::Port { .. }) => {
                // TODO: reduce
                println!(
                    "\t* port<>port, checking if {:?} or {:?} are already in map",
                    left, right
                );

                if let Some((_, other_right)) = self.connections.remove_by_left_key(left.id()) {
                    println!(
                        "\t\t* port already exists, connecting {:?}<>{:?}",
                        right, other_right
                    );
                    return self.push_connection(right, other_right);
                }

                if let Some((_, other_right)) = self.connections.remove_by_left_key(right.id()) {
                    println!(
                        "\t\t* port already exists, connecting {:?}<>{:?}",
                        left, other_right
                    );
                    return self.push_connection(left, other_right);
                }

                if let Some((other_left, _)) = self.connections.remove_by_right_key(right.id()) {
                    println!(
                        "\t\t* port already exists, connecting {:?}<>{:?}",
                        left, other_left
                    );
                    return self.push_connection(left, other_left);
                }

                if let Some((other_left, _)) = self.connections.remove_by_right_key(left.id()) {
                    println!(
                        "\t\t* port already exists, connecting {:?}<>{:?}",
                        left, other_left
                    );
                    return self.push_connection(right, other_left);
                }

                self.connections.insert(left, right).unwrap();
            }
            (left @ Term::Port { .. }, right @ Term::Agent(_)) => {
                println!("\t* port<>agent, checking for existing port");
                if let Some((_, other_right)) = self.connections.remove_by_left(&left) {
                    println!(
                        "\t\t* port already exists, connecting {:?}<>{:?}",
                        right, other_right
                    );
                    // we can just connect the two together :)
                    self.push_connection(right, other_right)
                } else {
                    self.connections.insert(left, right).unwrap();
                }
            }
            (left @ Term::Agent(_), right @ Term::Port { .. }) => self.push_connection(right, left),
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

                    println!("reducing {:?} <> {:?}", left, right);

                    let Term::Agent(left) = left else {
                        panic!("invalid runtime state: reduce action pointed to a port");
                    };

                    let Term::Agent(right) = right else {
                        panic!("invalid runtime state: reduce action pointed to a port");
                    };

                    let result = self.rulebook.rewrite(&self.ctx, left, right);
                    println!("* resulting connections:");

                    for Connection(left, right) in result.new_connections {
                        println!("    * {:?} <> {:?}", left, right);
                        self.push_connection(left, right);
                    }
                }
            }
        }

        self.connections
    }
}
