pub fn get_available_offline_nodes_in_onlineadj(
    offline_nodes_available: &Vec<bool>,
    online_adjacent: &Vec<usize>,
) -> Vec<usize> {
    let mut vec = Vec::with_capacity(online_adjacent.len());
    for &offline_node in online_adjacent.iter() {
        if offline_nodes_available[offline_node] {
            vec.push(offline_node)
        }
    }
    vec
}
