#[cfg(test)]
mod tests_bigraph {
    use std::vec;

    use onlinematching::bigraph::Bigraph;

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
}
