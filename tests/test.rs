#[cfg(test)]
mod tests {
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

    #[test]
    fn reuseableresource_ranking_alg_test() {
        let _graph = onlinematching::papers::reuseableresource::identical::example::z_graph_with_duration(200, 200);
        let edges = vec![
            (1, 1),
            (2, 2),
            (3, 3),
            (1, 4),
            (2, 5),
            (3, 6),
        ];
        let graph = Bigraph::from_edges(&edges);
        let duration = 2;
        let graph = graph.into_reuseable_online(duration);
        let mut opt = 0.;
        let mut alg = 0.;
        for _ in 0..10000 {
            opt += graph.OPT();
            alg += graph.ALG::<onlinematching::papers::reuseableresource::identical::algorithm::Ranking>();
        }
        let ratio = alg / opt;
        println!("the ratio is {:?}", ratio);
        println!("alg: {:?}, opt: {:?}", alg, opt);
    }
}
