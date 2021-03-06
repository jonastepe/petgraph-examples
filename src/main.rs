extern crate petgraph;

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;
use std::error::Error;

use petgraph::visit::{IntoNodeIdentifiers, IntoEdgeReferences, NodeCompactIndexable, EdgeRef};
use petgraph::algo::FloatMeasure;
use petgraph::graph::DiGraph;

fn main() {
    let mut g = DiGraph::<&str, f32>::new();

    let v0 = g.add_node("v0");
    let v1 = g.add_node("v1");
    let v2 = g.add_node("v2");
    let v3 = g.add_node("v3");

    g.extend_with_edges(&[(v0, v1, 2.2), (v1, v2, 2.4), (v2, v0, -5.3), (v3, v2, -1.0)]);

    match bellman_ford(&g, v0) {
        Ok(lengths) => {
            println!("δ(v2) = {:.1}", lengths.get(&v2).unwrap());
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn bellman_ford<G>
    (g: G,
     s: G::NodeId)
     -> Result<HashMap<G::NodeId, G::EdgeWeight>, CycleError<(G::NodeId, G::NodeId)>>
    where G: IntoNodeIdentifiers + NodeCompactIndexable + IntoEdgeReferences,
          G::EdgeWeight: FloatMeasure,
          G::NodeId: Eq + Hash
{
    let mut lengths: HashMap<G::NodeId, G::EdgeWeight> = g.node_identifiers()
        .map(|n| (n, G::EdgeWeight::infinite()))
        .collect();

    lengths.insert(s, G::EdgeWeight::zero());

    for _ in g.node_identifiers() {

        for e in g.edge_references() {
            match lengths.get(&e.target()) {
                Some(&target_len) => {
                    if let Some(&source_len) = lengths.get(&e.source()) {
                        if target_len > source_len + *e.weight() {
                            lengths.insert(e.target(), source_len + *e.weight());
                        }
                    }
                }
                None => {}
            }
        }
    }

    for e in g.edge_references() {
        match lengths.get(&e.target()) {
            Some(&target_len) => {
                if let Some(&source_len) = lengths.get(&e.source()) {
                    if target_len > source_len + *e.weight() {
                        return Err(CycleError((e.source(), e.target())));
                    }
                }
            }
            None => {}
        }
    }

    Ok(lengths)
}

#[derive(Debug)]
struct CycleError<E>(E);

impl<E: fmt::Debug> Error for CycleError<E> {
    fn description(&self) -> &str {
        "The graph contains a negative cycle."
    }
}

impl<E: fmt::Debug> fmt::Display for CycleError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A negative cycle detected at {:?}.", self.0)
    }
}
