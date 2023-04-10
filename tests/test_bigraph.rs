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

    #[test]
    fn weighted_greddy_test() {
        let n = 5;
        let g = onlinematching::papers::adwords::greedy::example::greedy_worst_case(n);
        let opt = (0.99 + 1.) * n as f64;
        let alg = g.ALG::<onlinematching::papers::adwords::greedy::Greddy<f64>>();
        println!("{:?}", g);
        println!("opt = {:?}, alg = {:?}", opt, alg);
    }

    #[test]
    fn thick_triangle_case_test() {
        let n = 3;
        let m = 4;
        let g = onlinematching::papers::adwords::msvv05::example::thick_triangle_case(m, n);
        let opt = n as f64 * m as f64;
        let alg = g.ALG::<onlinematching::papers::adwords::greedy::Greddy<i32>>();
        println!("----------------------");
        println!("{:?}", g);
        println!("----------------------");
        println!("opt = {:?}, alg = {:?}", opt, alg);
    }
}
