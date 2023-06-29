use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    app::App,
    components::{
        cpu_monitor::cpu_monitor, process_details::process_details, process_list::process_list,
        system_monitor::system_monitor,
    },
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Main block
    let block = Block::default();
    f.render_widget(block, size);

    // Layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(79),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(main_chunks[0]);

    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(main_chunks[1]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(main_chunks[2]);

    let top_left = top_chunks[0];
    let top_center = top_chunks[1];
    let top_right = top_chunks[2];
    let center_left = center_chunks[0];
    let center_right = center_chunks[1];
    let bottom_left = bottom_chunks[0];
    let bottom_right = bottom_chunks[1];

    // System Monitor
    f.render_widget(system_monitor(&mut app.state), top_left);

    // CPU Monitor
    f.render_widget(cpu_monitor(&mut app.state), top_center);

    // RAM Monitor
    let ram_monitor_area = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Plain)
        .title(vec![Span::from("Memory")]);
    f.render_widget(ram_monitor_area, top_right);

    // Proccess list
    let state = &mut app.state.selected_process.clone();
    f.render_stateful_widget(process_list(&mut app.state), center_left, state);

    // Process detail
    f.render_widget(process_details(&mut app.state), center_right);

    // Controls
    let controls_area = Block::default().title(vec![Span::from(
        "[Q]Quit [1]Name [2]PID [3]RAM [4]CPU [5]Disk",
    )]);
    f.render_widget(controls_area, bottom_left);

    // Version
    let version_area = Block::default()
        .title_alignment(Alignment::Right)
        .title(vec![Span::from("Bundle")]);
    f.render_widget(version_area, bottom_right);
}
