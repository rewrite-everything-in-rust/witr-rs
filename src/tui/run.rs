use crate::adapters::system::RealSystem;
use crate::core::service::WitrService;
use crate::tui::app::App;
use crate::tui::ui::ui;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

pub fn run_tui(target_pid: Option<u32>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(" Witr-RS Watch Mode ", target_pid);
    let sys = RealSystem::new();
    let service = WitrService::new(sys);

    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    refresh_data(&mut app, &service);

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => app.on_esc(),
                        KeyCode::Backspace => app.on_backspace(),
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Up => app.select_previous(),
                        KeyCode::Down => app.select_next(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            refresh_data(&mut app, &service);
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn refresh_data(app: &mut App, service: &WitrService<RealSystem>) {
    if let Some(pid) = app.target_pid {
        if let Ok(ancestry) = service.get_ancestry(pid) {
            app.set_data(ancestry);
            if let Some(target) = app.processes.last() {
                app.title = format!(
                    " Witr-RS Watch Mode - Term: {} (CPU: {:.1}%) ",
                    target.name, target.cpu_usage
                );
            }
        } else {
            app.title = format!(" Witr-RS Watch Mode (Process {} Lost) ", pid);
        }
    } else {
        if let Ok(_pids) = service.get_all_pids() {
            let procs = Vec::new();
            // TODO: Fetching all process details is resource intensive.
            // We disabled this for now to optimize performance later.

            /*
            for pid in pids.iter().take(300) {
                 if let Ok(p) = service.inspect_pid(*pid) {
                     procs.push(p);
                 }
            }
            procs.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
            */

            // Placeholder to indicate Global Mode is active but limited
            app.title =
                " Witr-RS Global Monitor (Detail Fetch Disabled - CPU Optimized) ".to_string();
            app.set_data(procs); // Empty list for now
        }
    }
}
