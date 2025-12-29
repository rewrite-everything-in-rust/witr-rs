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
}

fn main() -> Result<()> {
    let args = Args::parse();
    let colors = ColorScheme::new(!args.no_color);

    let sys_adapter = RealSystem::new();
    let service = WitrService::new(sys_adapter);

    if args.env {
        if let Some(pid) = args.pid {
            match service.inspect_pid(pid) {
                Ok(process) => output::envonly::print(&process),
                Err(e) => eprintln!("Error: {}", e),
            }
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
                if args.short {
                    output::short::print(&chain, &colors);
                } else if args.tree {
                    output::tree::print(&chain, 0);
                } else if args.json {
                    let _ = output::json::print(chain.last().unwrap(), &chain);
                } else if args.warnings {
                    output::warnings::print(&chain);
                } else {
                    output::standard::print(chain.last().unwrap(), &chain, &colors);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    } else if let Some(port) = args.port {
        match service.inspect_port(port) {
            Ok(process) => {
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
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        eprintln!("Please specify a process name, PID, or port to inspect");
        std::process::exit(1);
    }

    Ok(())
}
