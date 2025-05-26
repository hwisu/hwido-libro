//! 도움말 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::state::{AppMode, AppState};

/// 도움말 화면을 렌더링합니다
pub fn render_help(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 도움말 내용
        ])
        .split(area);

    // 헤더
    let header = Paragraph::new("📖 Libro - 도움말")
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(header, area);

    // 도움말 내용
    let help_content = match state.mode {
        AppMode::Normal => get_normal_mode_help(),
        AppMode::Edit => get_edit_mode_help(),
        AppMode::Search => get_search_mode_help(),
        AppMode::Confirm => get_confirm_mode_help(),
        AppMode::FormInput => get_form_input_mode_help(),
        AppMode::GenreSelect => get_genre_select_mode_help(),
        AppMode::YearSelect => get_year_select_mode_help(),
    };

    let help_text = Paragraph::new(help_content)
        .block(Block::default().borders(Borders::ALL).title("키 바인딩"))
        .style(Style::default());

    f.render_widget(help_text, chunks[1]);
}

/// Normal 모드 도움말을 반환합니다
fn get_normal_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "📚 도서 관리",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  a",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  도서 추가"),
        ]),
        Line::from(vec![
            Span::styled(
                "  e",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  선택한 도서 편집"),
        ]),
        Line::from(vec![
            Span::styled(
                "  d",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  선택한 도서 삭제"),
        ]),
        Line::from(vec![
            Span::styled(
                "  v",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  리뷰 작성/보기"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "🔍 탐색 및 검색",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  j",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  아래로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  k",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  위로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  /",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  도서 검색"),
        ]),
        Line::from(vec![
            Span::styled(
                "  r",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  리포트 보기"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "⚙️  시스템",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  ?",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  이 도움말 보기"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  뒤로가기"),
        ]),
        Line::from(vec![
            Span::styled(
                "  q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  프로그램 종료"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "💡 팁: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("해피해킹 키보드에 최적화된 키 배치입니다!"),
        ]),
    ]
}

/// YearSelect 모드 도움말을 반환합니다
fn get_year_select_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "📅 출간년도 선택 모드",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  j",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  아래로 이동 (더 오래된 년도)"),
        ]),
        Line::from(vec![
            Span::styled(
                "  k",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  위로 이동 (더 최근 년도)"),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↑/↓",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  화살표 키로도 이동 가능"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  선택한 년도 적용"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  취소하고 폼 입력 모드로 돌아가기"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "📋 선택 가능한 년도:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("  • 현재 년도부터 1900년까지"),
        Line::from("  • 스크롤하여 원하는 년도 찾기"),
        Line::from("  • 선택된 항목은 ● 표시"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "💡 팁: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("j/k 키로 빠르게 이동할 수 있습니다!"),
        ]),
    ]
}

/// FormInput 모드 도움말을 반환합니다
fn get_form_input_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "📝 폼 입력 모드",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Tab",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  다음 필드로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Shift+Tab",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  이전 필드로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  장르 필드: 선택 모드 | 다른 필드: 편집 모드"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+S",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  현재 필드 저장"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  Normal 모드로 돌아가기"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "💡 팁: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("바로 타이핑하면 입력됩니다!"),
        ]),
    ]
}

/// GenreSelect 모드 도움말을 반환합니다
fn get_genre_select_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "🎭 장르 선택 모드",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  j / ↓",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  아래로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  k / ↑",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  위로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  선택된 장르 적용"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Esc",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  폼 입력 모드로 돌아가기"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "💡 팁: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("미리 정의된 장르 중에서 선택하세요!"),
        ]),
    ]
}

/// Edit 모드 도움말을 반환합니다
fn get_edit_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "✏️  편집 모드",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Ctrl+S",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  저장하고 편집 모드 종료"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+X",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  취소하고 편집 모드 종료"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+Q",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  강제 종료"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "📝 텍스트 편집",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  새 줄 추가"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Backspace",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  문자 삭제"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+A",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  줄 시작으로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+E",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  줄 끝으로 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+U",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  커서부터 줄 시작까지 삭제"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+K",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  커서부터 줄 끝까지 삭제"),
        ]),
        Line::from(vec![
            Span::styled(
                "  Ctrl+W",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  이전 단어 삭제"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "⚠️  주의: ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw("편집 모드에서는 모든 전역 키(q, j, k 등)가 텍스트로 입력됩니다."),
        ]),
    ]
}

/// Search 모드 도움말을 반환합니다
fn get_search_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "🔍 검색 모드",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("검색어를 입력하고 Enter를 누르세요."),
        Line::from("Esc를 누르면 검색을 취소합니다."),
    ]
}

/// Confirm 모드 도움말을 반환합니다
fn get_confirm_mode_help() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "❓ 확인 모드",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("y: 예, n: 아니오"),
    ]
}
