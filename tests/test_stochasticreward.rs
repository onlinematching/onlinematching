#[cfg(test)]
mod tests_bigraph {
    use onlinematching::papers::stochastic_reward::mp12;
    #[test]
    fn st_simplist() {
        let t = 100000;
        let m = 100;
        let sr = mp12::example::simplist(m);
        let opt = sr.OPT();
        let alg = sr.adaptive_ALG::<mp12::Balance>(t);
        println!("opt = {:?}, alg = {:?}", opt, alg);
    }

    #[test]
    fn test_g2() {
        let t = 10000;
        let m = 100;
        let sr = mp12::example::gk(2, m);
        let opt = sr.OPT();
        let alg = sr.adaptive_ALG::<mp12::Balance>(t);
        let ratio = alg / opt;
        println!("opt = {:?}, alg = {:?}, ratio = {:?}", opt, alg, ratio);
    }
}
