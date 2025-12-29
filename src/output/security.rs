use crate::core::color::ColorScheme;
use crate::core::models::InspectionResult;
use colored::Colorize;

pub fn print_report(results: &[InspectionResult], colors: &ColorScheme) {
    println!("{}", colors.header("SECURITY SCAN REPORT"));
    println!("{}", colors.header("===================="));
    println!();

    if results.is_empty() {
        println!(
            "{}",
            colors.success(
                "No security issues found. System looks clean (based on basic heuristics)."
            )
        );
        return;
    }

    let mut critical_count = 0;
    let mut warning_count = 0;

    for res in results {
        for warning in &res.warnings {
            let (label, color_func): (&str, fn(&str) -> colored::ColoredString) = if warning
                .contains("CRITICAL")
                || warning.contains("REVERSE SHELL")
                || warning.contains("BINARY DELETED")
            {
                critical_count += 1;
                ("CRITICAL", |s| s.red().bold())
            } else {
                warning_count += 1;
                ("WARNING", |s| s.yellow().bold())
            };

            println!(
                "[{}] PID {} ({})",
                color_func(label),
                res.process.pid,
                colors.highlight(&res.process.name)
            );

            if let Some(parent) = res.ancestry.get(1) {
                println!("  Parent: {} ({})", parent.name, parent.pid);
            }
            println!("  Issue : {}", warning);
            println!();
        }
    }

    println!("Summary:");
    if critical_count > 0 {
        println!(
            "  Critical Issues: {}",
            critical_count.to_string().red().bold()
        );
    }
    println!("  Warnings       : {}", warning_count);
}
