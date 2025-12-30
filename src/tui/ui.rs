use crate::core::models::Process;
use crate::tui::app::{App, InputMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(3), // Search
                Constraint::Min(0),    // Body
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.area());

    let title_text = if app.target_pid.is_none() {
        " Witr-RS Global Process Monitor ".to_string()
    } else {
        format!(" {} ", app.title)
    };

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .title(title_text);
    f.render_widget(title_block, chunks[0]);

    // Search Bar
    let search_text = format!(
        "Filter: {}{}",
        app.search_query,
        if let InputMode::Editing = app.input_mode {
            "_"
        } else {
            ""
        }
    );
    let search_style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::Yellow),
        _ => Style::default(),
    };
    let search_block = Paragraph::new(search_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Search (Press '/') ")
            .border_style(search_style),
    );
    f.render_widget(search_block, chunks[1]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[2]);

    // List of Processes
    let query = app.search_query.to_lowercase();
    let filtered: Vec<&Process> = if query.is_empty() {
        app.processes.iter().collect()
    } else {
        app.processes
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query)
                    || p.pid.to_string().contains(&query)
                    || p.cmd.join(" ").to_lowercase().contains(&query)
            })
            .collect()
    };
    let items: Vec<ListItem> = filtered
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let style = if app.target_pid == Some(p.pid) {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let mem_mb = p.memory_usage as f32 / 1024.0 / 1024.0;

            // Tree View
            let name_display = if app.target_pid.is_some() && query.is_empty() {
                if i == 0 {
                    p.name.clone()
                } else {
                    format!("{}└─ {}", "  ".repeat(i), p.name)
                }
            } else {
                p.name.clone()
            };

            let content = Line::from(vec![
                Span::styled(format!("{:<6}", p.pid), style),
                Span::styled(
                    format!("{:<5.1}% ", p.cpu_usage),
                    if p.cpu_usage > 50.0 {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default().fg(Color::Cyan)
                    },
                ),
                Span::styled(
                    format!("{:<5.0}MB ", mem_mb),
                    Style::default().fg(Color::Magenta),
                ),
                Span::styled(name_display, style),
            ]);
            ListItem::new(content)
        })
        .collect();

    let list_title = if app.target_pid.is_some() {
        " Ancestry Tree "
    } else {
        " Process List "
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(list_title))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, body_chunks[0], &mut app.list_state);

    // Details Panel
    if let Some(selected_index) = app.list_state.selected() {
        if let Some(process) = filtered.get(selected_index) {
            draw_details(f, process, body_chunks[1]);
        }
    } else {
        let block = Block::default().borders(Borders::ALL).title(" Details ");
        f.render_widget(block, body_chunks[1]);
    }

    // Help footer
    let help_text = match app.input_mode {
        InputMode::Normal => "Press 'q' or 'Esc' to quit | 'j'/'k' to navigate | '/' to search",
        InputMode::Editing => "Type to filter | 'Esc' to format input | 'Backspace' to delete",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}

fn draw_details(f: &mut Frame, p: &Process, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(8),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    // Collecting info lines
    let mut lines = vec![
        Line::from(vec![
            Span::styled("PID: ", Style::default().fg(Color::Yellow)),
            Span::raw(p.pid.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Yellow)),
            Span::raw(&p.name),
        ]),
        Line::from(vec![
            Span::styled("Command: ", Style::default().fg(Color::Yellow)),
            Span::raw(p.cmd.join(" ")),
        ]),
        Line::from(""),
    ];

    if let Some(user) = &p.username {
        lines.push(Line::from(vec![
            Span::styled("User: ", Style::default().fg(Color::Cyan)),
            Span::raw(user),
        ]));
    }
    if let Some(service) = &p.service {
        lines.push(Line::from(vec![
            Span::styled("Service: ", Style::default().fg(Color::Cyan)),
            Span::raw(service),
        ]));
    }
    if let Some(unit) = &p.service_file {
        lines.push(Line::from(vec![
            Span::styled("Unit File: ", Style::default().fg(Color::Cyan)),
            Span::raw(unit),
        ]));
    }
    if let Some(container) = &p.container {
        lines.push(Line::from(vec![
            Span::styled("Container: ", Style::default().fg(Color::Magenta)),
            Span::raw(container),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("State: ", Style::default().fg(Color::Green)),
        Span::raw(&p.health),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Start Time: ", Style::default().fg(Color::Green)),
        Span::raw(crate::core::time::format_duration(p.start_time).0),
    ]));

    // Add Ports
    if !p.ports.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Open Ports:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )));
        for port in &p.ports {
            lines.push(Line::from(format!(" - {}", port)));
        }
    }

    let info = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Process Details "),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(info, chunks[0]);

    // CPU Gauge
    let cpu_clamped = p.cpu_usage.min(100.0) as u16;
    let cpu_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" CPU Usage "))
        .gauge_style(Style::default().fg(Color::Red))
        .percent(cpu_clamped)
        .label(format!("{:.1}%", p.cpu_usage));
    f.render_widget(cpu_gauge, chunks[1]);

    // Visual limit: 4GB
    let mem_mb = p.memory_usage as f64 / 1024.0 / 1024.0;
    let mem_percent = (mem_mb / 4096.0 * 100.0).min(100.0) as u16; // Scale to 4GB

    let mem_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Memory Usage "),
        )
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(mem_percent)
        .label(format!("{:.0} MB", mem_mb));
    f.render_widget(mem_gauge, chunks[2]);
}
