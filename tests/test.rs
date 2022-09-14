#[cfg(test)]
mod tests {

    #[test]
    fn bigraph_test() {
        use onlinematching::bigraph;
        let mut g = bigraph::Bigraph::from_edges(&vec![
            ("u1", "v1"),
            ("u2", "v1"),
            ("u3", "v1"),
            ("u2", "v2"),
        ]);
        g.insert_online("v2").expect("");
        drop(g);
    }

    #[test]
    fn bigraph_random_alg_test() {
        let graph = onlinematching::papers::kvv90::example::random_worst_case(200);
        type Random = onlinematching::papers::kvv90::algorithm::Random;
        let opt = graph.OPT();
        let alg = graph.ALG::<Random>();
        let ratio = alg / opt;
        println!("the ratio is {:?}", ratio);
        println!("alg: {:?}, opt: {:?}", alg, opt);
    }

    #[test]
    fn bigraph_ranking_alg_test() {
        let graph = onlinematching::papers::kvv90::example::ranking_worst_case(200);
        type Ranking = onlinematching::papers::kvv90::algorithm::Ranking;
        let opt = graph.OPT();
        let alg = graph.ALG::<Ranking>();
        let ratio = alg / opt;
        println!("the ratio is {:?}", ratio);
        println!("alg: {:?}, opt: {:?}", alg, opt);
    }
}
