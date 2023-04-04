pub fn get_available_offline_nodes_in_weighted_onlineadj<Weight: Copy>(
    offline_nodes_available: &Vec<bool>,
    online_adjacent: &Vec<(usize, Weight)>,
) -> Vec<(usize, Weight)> {
    let mut vec = Vec::with_capacity(online_adjacent.len());
    for (offline_node, w) in online_adjacent.iter() {
        if offline_nodes_available[*offline_node] {
            vec.push((*offline_node, *w))
        }
    }
    vec
}
