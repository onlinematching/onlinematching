use crate::papers::algorithm::algorithm::OnlineAlgorithm;
use std::default::Default;

use super::util::get_available_offline_nodes_in_weighted_onlineadj;

type OfflineInfo<Weight> = Vec<Weight>;

pub struct Balance<Weight> {
    offline_nodes_budgets: Vec<Weight>,
    offline_nodes_available: Vec<bool>,
    offline_nodes_loads: Vec<Weight>,
    alg: Weight,
}

