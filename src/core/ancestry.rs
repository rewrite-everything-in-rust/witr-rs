use crate::core::models::Process;

pub fn build_ancestry_tree(processes: Vec<Process>) -> Vec<Process> {
     processes.into_iter().rev().collect()
}
