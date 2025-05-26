//! TUI 모드 모듈
//! 터미널 사용자 인터페이스 구현

pub mod app;
pub mod events;
pub mod handlers;
pub mod input;
pub mod state;
pub mod ui;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use app::App;

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // 터미널 설정
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 앱 실행
    let mut app = App::new()?;
    let result = app.run(&mut terminal);

    // 터미널 정리
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}
