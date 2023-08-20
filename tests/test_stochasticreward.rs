#[cfg(test)]
mod test_stochastic_reward {
    use onlinematching::papers::stochastic_reward::mp12;
    use onlinematching::papers::stochastic_reward::mp12::from_nonweight_edges;
    use onlinematching::papers::stochastic_reward::ranking;
    #[test]
    fn st_simplist() {
        let t = 100000;
        let m = 200;
        let sr = mp12::example::simplist(m);
        let opt = sr.OPT();
        let alg = sr.adaptive_ALG::<mp12::Balance>(t);
        println!("opt = {:?}, alg = {:?}", opt, alg);
    }

    #[test]
    fn test_g2() {
        let t = 100000;
        let m = 200;
        let sr = mp12::example::gk(2, m);
        let opt = sr.OPT();
        let alg = sr.adaptive_ALG::<mp12::Balance>(t);
        let ratio = alg / opt;
        println!("opt = {:?}, alg = {:?}, ratio = {:?}", opt, alg, ratio);
    }

    #[test]
    fn test_ranking_g2() {
        let t = 100000;
        let m = 100;
        let sr = mp12::example::gk(2, m);
        let opt = sr.OPT();
        let alg = sr.adaptive_ALG::<ranking::Ranking>(t);
        let ratio = alg / opt;
        println!("opt = {:?}, alg = {:?}, ratio = {:?}", opt, alg, ratio);
    }

    #[test]
    fn test_balance_for_not_symmetry() {
        let t = 100000;
        let m = 200;
        let edges: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (1, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        let g = from_nonweight_edges(&edges, m);
        let sr = g.into_stochastic_reward();
        let opt = sr.OPT();
        let alg = sr.adaptive_ALG::<ranking::Ranking>(t);
        let ratio = alg / opt;
        println!("opt = {:?}, alg = {:?}, ratio = {:?}", opt, alg, ratio);
    }
}
