use sysinfo::System;

pub fn get_boot_time() -> u64 {
    System::boot_time()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_boot_time() {
        let boot_time = get_boot_time();
        assert!(boot_time > 0);
    }
}
