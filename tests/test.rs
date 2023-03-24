#[cfg(test)]
mod tests {
    use std::vec;

    use onlinematching::bigraph::Bigraph;

    #[test]
    fn bigraph_random_alg_test() {
        let graph = onlinematching::papers::algorithm::example::random_worst_case(200);
        type Random = onlinematching::papers::algorithm::algorithm::Random;
        let opt = graph.OPT();
        let alg = graph.ALG::<Random>();
        let ratio = alg / opt;
        println!("the ratio is {:?}", ratio);
        println!("alg: {:?}, opt: {:?}", alg, opt);
    }

    #[test]
    fn bigraph_ranking_alg_test() {
        let graph = onlinematching::papers::algorithm::example::ranking_worst_case(200);
        type Ranking = onlinematching::papers::algorithm::algorithm::Ranking;
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
            (1, 2),
            (1, 3),
            (2, 3)
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
