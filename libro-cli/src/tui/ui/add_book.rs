//! 도서 추가 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::input::TextInput;
use crate::tui::state::{AppMode, AppState};

/// 도서 추가 화면을 렌더링합니다
pub fn render_add_book(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 폼 영역
            Constraint::Length(3), // 상태바
        ])
        .split(area);

    // 헤더
    render_header(f, chunks[0]);

    // 폼 영역
    match state.mode {
        AppMode::Edit => render_form_edit(f, chunks[1], state, text_input),
        AppMode::FormInput => render_form_input(f, chunks[1], state, text_input),
        AppMode::GenreSelect => render_genre_select(f, chunks[1], state),
        AppMode::YearSelect => render_year_select(f, chunks[1], state),
        _ => render_form_display(f, chunks[1], state),
    }

    // 상태바
    render_status_bar(f, chunks[2], state);
}

/// 헤더를 렌더링합니다
fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("📚 새 도서 추가")
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(header, area);
}

/// 폼 편집 모드를 렌더링합니다 (제자리 편집)
fn render_form_edit(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 제목
            Constraint::Length(3), // 저자
            Constraint::Length(3), // 번역자
            Constraint::Length(3), // 장르
            Constraint::Length(3), // 페이지
            Constraint::Length(3), // 출간년도
            Constraint::Min(0),    // 도움말/여백
        ])
        .split(area);

    // 모든 필드를 렌더링 (현재 선택된 필드는 편집 가능하게)
    for i in 0..6 {
        let (title, value, required) = match i {
            0 => ("📖 제목", state.form_title.as_str(), true),
            1 => ("✍️ 저자", state.form_authors.as_str(), true),
            2 => ("🌐 번역자", state.form_translators.as_str(), false),
            3 => ("🎭 장르", state.form_genre.as_str(), true),
            4 => ("📄 페이지", state.form_pages.as_str(), false),
            5 => ("📅 출간년도", state.form_pub_year.as_str(), false),
            _ => ("", "", false),
        };

        if i == state.form_field_index {
            // 현재 선택된 필드는 편집 가능하게 렌더링 (Edit 모드용)
            render_form_field_editable_edit_mode(
                f,
                form_chunks[i],
                title,
                state,
                text_input,
                required,
            );
        } else {
            // 다른 필드들은 읽기 전용으로 렌더링
            render_form_field_readonly(f, form_chunks[i], title, value, required);
        }
    }

    // 도움말 영역
    render_edit_mode_help(f, form_chunks[6]);
}

/// 폼 직접 입력 모드를 렌더링합니다
fn render_form_input(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 제목
            Constraint::Length(3), // 저자
            Constraint::Length(3), // 번역자
            Constraint::Length(3), // 장르
            Constraint::Length(3), // 페이지
            Constraint::Length(3), // 출간년도
            Constraint::Min(0),    // 도움말/여백
        ])
        .split(area);

    // 모든 필드를 렌더링 (현재 선택된 필드는 편집 가능하게)
    for i in 0..6 {
        let (title, value, required) = match i {
            0 => ("📖 제목", state.form_title.as_str(), true),
            1 => ("✍️ 저자", state.form_authors.as_str(), true),
            2 => ("🌐 번역자", state.form_translators.as_str(), false),
            3 => ("🎭 장르", state.form_genre.as_str(), true),
            4 => ("📄 페이지", state.form_pages.as_str(), false),
            5 => ("📅 출간년도", state.form_pub_year.as_str(), false),
            _ => ("", "", false),
        };

        if i == state.form_field_index {
            // 현재 선택된 필드는 편집 가능하게 렌더링
            render_form_field_editable(f, form_chunks[i], title, state, text_input, required);
        } else {
            // 다른 필드들은 읽기 전용으로 렌더링
            render_form_field_readonly(f, form_chunks[i], title, value, required);
        }
    }

    // 도움말 영역
    render_form_help(f, form_chunks[6], state);
}

/// 장르 선택 모드를 렌더링합니다
fn render_genre_select(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // 헤더
            Constraint::Length(12), // 장르 목록 (5개 장르 + 여백)
            Constraint::Length(3),  // 현재 선택
            Constraint::Min(0),     // 도움말
        ])
        .split(area);

    // 헤더
    let header = Paragraph::new("🎭 장르를 선택해주세요")
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(header, chunks[0]);

    // 장르 목록 - 더 큰 박스로 표시
    let genres = AppState::get_genres();
    let mut genre_lines = vec![Line::from("")]; // 상단 여백

    for (i, genre) in genres.iter().enumerate() {
        if i == state.genre_selected_index {
            genre_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "●",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{}", genre),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ← 선택됨", Style::default().fg(Color::Green)),
            ]));
        } else {
            genre_lines.push(Line::from(vec![
                Span::raw("  ○ "),
                Span::styled(format!("{}", genre), Style::default().fg(Color::White)),
            ]));
        }
    }

    genre_lines.push(Line::from("")); // 하단 여백

    let genre_list = Paragraph::new(genre_lines)
        .block(Block::default().borders(Borders::ALL).title("📚 장르 목록"))
        .style(Style::default().fg(Color::White));
    f.render_widget(genre_list, chunks[1]);

    // 현재 선택된 장르 표시
    let selected_genre = state.get_selected_genre();
    let current_selection = Paragraph::new(format!("현재 선택: {}", selected_genre))
        .block(Block::default().borders(Borders::ALL).title("선택된 장르"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(current_selection, chunks[2]);

    // 도움말
    let help_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "j/k",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" 또는 "),
            Span::styled(
                "↑/↓",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": 위아래 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": 선택한 장르 적용"),
        ]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(": 취소하고 돌아가기"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "💡 팁: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("한국어로 된 간단한 장르 분류입니다!"),
        ]),
    ];

    let help = Paragraph::new(help_lines)
        .block(Block::default().borders(Borders::ALL).title("사용법"))
        .style(Style::default().fg(Color::White));
    f.render_widget(help, chunks[3]);
}

/// 출간년도 선택 모드를 렌더링합니다
fn render_year_select(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 년도 목록 (스크롤 가능)
            Constraint::Length(3), // 현재 선택
            Constraint::Length(8), // 도움말
        ])
        .split(area);

    // 헤더
    let header = Paragraph::new("📅 출간년도를 선택해주세요")
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(header, chunks[0]);

    // 년도 목록 - 스크롤 가능한 리스트
    let years = AppState::get_years();
    let list_height = chunks[1].height as usize - 2; // 테두리 제외
    let start_index = if state.year_selected_index >= list_height / 2 {
        (state.year_selected_index + 1).saturating_sub(list_height / 2)
    } else {
        0
    };
    let end_index = (start_index + list_height).min(years.len());

    let mut year_lines = vec![Line::from("")]; // 상단 여백

    for (i, year) in years[start_index..end_index].iter().enumerate() {
        let actual_index = start_index + i;
        if actual_index == state.year_selected_index {
            year_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "●",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
                Span::styled(
                    format!("{}", year),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ← 선택됨", Style::default().fg(Color::Green)),
            ]));
        } else {
            year_lines.push(Line::from(vec![
                Span::raw("  ○ "),
                Span::styled(format!("{}", year), Style::default().fg(Color::White)),
            ]));
        }
    }

    if end_index < years.len() {
        year_lines.push(Line::from(vec![Span::styled(
            "  ... (더 많은 년도)",
            Style::default().fg(Color::Gray),
        )]));
    }

    let year_list = Paragraph::new(year_lines)
        .block(Block::default().borders(Borders::ALL).title("📅 년도 목록"))
        .style(Style::default().fg(Color::White));
    f.render_widget(year_list, chunks[1]);

    // 현재 선택된 년도 표시
    let selected_year = state.get_selected_year();
    let current_selection = Paragraph::new(format!("현재 선택: {}년", selected_year))
        .block(Block::default().borders(Borders::ALL).title("선택된 년도"))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(current_selection, chunks[2]);

    // 도움말
    let help_lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "j/k",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" 또는 "),
            Span::styled(
                "↑/↓",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": 위아래 이동"),
        ]),
        Line::from(vec![
            Span::styled(
                "Enter",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": 선택한 년도 적용"),
        ]),
        Line::from(vec![
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(": 취소하고 돌아가기"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "💡 팁: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("현재 년도부터 1900년까지 선택 가능합니다!"),
        ]),
    ];

    let help = Paragraph::new(help_lines)
        .block(Block::default().borders(Borders::ALL).title("사용법"))
        .style(Style::default().fg(Color::White));
    f.render_widget(help, chunks[3]);
}

/// 폼 표시 모드를 렌더링합니다 (Normal 모드)
fn render_form_display(f: &mut Frame, area: Rect, state: &AppState) {
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 제목
            Constraint::Length(3), // 저자
            Constraint::Length(3), // 번역자
            Constraint::Length(3), // 장르
            Constraint::Length(3), // 페이지
            Constraint::Length(3), // 출간년도
            Constraint::Min(0),    // 설명
        ])
        .split(area);

    // 각 필드를 렌더링 (현재 선택된 필드 강조)
    for i in 0..6 {
        let (title, value, required) = match i {
            0 => ("📖 제목", state.form_title.as_str(), true),
            1 => ("✍️ 저자", state.form_authors.as_str(), true),
            2 => ("🌐 번역자", state.form_translators.as_str(), false),
            3 => ("🎭 장르", state.form_genre.as_str(), true),
            4 => ("📄 페이지", state.form_pages.as_str(), false),
            5 => ("📅 출간년도", state.form_pub_year.as_str(), false),
            _ => ("", "", false),
        };

        render_form_field(f, form_chunks[i], state, i, title, value, required);
    }

    // 설명
    let instructions = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "💡 사용법:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("• 바로 타이핑하여 입력 시작"),
        Line::from("• Enter: 현재 필드 편집 시작"),
        Line::from("• Tab/Shift+Tab: 다음/이전 필드로 이동"),
        Line::from("• Ctrl+S: 도서 저장하고 나가기"),
        Line::from("• Esc: 이전 화면으로 돌아가기"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "⚠️ 주의:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from("제목, 저자, 장르는 필수 입력 항목입니다."),
    ];

    let help_text = Paragraph::new(instructions)
        .block(Block::default().borders(Borders::ALL).title("도움말"))
        .style(Style::default().fg(Color::White));
    f.render_widget(help_text, form_chunks[6]);
}

/// 개별 폼 필드를 렌더링합니다
fn render_form_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    field_index: usize,
    title: &str,
    value: &str,
    required: bool,
) {
    let is_selected = state.form_field_index == field_index;
    let is_empty = value.trim().is_empty();

    let display_text = if is_empty {
        if required {
            "(필수 입력 항목)".to_string()
        } else {
            "(선택사항)".to_string()
        }
    } else {
        value.to_string()
    };

    let style = if is_selected {
        if required && is_empty {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    } else if required && is_empty {
        Style::default().fg(Color::Red)
    } else if is_empty {
        Style::default().fg(Color::Gray)
    } else {
        Style::default().fg(Color::Green)
    };

    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let field = Paragraph::new(display_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(border_style),
        )
        .style(style);

    f.render_widget(field, area);
}

/// 편집 가능한 폼 필드를 렌더링합니다 (FormInput 모드용)
fn render_form_field_editable(
    f: &mut Frame,
    area: Rect,
    title: &str,
    state: &AppState,
    text_input: &mut TextInput,
    required: bool,
) {
    let border_style = Style::default().fg(Color::Yellow);
    let title_with_indicator = format!("📝 {} (편집 중)", title);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title_with_indicator)
        .style(border_style);

    text_input.render(f, area, block, true);
}

/// 편집 가능한 폼 필드를 렌더링합니다 (Edit 모드용)
fn render_form_field_editable_edit_mode(
    f: &mut Frame,
    area: Rect,
    title: &str,
    state: &AppState,
    text_input: &mut TextInput,
    required: bool,
) {
    let border_style = Style::default().fg(Color::Red);
    let title_with_indicator = format!("✏️ {} (Ctrl+S: 저장, Ctrl+X: 취소)", title);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title_with_indicator)
        .style(border_style);

    text_input.render(f, area, block, true);
}

/// Edit 모드 도움말을 렌더링합니다
fn render_edit_mode_help(f: &mut Frame, area: Rect) {
    let help_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "✏️ 편집 모드:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("• 자유롭게 텍스트 편집 가능"),
        Line::from("• Enter: 새 줄 추가"),
        Line::from("• Ctrl+S: 저장하고 편집 모드 종료"),
        Line::from("• Ctrl+X: 취소하고 편집 모드 종료"),
        Line::from("• Ctrl+A/E: 줄 시작/끝으로 이동"),
        Line::from("• Ctrl+U/K: 줄 시작까지/끝까지 삭제"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "⚠️ 주의:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("편집 모드에서는 모든 전역 키가 텍스트로 입력됩니다."),
    ];

    let help_text = Paragraph::new(help_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("편집 모드 도움말"),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(help_text, area);
}

/// 폼 도움말을 렌더링합니다
fn render_form_help(f: &mut Frame, area: Rect, state: &AppState) {
    let help_lines = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "💡 사용법:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("• 바로 타이핑하여 입력"),
        Line::from("• Tab/Shift+Tab: 다음/이전 필드로 이동"),
        Line::from("• Enter: 장르/년도 필드는 선택 모드, 다른 필드는 편집 모드"),
        Line::from("• Ctrl+S: 도서 저장하고 나가기"),
        Line::from("• Esc: 이전 화면으로 돌아가기"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "⚠️ 주의:",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )]),
        Line::from("제목, 저자, 장르는 필수 입력 항목입니다."),
    ];

    let help_text = Paragraph::new(help_lines)
        .block(Block::default().borders(Borders::ALL).title("도움말"))
        .style(Style::default().fg(Color::White));
    f.render_widget(help_text, area);
}

/// 읽기 전용 폼 필드를 렌더링합니다
fn render_form_field_readonly(f: &mut Frame, area: Rect, title: &str, value: &str, required: bool) {
    let is_empty = value.trim().is_empty();

    let display_text = if is_empty {
        if required {
            "(필수 입력 항목)".to_string()
        } else {
            "(선택사항)".to_string()
        }
    } else {
        value.to_string()
    };

    let style = if required && is_empty {
        Style::default().fg(Color::Red)
    } else if is_empty {
        Style::default().fg(Color::Gray)
    } else {
        Style::default().fg(Color::Green)
    };

    let field = Paragraph::new(display_text)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style);

    f.render_widget(field, area);
}

/// 상태바를 렌더링합니다
fn render_status_bar(f: &mut Frame, area: Rect, state: &AppState) {
    let status_text = match state.mode {
        AppMode::Edit => {
            let field_name = state.get_current_form_field_name();
            format!("💡 {} 편집 중: Ctrl+S로 저장, Ctrl+X로 취소", field_name)
        }
        AppMode::FormInput => {
            let field_name = state.get_current_form_field_name();
            format!(
                "💡 {} 입력 중: Tab으로 다음 필드, Ctrl+S로 저장",
                field_name
            )
        }
        AppMode::GenreSelect => "💡 장르 선택: j/k로 이동, Enter로 선택, Esc로 취소".to_string(),
        AppMode::YearSelect => "💡 년도 선택: j/k로 이동, Enter로 선택, Esc로 취소".to_string(),
        AppMode::Normal => {
            let current_field = state.get_current_form_field_name();
            format!(
                "💡 현재 필드: {} | 바로 입력 가능 | Tab: 다음 필드 | Ctrl+S: 저장 | Esc: 돌아가기",
                current_field
            )
        }
        _ => "도서 추가 화면".to_string(),
    };

    let status_style = match state.mode {
        AppMode::Edit | AppMode::FormInput => Style::default().fg(Color::Yellow),
        AppMode::GenreSelect | AppMode::YearSelect => Style::default().fg(Color::Green),
        _ => Style::default().fg(Color::Cyan),
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(status_style);

    f.render_widget(status, area);
}
