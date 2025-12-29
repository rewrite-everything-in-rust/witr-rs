use crate::core::color::ColorScheme;
use crate::core::models::Process;
use crate::core::time;

pub fn print(target: &Process, chain: &[Process], colors: &ColorScheme) {
    println!("{}      : {}", colors.header("Target"), target.name);
    println!();

    print!(
        "{}     : {} {}",
        colors.header("Process"),
        target.name,
        colors.dim(&format!("(pid {})", target.pid))
    );

    if target.health != "healthy" {
        print!(" {}", colors.badge(&format!("[{}]", target.health)));
    }
    if target.forked == "forked" {
        print!(" {}", colors.badge("{forked}"));
    }
    println!();

    if let Some(uid) = &target.uid {
        println!("{}        : {}", colors.metadata("User"), uid);
    }

    if target.container.is_some() {
        println!(
            "{}   : {}",
            colors.metadata("Container"),
            target.container.as_ref().unwrap()
        );
    }

    if target.service.is_some() {
        println!(
            "{}     : {}",
            colors.metadata("Service"),
            target.service.as_ref().unwrap()
        );
    }

    if !target.cmd.is_empty() {
        println!(
            "{}     : {}",
            colors.command("Command"),
            target.cmd.join(" ")
        );
    }

    let (relative, absolute) = time::format_duration(target.start_time);
    println!(
        "{}     : {} {}",
        colors.metadata("Started"),
        relative,
        colors.dim(&format!("({})", absolute))
    );

    println!();
    print!("{}: ", colors.header("Why It Exists"));
    let names: Vec<String> = chain
        .iter()
        .map(|p| format!("{} {}", p.name, colors.dim(&format!("(pid {})", p.pid))))
        .collect();
    println!("{}", names.join(" → "));
    println!();

    let source_label = if let Some(name) = chain
        .last()
        .and_then(|p| p.service.as_ref().or(p.container.as_ref()))
    {
        name.clone()
    } else if target.parent_pid == Some(1) || target.parent_pid.is_none() {
        "system".to_string()
    } else {
        "manual".to_string()
    };

    println!("{}      : {}", colors.metadata("Source"), source_label);
    println!();

    if let Some(cwd) = &target.cwd {
        println!("{} : {}", colors.command("Working Dir"), cwd);
    }
    if let Some(repo) = &target.git_repo {
        if let Some(branch) = &target.git_branch {
            println!(
                "{}    : {} {}",
                colors.metadata("Git Repo"),
                repo,
                colors.dim(&format!("({})", branch))
            );
        } else {
            println!("{}    : {}", colors.metadata("Git Repo"), repo);
        }
    }
    if !target.ports.is_empty() {
        for (i, (port, addr)) in target.ports.iter().zip(&target.bind_addrs).enumerate() {
            if i == 0 {
                println!("{}   : {}:{}", colors.command("Listening"), addr, port);
            } else {
                println!("              {}:{}", addr, port);
            }
        }
    }

    if target.health != "healthy"
        || target.uid == Some("0".to_string())
        || target
            .bind_addrs
            .iter()
            .any(|a| a.starts_with("0.0.0.0") || a == "::")
    {
        println!();
        println!("{}:", colors.warning("Warnings"));
        if target.health != "healthy" {
            println!("  • Process is {}", target.health);
        }
        if target.uid == Some("0".to_string()) {
            println!("  • Running as root");
        }
        for (port, addr) in target.ports.iter().zip(&target.bind_addrs) {
            if addr.starts_with("0.0.0.0") || addr == "::" {
                println!("  • Listening publicly on {}:{}", addr, port);
            }
        }
    }
}
