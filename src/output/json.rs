use crate::core::models::Process;
use serde_json;
use std::io::{self, Write};

pub fn print_to_writer<W: Write>(
    writer: &mut W,
    target: &Process,
    chain: &[Process],
) -> Result<(), Box<dyn std::error::Error>> {
    let output = serde_json::json!({
        "target": target,
        "ancestry": chain,
    });
    writeln!(writer, "{}", serde_json::to_string_pretty(&output)?)?;
    Ok(())
}

pub fn print(target: &Process, chain: &[Process]) -> Result<(), Box<dyn std::error::Error>> {
    let mut handle = io::stdout().lock();
    print_to_writer(&mut handle, target, chain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_json() {
        let p = Process::default();
        let chain = vec![];
        let mut buffer = Vec::new();

        let res = print_to_writer(&mut buffer, &p, &chain);
        assert!(res.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("\"target\""));
        assert!(output.contains("\"ancestry\""));
    }
}
