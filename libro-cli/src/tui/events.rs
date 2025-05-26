//! 이벤트 처리

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
}

pub struct EventHandler {
    last_tick: Instant,
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            last_tick: Instant::now(),
            tick_rate,
        }
    }

    pub fn next(&mut self) -> Result<AppEvent, Box<dyn std::error::Error>> {
        let timeout = self
            .tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => Ok(AppEvent::Key(key)),
                Event::Resize(width, height) => Ok(AppEvent::Resize(width, height)),
                _ => Ok(AppEvent::Tick),
            }
        } else {
            self.last_tick = Instant::now();
            Ok(AppEvent::Tick)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    // 네비게이션 (Vim 스타일)
    MoveUp,    // j
    MoveDown,  // k
    MoveLeft,  // h
    MoveRight, // l

    // 기본 액션
    Select, // Enter
    Back,   // Esc
    Quit,   // q

    // 도서 관리
    AddBook,      // a
    EditBook,     // e
    DeleteBook,   // d
    AddReview,    // v (view/review)
    NewReview,    // n (new review)
    DeleteReview, // d (delete review - context dependent)

    // 기능
    Search, // /
    Report, // r
    Help,   // ?

    // 폼 네비게이션
    NextField, // Tab
    PrevField, // Shift+Tab

    // 리포트 뷰 전환
    AuthorReport, // 1
    YearReport,   // 2
    RecentReport, // 3

    // 편집 모드 전용
    SaveEdit,   // Ctrl+S
    CancelEdit, // Ctrl+X
    ForceQuit,  // Ctrl+Q

    // 텍스트 편집
    InsertChar(char),
    DeleteChar,
    Backspace,
    NewLine,

    // 커서 이동 (편집 모드)
    CursorLeft,  // Ctrl+H
    CursorRight, // Ctrl+L
    CursorUp,    // Ctrl+J (또는 Ctrl+K와 충돌 시)
    CursorDown,  // Ctrl+J

    // 라인 편집
    LineStart,   // Ctrl+A
    LineEnd,     // Ctrl+E
    ClearLine,   // Ctrl+U
    DeleteToEnd, // Ctrl+K
    DeleteWord,  // Ctrl+W

    // 확인 모드
    Confirm, // y/Y
    Cancel,  // n/N

    // 특수
    ToggleMode, // Space (선택 토글용)

    // 무시 (편집 모드에서 전역 키)
    Ignore,
}

pub fn key_to_action(key: KeyEvent, mode: &crate::tui::state::AppMode) -> KeyAction {
    use crate::tui::state::AppMode;

    match mode {
        AppMode::FormInput => {
            // 폼 직접 입력 모드 - 텍스트 입력과 기본 네비게이션만 허용
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => KeyAction::Back,
                (KeyModifiers::NONE, KeyCode::Tab) => KeyAction::NextField,
                (KeyModifiers::SHIFT, KeyCode::BackTab) => KeyAction::PrevField,
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::Select,
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => KeyAction::SaveEdit,
                (KeyModifiers::NONE, KeyCode::Backspace) => KeyAction::Backspace,
                (KeyModifiers::NONE, KeyCode::Delete) => KeyAction::DeleteChar,
                (KeyModifiers::CONTROL, KeyCode::Char('u')) => KeyAction::ClearLine,
                (KeyModifiers::CONTROL, KeyCode::Char('a')) => KeyAction::LineStart,
                (KeyModifiers::CONTROL, KeyCode::Char('e')) => KeyAction::LineEnd,
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => KeyAction::DeleteToEnd,
                (KeyModifiers::CONTROL, KeyCode::Char('w')) => KeyAction::DeleteWord,
                (KeyModifiers::NONE, KeyCode::Left) => KeyAction::CursorLeft,
                (KeyModifiers::NONE, KeyCode::Right) => KeyAction::CursorRight,
                (KeyModifiers::NONE, KeyCode::Char(c)) => KeyAction::InsertChar(c),
                _ => KeyAction::Ignore,
            }
        }
        AppMode::GenreSelect => {
            // 장르 선택 모드
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => KeyAction::Back,
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::Select,
                (KeyModifiers::NONE, KeyCode::Char('j')) | (KeyModifiers::NONE, KeyCode::Down) => {
                    KeyAction::MoveDown
                }
                (KeyModifiers::NONE, KeyCode::Char('k')) | (KeyModifiers::NONE, KeyCode::Up) => {
                    KeyAction::MoveUp
                }
                _ => KeyAction::Ignore,
            }
        }
        AppMode::YearSelect => {
            // 출간년도 선택 모드
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => KeyAction::Back,
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::Select,
                (KeyModifiers::NONE, KeyCode::Char('j')) | (KeyModifiers::NONE, KeyCode::Down) => {
                    KeyAction::MoveDown
                }
                (KeyModifiers::NONE, KeyCode::Char('k')) | (KeyModifiers::NONE, KeyCode::Up) => {
                    KeyAction::MoveUp
                }
                _ => KeyAction::Ignore,
            }
        }
        AppMode::Edit => {
            // 편집 모드에서는 Ctrl 조합키와 특수 키만 처리
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => KeyAction::SaveEdit,
                (KeyModifiers::CONTROL, KeyCode::Char('x')) => KeyAction::CancelEdit,
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => KeyAction::ForceQuit,
                (KeyModifiers::CONTROL, KeyCode::Char('a')) => KeyAction::LineStart,
                (KeyModifiers::CONTROL, KeyCode::Char('e')) => KeyAction::LineEnd,
                (KeyModifiers::CONTROL, KeyCode::Char('u')) => KeyAction::ClearLine,
                (KeyModifiers::CONTROL, KeyCode::Char('k')) => KeyAction::DeleteToEnd,
                (KeyModifiers::CONTROL, KeyCode::Char('w')) => KeyAction::DeleteWord,
                (KeyModifiers::CONTROL, KeyCode::Char('h')) => KeyAction::CursorLeft,
                (KeyModifiers::CONTROL, KeyCode::Char('l')) => KeyAction::CursorRight,
                (KeyModifiers::CONTROL, KeyCode::Char('j')) => KeyAction::CursorDown,
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::NewLine,
                (KeyModifiers::NONE, KeyCode::Backspace) => KeyAction::Backspace,
                (KeyModifiers::NONE, KeyCode::Delete) => KeyAction::DeleteChar,
                (KeyModifiers::NONE, KeyCode::Left) => KeyAction::CursorLeft,
                (KeyModifiers::NONE, KeyCode::Right) => KeyAction::CursorRight,
                (KeyModifiers::NONE, KeyCode::Up) => KeyAction::CursorUp,
                (KeyModifiers::NONE, KeyCode::Down) => KeyAction::CursorDown,
                (KeyModifiers::NONE, KeyCode::Char(c)) => KeyAction::InsertChar(c),
                _ => KeyAction::Ignore, // 모든 다른 키는 무시
            }
        }
        AppMode::Search => {
            // 검색 모드
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::Select,
                (KeyModifiers::NONE, KeyCode::Esc) => KeyAction::Back,
                (KeyModifiers::CONTROL, KeyCode::Char('u')) => KeyAction::ClearLine,
                (KeyModifiers::NONE, KeyCode::Backspace) => KeyAction::Backspace,
                (KeyModifiers::NONE, KeyCode::Char(c)) => KeyAction::InsertChar(c),
                _ => KeyAction::Ignore,
            }
        }
        AppMode::Confirm => {
            // 확인 모드
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('y'))
                | (KeyModifiers::SHIFT, KeyCode::Char('Y')) => KeyAction::Confirm,
                (KeyModifiers::NONE, KeyCode::Char('n'))
                | (KeyModifiers::SHIFT, KeyCode::Char('N')) => KeyAction::Cancel,
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::Select,
                (KeyModifiers::NONE, KeyCode::Esc) => KeyAction::Back,
                _ => KeyAction::Ignore,
            }
        }
        AppMode::Normal => {
            // 일반 모드 - 모든 전역 키매핑 활성
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('q')) => KeyAction::Quit,
                (KeyModifiers::NONE, KeyCode::Esc) => KeyAction::Back,
                (KeyModifiers::NONE, KeyCode::Char('?')) => KeyAction::Help,
                (KeyModifiers::NONE, KeyCode::Char('j')) => KeyAction::MoveDown,
                (KeyModifiers::NONE, KeyCode::Char('k')) => KeyAction::MoveUp,
                (KeyModifiers::NONE, KeyCode::Char('h')) => KeyAction::MoveLeft,
                (KeyModifiers::NONE, KeyCode::Char('l')) => KeyAction::MoveRight,
                (KeyModifiers::NONE, KeyCode::Enter) => KeyAction::Select,
                (KeyModifiers::NONE, KeyCode::Char('a')) => KeyAction::AddBook,
                (KeyModifiers::NONE, KeyCode::Char('e')) => KeyAction::EditBook,
                (KeyModifiers::NONE, KeyCode::Char('d')) => KeyAction::DeleteBook, // 컨텍스트에 따라 DeleteReview로 처리됨
                (KeyModifiers::NONE, KeyCode::Char('v')) => KeyAction::AddReview,
                (KeyModifiers::NONE, KeyCode::Char('n')) => KeyAction::NewReview,
                (KeyModifiers::NONE, KeyCode::Char('/')) => KeyAction::Search,
                (KeyModifiers::NONE, KeyCode::Char('r')) => KeyAction::Report,
                (KeyModifiers::NONE, KeyCode::Tab) => KeyAction::NextField,
                (KeyModifiers::SHIFT, KeyCode::BackTab) => KeyAction::PrevField,
                (KeyModifiers::NONE, KeyCode::Char(' ')) => KeyAction::ToggleMode,
                (KeyModifiers::NONE, KeyCode::Char('1')) => KeyAction::AuthorReport,
                (KeyModifiers::NONE, KeyCode::Char('2')) => KeyAction::YearReport,
                (KeyModifiers::NONE, KeyCode::Char('3')) => KeyAction::RecentReport,
                (KeyModifiers::NONE, KeyCode::Up) => KeyAction::MoveUp,
                (KeyModifiers::NONE, KeyCode::Down) => KeyAction::MoveDown,
                (KeyModifiers::NONE, KeyCode::Left) => KeyAction::MoveLeft,
                (KeyModifiers::NONE, KeyCode::Right) => KeyAction::MoveRight,
                _ => KeyAction::Ignore,
            }
        }
    }
}
