#[cfg(test)]
mod tests_bigraph {
    use std::vec;

    use onlinematching::bigraph::Bigraph;
    use onlinematching::weightedbigraph::WBigraph;

    #[test]
    fn bigraph_test() {
        let mut g = Bigraph::from_edges(&vec![
            ("u1", "v1"),
            ("u2", "v1"),
            ("u3", "v1"),
            ("u2", "v2"),
        ]);
        println!("{:?}", g);
        g.insert_v("v2")
            .expect_err("The online nodes already have this key");
        drop(g);
    }

    #[test]
    fn weighted_bigraph_test() {
        let mut g = WBigraph::from_edges(&vec![
            (("u1", "v1"), 1),
            (("u2", "v1"), 2),
            (("u3", "v1"), 3),
            (("u2", "v2"), 4),
        ]);
        println!("{:?}", g);
        g.insert_v("v3")
            .expect("The online nodes already have this key");

        println!("{:?}", g);
        drop(g);
    }
}
