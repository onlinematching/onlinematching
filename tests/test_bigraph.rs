#[cfg(test)]
mod tests_bigraph {
    use std::vec;

    use onlinematching::bigraph::{hopcroft_karp_matching, Bigraph};

    #[test]
    fn bigraph_test() {
        let mut g = Bigraph::from_edges(&vec![
            ("u1", "v1"),
            ("u2", "v1"),
            ("u3", "v1"),
            ("u2", "v2"),
        ]);
        g.insert_v("v2")
            .expect("The online nodes already have this key");
        drop(g);
    }

    #[test]
    fn hopcroft_karp_test() {
        let edges = vec![
            ('a', '1'),
            ('a', '2'),
            ('b', '1'),
            ('b', '3'),
            ('c', '2'),
            ('c', '3'),
            ('d', '3'),
        ];

        let graph = Bigraph::from_edges(&edges);

        println!("{:?}", graph);

        let matching = hopcroft_karp_matching(&graph);
        println!("Maximum matching: {:?}", matching);
    }
}
