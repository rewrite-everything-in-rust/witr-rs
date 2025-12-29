use crate::core::color::ColorScheme;
use crate::core::models::Process;

pub fn print(chain: &[Process], colors: &ColorScheme) {
    let names: Vec<String> = chain
        .iter()
        .map(|p| {
            format!(
                "{} {}",
                p.name,
                colors.dim(&format!("(pid {})", p.pid))
            )
        })
        .collect();
    println!("{}", names.join(" → "));
}
