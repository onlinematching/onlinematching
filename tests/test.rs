pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


mod tests_bi {
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
    
}