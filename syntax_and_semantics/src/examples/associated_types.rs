#![allow(unused_variables)]

pub fn associated_types() {
    println!("Associated Types\n");

    // Associated types are a powerful part of Rust's type system. They provide
    // a mechanism for grouping mutliple types together.

    // For example, here is a graph trait that uses associated types to state
    // that a graph is composed of a certain Node and Edge type.

    trait Graph {
        type N;
        type E;

        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
    }

    // Associated types are declared are declared within the body of the trait
    // using the type keyword.

    // These type declarations work in the same way as those for functions. For
    // example if we wanted our N type to implement Display, so we can print
    // the nodes out, the type declaration could be modified as follows.
    //
    //  type N: fmt::Display;

    // Implementing associated types.

    // Just like any trait, traits with associated types use the impl keyword
    // to provide implementations. Here is a simple implementation of Graph.

    struct Node;

    struct Edge;

    struct MyGraph;

    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            true
        }

        fn edges(&self, n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }

    // Trait objects with associated types.

    // When creating a trait object we must provide concrete types for the
    // associated types declared within the trait.

    // For example we can provide the Node and Edge concrete types as follows.
    let graph = MyGraph;

    let obj = Box::new(graph) as Box<Graph<N=Node, E=Edge>>;

    // Without the concrete types the compiler is unable to determine which
    // impl should be used.

    println!();
}
