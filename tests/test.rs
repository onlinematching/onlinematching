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
        let g = bigraph::bigraph::from_edges(&vec![
            ("","w")
        ]);
        drop(g);
    }
    
}