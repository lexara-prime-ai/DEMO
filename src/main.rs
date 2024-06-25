#![allow(non_snake_case)]
#![allow(unused)]

#[derive(Debug)]
enum AppState {
    RUNNING,
    IDLE,
}

impl AppState {
    fn as_str(&self) -> &'static str {
        match self {
            AppState::RUNNING => "RUNNING",
            AppState::IDLE => "IDLE",
        }
    }
}

#[derive(Debug)]
struct State {
    status: AppState,
}

impl State {
    fn new() -> Self {
        Self {
            status: AppState::RUNNING,
        }
    } 
}

fn main() {
    let state = State::new();

    println!("{:?}", state);
}