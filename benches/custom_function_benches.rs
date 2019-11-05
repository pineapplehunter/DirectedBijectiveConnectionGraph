#![feature(test)]

extern crate test;
use directed_bijectional_connection_graph::{
    Dims, DirectedBijectiveConnectionGraphFunctions, Lemma1, Lemma2, Node, NodeToNode, NodeToSet,
};
use std::ops::BitXor;
use test::Bencher;

#[bench]
fn custom_function_lemma1_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let d = 0x0000_0000_0000_0000;

    b.iter(|| graph.lemma1(dim, d));
}

#[bench]
fn custom_function_lemma2_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_0000_0000;
    let d = 0xffff_ffff_ffff_ffff;

    b.iter(|| graph.lemma2(s, d));
}

#[bench]
fn custom_function_node_to_set_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_0000_0000;
    let mut d = vec![];

    for i in 0..64 {
        d.push(1 << i);
    }

    b.iter(|| graph.node_to_set(s, &d));
}

#[bench]
fn custom_function_node_to_node_64bit(b: &mut Bencher) {
    let dim = 64;

    let graph = CustomFunctionGraph::new(dim);

    let s = 0x0000_0000_0000_0000;
    let d = 0xffff_ffff_ffff_ffff;
    b.iter(|| graph.node_to_node(s, d));
}

struct CustomFunctionGraph(Dims);

impl CustomFunctionGraph {
    pub fn new(n: Dims) -> Self {
        Self(n)
    }
}

impl DirectedBijectiveConnectionGraphFunctions for CustomFunctionGraph {
    fn phi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask == 0 {
            (1 << (n - 1)).bitxor(node)
        } else {
            (u64::max_value() << (n))
                .bitxor(u64::max_value())
                .bitxor(node)
        }
    }

    fn psi(&self, n: Dims, node: Node) -> Node {
        let mask = 1 << (n - 1);
        if node & mask != 0 {
            (1 << (n - 1)).bitxor(node)
        } else {
            (u64::max_value() << (n))
                .bitxor(u64::max_value())
                .bitxor(node)
        }
    }

    #[inline(always)]
    fn dimension(&self) -> u64 {
        self.0
    }
}