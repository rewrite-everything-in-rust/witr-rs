use clap::Parser;
use anyhow::Result;
use witr_rs::adapters::system::RealSystem;
use witr_rs::core::service::WitrService;
use witr_rs::core::models::Process;
use witr_rs::core::color::ColorScheme;
use witr_rs::core::time;

#[derive(Parser, Debug)]
#[command(name = "witr-rs", version, about = "Why is this running? (Rust edition)", long_about = None)]
struct Args {
    name: Option<String>,

    #[arg(short = 'p', long)]
    pid: Option<u32>,

    #[arg(short = 'P', long)]
    port: Option<u16>,

    #[arg(long)]
    short: bool,

    #[arg(long)]
    tree: bool,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    warnings: bool,

    #[arg(long)]
    no_color: bool,

    #[arg(long)]
    env: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let colors = ColorScheme::new(!args.no_color);
    
    let sys_adapter = RealSystem::new();
    let service = WitrService::new(sys_adapter);

    if args.env {
        if let Some(pid) = args.pid {
            match service.inspect_pid(pid) {
                Ok(process) => {
                    for e in &process.env {
                        println!("{}", e);
                    }
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        } else {
            eprintln!("--env requires --pid");
        }
        return Ok(());
    }

    if args.json {
        if let Some(pid) = args.pid {
            match service.inspect_pid(pid) {
                Ok(process) => println!("{}", serde_json::to_string_pretty(&process)?),
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if let Some(name) = args.name {
            match service.inspect_name(&name) {
                Ok(processes) => println!("{}", serde_json::to_string_pretty(&processes)?),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        return Ok(());
    }

    if let Some(pid) = args.pid {
        match service.get_ancestry(pid) {
            Ok(chain) => {
                if args.short {
                    let names: Vec<String> = chain.iter()
                        .map(|p| format!("{} (pid {})", p.name, p.pid))
                        .collect();
                    println!("{}", names.join(" → "));
                } else if args.tree {
                    print_tree(&chain, 0);
                } else if args.warnings {
                    print_warnings(&chain);
                } else {
                    print_detailed(&chain.last().unwrap(), &chain, &colors);
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(name) = args.name {
        match service.inspect_name(&name) {
            Ok(processes) => {
                if processes.len() == 1 {
                    let p = &processes[0];
                    match service.get_ancestry(p.pid) {
                        Ok(chain) => {
                            if args.short {
                                let names: Vec<String> = chain.iter()
                                    .map(|p| format!("{} (pid {})", p.name, p.pid))
                                    .collect();
                                println!("{}", names.join(" → "));
                            } else if args.tree {
                                print_tree(&chain, 0);
                            } else if args.warnings {
                                print_warnings(&chain);
                            } else {
                                print_detailed(p, &chain, &colors);
                            }
                        },
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    println!("Multiple matching processes found:\n");
                    for (idx, p) in processes.iter().enumerate() {
                        println!("[{}] PID {}  {}  ({})", 
                            idx + 1, 
                            p.pid, 
                            p.cmd.first().unwrap_or(&p.name),
                            p.service.as_ref().unwrap_or(&"manual".to_string()));
                    }
                    println!("\nRe-run with:");
                    println!("  witr-rs --pid <pid>");
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(port) = args.port {
        match service.inspect_port(port) {
            Ok(process) => {
                match service.get_ancestry(process.pid) {
                    Ok(chain) => {
                        if args.short {
                            let names: Vec<String> = chain.iter()
                                .map(|p| format!("{} (pid {})", p.name, p.pid))
                                .collect();
                            println!("{}", names.join(" → "));
                        } else if args.tree {
                            print_tree(&chain, 0);
                        } else if args.warnings {
                            print_warnings(&chain);
                        } else {
                            print_detailed(&process, &chain, &colors);
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn print_tree(chain: &[Process], indent: usize) {
    for (i, p) in chain.iter().enumerate() {
        if i == 0 {
            println!("{} (pid {})", p.name, p.pid);
        } else {
            let prefix = "  ".repeat(indent + i - 1);
            println!("{}└─ {} (pid {})", prefix, p.name, p.pid);
        }
    }
}

fn print_warnings(chain: &[Process]) {
    for p in chain {
        if p.health != "healthy" {
            println!("⚠  PID {} is {}", p.pid, p.health);
        }
        if p.uid == Some("0".to_string()) {
            println!("⚠  PID {} is running as root", p.pid);
        }
        if !p.ports.is_empty() {
            for (port, addr) in p.ports.iter().zip(&p.bind_addrs) {
                if addr.starts_with("0.0.0.0") || addr == "::" {
                    println!("⚠  PID {} is listening publicly on {}:{}", p.pid, addr, port);
                }
            }
        }
    }
}

fn print_detailed(target: &Process, chain: &[Process], colors: &ColorScheme) {
    println!("{}      : {}", colors.header("Target"), target.name);
    println!();
    
    print!("{}     : {} {}", 
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
        println!("{}   : {}", colors.metadata("Container"), target.container.as_ref().unwrap());
    }
    
    if target.service.is_some() {
        println!("{}     : {}", colors.metadata("Service"), target.service.as_ref().unwrap());
    }
    
    if !target.cmd.is_empty() {
        println!("{}     : {}", colors.command("Command"), target.cmd.join(" "));
    }
    
    let (relative, absolute) = time::format_duration(target.start_time);
    println!("{}     : {} {}", 
        colors.metadata("Started"), 
        relative,
        colors.dim(&format!("({})", absolute))
    );
    
    println!();
    print!("{}: ", colors.header("Why It Exists"));
    let names: Vec<String> = chain.iter()
        .map(|p| format!("{} {}", p.name, colors.dim(&format!("(pid {})", p.pid))))
        .collect();
    println!("{}", names.join(" → "));
    println!();

    let source_label = if let Some(name) = chain.last().and_then(|p| {
        p.service.as_ref().or(p.container.as_ref())
    }) {
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
            println!("{}    : {} {}", 
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
    
    if target.health != "healthy" || target.uid == Some("0".to_string()) || 
       target.bind_addrs.iter().any(|a| a.starts_with("0.0.0.0") || a == "::") {
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
