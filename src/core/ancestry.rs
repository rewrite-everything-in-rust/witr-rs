use crate::core::models::Process;

pub fn build_ancestry_tree(processes: Vec<Process>) -> Vec<Process> {
    processes.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::Process;

    #[test]
    fn test_build_ancestry_tree() {
        let p1 = Process::default();
        let p2 = Process::default();
        let list = vec![p1.clone(), p2.clone()];
        
        // Should reverse
        let tree = build_ancestry_tree(list);
        assert_eq!(tree.len(), 2);
    }
}
