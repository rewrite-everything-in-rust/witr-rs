pub fn is_shell_process(comm: &str) -> bool {
    matches!(
        comm,
        "bash" | "sh" | "zsh" | "fish" | "csh" | "tcsh" | "ksh"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_shell_process() {
        assert!(is_shell_process("bash"));
        assert!(is_shell_process("zsh"));
        assert!(!is_shell_process("nginx"));
    }
}
