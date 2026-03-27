mod slurm;
mod state;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, List, ListItem},
};
use std::{io, time::Duration};
use state::{RunState, Status};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = state::load();
    update_status(&mut state);

    let mut selected = 0;

    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = state.runs
                .iter()
                .enumerate()
                .map(|(i, run)| {
                    let marker = if run.checked { "[✓]" } else { "[ ]" };

                    let status = match run.status {
                        Status::Running => "⏳",
                        Status::Pending => "🟡",
                        Status::Finished => "✔",
                    };

                    let line = format!("{} {} {}", marker, run.name, status);

                    if i == selected {
                        ListItem::new(format!(">> {}", line))
                    } else {
                        ListItem::new(line)
                    }
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Cluster Monitor").borders(Borders::ALL));

            f.render_widget(list, f.size());
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    KeyCode::Down => {
                        if selected < state.runs.len() - 1 {
                            selected += 1;
                        }
                    }

                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }

                    KeyCode::Char(' ') => {
                        state.runs[selected].checked =
                            !state.runs[selected].checked;
                        state::save(&state);
                    }

                    KeyCode::Char('r') => {
                        update_status(&mut state);
                        state::save(&state);
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}

fn update_status(state: &mut RunState) {
    let jobs = slurm::get_jobs();

    for run in &mut state.runs {
        if let Some(s) = jobs.get(&run.job_id) {
            run.status = match s.as_str() {
                "R" => Status::Running,
                "PD" => Status::Pending,
                _ => Status::Running,
            };
        } else {
            run.status = Status::Finished;
        }
    }
}
