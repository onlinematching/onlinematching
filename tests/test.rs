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
    
}