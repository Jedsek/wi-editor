use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Dashboard(DashState),
    Normal,
    Command,
    Input,
    Search,
    Help,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DashState {
    Default,
    Recent,
}

impl Display for DashState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            DashState::Default => "Dashboard/default",
            DashState::Recent => "Dashboard/recent",
        };
        write!(f, "{}", state)
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mode::*;
        let mode = match self {
            Dashboard(state) => return state.fmt(f),
            Normal => "Normal",
            Command => "Command",
            Input => "Input",
            Search => "Search",
            Help => "Help",
        };
        write!(f, "{}", mode)
    }
}

impl Default for Mode {
    fn default() -> Self {
        // Self::Dashboard(DashState::Default)
        Self::Normal
    }
}

impl Mode {
    pub fn is_input(&self) -> bool {
        self == &Mode::Input
    }

    pub fn is_normal(&self) -> bool {
        self == &Mode::Normal
    }

    pub fn is_command(&self) -> bool {
        self == &Mode::Command
    }

    pub fn is_search(&self) -> bool {
        self == &Mode::Search
    }

    pub fn is_help(&self) -> bool {
        self == &Mode::Help
    }

    pub fn is_dashboard(&self, state: DashState) -> bool {
        self == &Mode::Dashboard(state)
    }

    pub fn to_input(&mut self) {
        *self = Mode::Input
    }

    pub fn to_dashboard(&mut self, state: DashState) {
        *self = Mode::Dashboard(state)
    }

    pub fn to_normal(&mut self) {
        *self = Mode::Normal
    }

    pub fn to_command(&mut self) {
        *self = Mode::Command
    }

    pub fn to_search(&mut self) {
        *self = Mode::Search
    }

    pub fn to_help(&mut self) {
        *self = Mode::Help
    }
}
