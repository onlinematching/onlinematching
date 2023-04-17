#[cfg(test)]
mod tests_bigraph {
    #[test]
    fn st_simplist() {
        let t = 100000;
        let m = 100;
        let sr = onlinematching::papers::stochastic_reward::mp12::example::simplist(m);
        let opt = sr.OPT();
        let alg = sr.ALG::<onlinematching::papers::stochastic_reward::mp12::Balance>(t);
        println!("opt = {:?}, alg = {:?}", opt, alg);
    }
}
