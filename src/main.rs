use anyhow::Result;
use clap::Parser;
use witr_rs::adapters::system::RealSystem;
use witr_rs::core::color::ColorScheme;
use witr_rs::core::service::WitrService;
use witr_rs::output;

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

    #[arg(long, aliases = ["sec", "scan"])]
    security_scan: bool,

    #[arg(long, help = "Live watch mode")]
    watch: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let colors = ColorScheme::new(!args.no_color);

    let sys_adapter = RealSystem::new();
    let service = WitrService::new(sys_adapter);

    if args.watch {
        let mut target_pid = args.pid;

        if target_pid.is_none() {
            if let Some(name) = &args.name {
                if let Ok(procs) = service.inspect_name(name) {
                    if let Some(p) = procs.first() {
                        target_pid = Some(p.pid);
                    }
                }
            } else if let Some(port) = args.port {
                if let Ok(p) = service.inspect_port(port) {
                    target_pid = Some(p.pid);
                }
            }
        }

        if let Some(pid) = target_pid {
            if let Err(e) = witr_rs::tui::run::run_tui(Some(pid)) {
                eprintln!("Error running TUI: {}", e);
            }
            return Ok(());
        } else {
            eprintln!(
                "Global Watch Mode is currently disabled/WIP (Performance Optimization pending)."
            );
            eprintln!("Please specify a process target (PID, Name, or Port).");
            eprintln!("Usage: witr-rs <NAME> --watch OR witr-rs -p <PID> --watch");
            return Ok(());
        }
    }

    if args.env {
        if let Some(pid) = args.pid {
            match service.inspect_pid(pid) {
                Ok(process) => output::envonly::print(&process),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        return Ok(());
    }

    if args.security_scan {
        let targets = if let Some(pid) = args.pid {
            match service.get_inspection(pid) {
                Ok(r) => vec![r],
                Err(e) => {
                    eprintln!("Error inspecting PID {}: {}", pid, e);
                    return Ok(());
                }
            }
        } else if let Some(name) = &args.name {
            match service.inspect_name(name) {
                Ok(procs) => {
                    let mut results = Vec::new();
                    for p in procs {
                        if let Ok(res) = service.get_inspection(p.pid) {
                            results.push(res);
                        }
                    }
                    results
                }
                Err(e) => {
                    eprintln!("Error inspecting name {}: {}", name, e);
                    return Ok(());
                }
            }
        } else if let Some(port) = args.port {
            match service.inspect_port(port) {
                Ok(p) => match service.get_inspection(p.pid) {
                    Ok(r) => vec![r],
                    Err(e) => {
                        eprintln!("Error getting inspection for PID {}: {}", p.pid, e);
                        return Ok(());
                    }
                },
                Err(e) => {
                    eprintln!("Error inspecting port {}: {}", port, e);
                    return Ok(());
                }
            }
        } else {
            println!("Scanning all processes for security issues... (this may take a moment)");
            match service.inspect_all() {
                Ok(results) => results,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return Ok(());
                }
            }
        };

        if !targets.is_empty() {
            output::security::print_report(&targets, &colors);
        } else {
            println!("No targets found to scan.");
        }
        return Ok(());
    }

    if let Some(name) = args.name {
        match service.inspect_name(&name) {
            Ok(processes) => {
                for process in processes {
                    match service.get_ancestry(process.pid) {
                        Ok(chain) => {
                            if args.short {
                                output::short::print(&chain, &colors);
                            } else if args.tree {
                                output::tree::print(&chain, 0);
                            } else if args.json {
                                let _ = output::json::print(&process, &chain);
                            } else if args.warnings {
                                output::warnings::print(&chain);
                            } else {
                                output::standard::print(&process, &chain, &colors);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(pid) = args.pid {
        match service.get_ancestry(pid) {
            Ok(chain) => {
                if let Some(target) = chain.last() {
                    if args.short {
                        output::short::print(&chain, &colors);
                    } else if args.tree {
                        output::tree::print(&chain, 0);
                    } else if args.json {
                        let _ = output::json::print(target, &chain);
                    } else if args.warnings {
                        output::warnings::print(&chain);
                    } else {
                        output::standard::print(target, &chain, &colors);
                    }
                } else {
                    eprintln!("Error: Process {} not found", pid);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(port) = args.port {
        match service.inspect_port(port) {
            Ok(process) => match service.get_ancestry(process.pid) {
                Ok(chain) => {
                    if args.short {
                        output::short::print(&chain, &colors);
                    } else if args.tree {
                        output::tree::print(&chain, 0);
                    } else if args.json {
                        let _ = output::json::print(&process, &chain);
                    } else if args.warnings {
                        output::warnings::print(&chain);
                    } else {
                        output::standard::print(&process, &chain, &colors);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        eprintln!("Please specify a process name, PID, or port to inspect");
        std::process::exit(1);
    }

    Ok(())
}
