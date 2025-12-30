use crate::core::models::Process;
use ratatui::widgets::ListState;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub target_pid: Option<u32>,
    pub processes: Vec<Process>,
    pub list_state: ListState,
    pub input_mode: InputMode,
    pub search_query: String,
}

impl App {
    pub fn new(title: &str, target_pid: Option<u32>) -> Self {
        Self {
            title: title.to_string(),
            should_quit: false,
            target_pid,
            processes: Vec::new(),
            list_state: ListState::default(),
            input_mode: InputMode::Normal,
            search_query: String::new(),
        }
    }

    pub fn on_key(&mut self, c: char) {
        match self.input_mode {
            InputMode::Normal => match c {
                'q' => self.should_quit = true,
                'j' => self.select_next(),
                'k' => self.select_previous(),
                '/' => self.input_mode = InputMode::Editing,
                _ => {}
            },
            InputMode::Editing => {
                self.search_query.push(c);
            }
        }
    }

    pub fn on_backspace(&mut self) {
        if let InputMode::Editing = self.input_mode {
            self.search_query.pop();
        }
    }

    pub fn on_esc(&mut self) {
        match self.input_mode {
            InputMode::Editing => {
                self.input_mode = InputMode::Normal;
            }
            InputMode::Normal => {
                self.should_quit = true;
            }
        }
    }

    pub fn select_next(&mut self) {
        if self.get_filtered_processes().is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.get_filtered_processes().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_previous(&mut self) {
        if self.get_filtered_processes().is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.get_filtered_processes().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn set_data(&mut self, data: Vec<Process>) {
        self.processes = data;
        // Auto-select if nothing selected
        if self.list_state.selected().is_none() && !self.processes.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn get_filtered_processes(&self) -> Vec<&Process> {
        if self.search_query.is_empty() {
            self.processes.iter().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.processes
                .iter()
                .filter(|p| {
                    p.name.to_lowercase().contains(&query)
                        || p.pid.to_string().contains(&query)
                        || p.cmd.join(" ").to_lowercase().contains(&query)
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_navigation() {
        let mut app = App::new("Test", Some(123));
        let p1 = Process {
            pid: 1,
            ..Default::default()
        };

        app.set_data(vec![p1]);

        assert_eq!(app.list_state.selected(), Some(0));
    }
}
