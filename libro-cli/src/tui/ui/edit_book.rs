//! 도서 편집 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    lib::models::ExtendedBook,
    tui::{
        input::TextInput,
        state::{AppMode, AppState},
    },
};

/// 도서 편집 화면을 렌더링합니다
pub fn render_edit_book(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Min(0),    // 폼 내용
            Constraint::Length(6), // 도움말
        ])
        .split(area);

    // 헤더
    render_header(f, chunks[0], state);

    // 폼 내용
    match state.mode {
        AppMode::GenreSelect => render_genre_selector(f, chunks[1], state),
        AppMode::YearSelect => render_year_selector(f, chunks[1], state),
        _ => render_form(f, chunks[1], state, text_input),
    }

    // 도움말
    render_form_help(f, chunks[2], state);
}

/// 헤더를 렌더링합니다
fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let book_title = if let Some(book) = state.books.get(state.selected_book_index) {
        &book.book.title
    } else {
        "알 수 없음"
    };

    let header_text = format!("📝 도서 편집: {}", book_title);

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(header, area);
}

/// 폼을 렌더링합니다
fn render_form(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // 제목
            Constraint::Length(4), // 저자
            Constraint::Length(4), // 번역자
            Constraint::Length(4), // 장르
            Constraint::Length(4), // 페이지
            Constraint::Length(4), // 출간년도
            Constraint::Min(1),    // 여백
        ])
        .split(area);

    // 각 필드 렌더링
    render_title_field(f, chunks[0], state, text_input, state.form_field_index == 0);
    render_authors_field(f, chunks[1], state, text_input, state.form_field_index == 1);
    render_translators_field(f, chunks[2], state, text_input, state.form_field_index == 2);
    render_genre_field(f, chunks[3], state, text_input, state.form_field_index == 3);
    render_pages_field(f, chunks[4], state, text_input, state.form_field_index == 4);
    render_year_field(f, chunks[5], state, text_input, state.form_field_index == 5);
}

/// 제목 필드를 렌더링합니다
fn render_title_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    text_input: &mut TextInput,
    is_selected: bool,
) {
    let title = if is_selected && state.mode == AppMode::FormInput {
        format!("제목 * (편집 중)")
    } else if is_selected {
        format!("제목 * (선택됨)")
    } else {
        "제목 *".to_string()
    };

    let content =
        if is_selected && (state.mode == AppMode::FormInput || state.mode == AppMode::Edit) {
            text_input.get_text()
        } else {
            state.form_title.clone()
        };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let field = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: false });

    f.render_widget(field, area);
}

/// 저자 필드를 렌더링합니다
fn render_authors_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    text_input: &mut TextInput,
    is_selected: bool,
) {
    let title = if is_selected && state.mode == AppMode::FormInput {
        format!("저자 * (편집 중) - 쉼표로 구분")
    } else if is_selected {
        format!("저자 * (선택됨) - 쉼표로 구분")
    } else {
        "저자 * - 쉼표로 구분".to_string()
    };

    let content =
        if is_selected && (state.mode == AppMode::FormInput || state.mode == AppMode::Edit) {
            text_input.get_text()
        } else {
            state.form_authors.clone()
        };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let field = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: false });

    f.render_widget(field, area);
}

/// 번역자 필드를 렌더링합니다
fn render_translators_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    text_input: &mut TextInput,
    is_selected: bool,
) {
    let title = if is_selected && state.mode == AppMode::FormInput {
        format!("번역자 (편집 중) - 쉼표로 구분")
    } else if is_selected {
        format!("번역자 (선택됨) - 쉼표로 구분")
    } else {
        "번역자 - 쉼표로 구분".to_string()
    };

    let content =
        if is_selected && (state.mode == AppMode::FormInput || state.mode == AppMode::Edit) {
            text_input.get_text()
        } else {
            state.form_translators.clone()
        };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let field = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: false });

    f.render_widget(field, area);
}

/// 장르 필드를 렌더링합니다
fn render_genre_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    text_input: &mut TextInput,
    is_selected: bool,
) {
    let title = if is_selected && state.mode == AppMode::FormInput {
        format!("장르 * (편집 중) - Enter로 선택")
    } else if is_selected {
        format!("장르 * (선택됨) - Enter로 선택")
    } else {
        "장르 * - Enter로 선택".to_string()
    };

    let content = if is_selected && state.mode == AppMode::FormInput {
        text_input.get_text()
    } else {
        state.form_genre.clone()
    };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let field = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: false });

    f.render_widget(field, area);
}

/// 페이지 필드를 렌더링합니다
fn render_pages_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    text_input: &mut TextInput,
    is_selected: bool,
) {
    let title = if is_selected && state.mode == AppMode::FormInput {
        format!("페이지 수 (편집 중)")
    } else if is_selected {
        format!("페이지 수 (선택됨)")
    } else {
        "페이지 수".to_string()
    };

    let content =
        if is_selected && (state.mode == AppMode::FormInput || state.mode == AppMode::Edit) {
            text_input.get_text()
        } else {
            state.form_pages.clone()
        };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let field = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: false });

    f.render_widget(field, area);
}

/// 출간년도 필드를 렌더링합니다
fn render_year_field(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    text_input: &mut TextInput,
    is_selected: bool,
) {
    let title = if is_selected && state.mode == AppMode::FormInput {
        format!("출간년도 (편집 중) - Enter로 선택")
    } else if is_selected {
        format!("출간년도 (선택됨) - Enter로 선택")
    } else {
        "출간년도 - Enter로 선택".to_string()
    };

    let content = if is_selected && state.mode == AppMode::FormInput {
        text_input.get_text()
    } else {
        state.form_pub_year.clone()
    };

    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let field = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(style)
        .wrap(Wrap { trim: false });

    f.render_widget(field, area);
}

/// 장르 선택기를 렌더링합니다
fn render_genre_selector(f: &mut Frame, area: Rect, state: &AppState) {
    let genres = AppState::get_genres();
    let items: Vec<ListItem> = genres
        .iter()
        .enumerate()
        .map(|(i, genre)| {
            let style = if i == state.genre_selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(*genre).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("장르 선택 (j/k: 이동, Enter: 선택, Esc: 취소)"),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

/// 출간년도 선택기를 렌더링합니다
fn render_year_selector(f: &mut Frame, area: Rect, state: &AppState) {
    let years = AppState::get_years();
    let visible_range = 10;
    let start_idx = if state.year_selected_index >= visible_range / 2 {
        std::cmp::min(
            state.year_selected_index - visible_range / 2,
            years.len().saturating_sub(visible_range),
        )
    } else {
        0
    };
    let end_idx = std::cmp::min(start_idx + visible_range, years.len());

    let items: Vec<ListItem> = years[start_idx..end_idx]
        .iter()
        .enumerate()
        .map(|(i, year)| {
            let actual_index = start_idx + i;
            let style = if actual_index == state.year_selected_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(year.to_string()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("출간년도 선택 (j/k: 이동, Enter: 선택, Esc: 취소)"),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

/// 폼 도움말을 렌더링합니다
fn render_form_help(f: &mut Frame, area: Rect, _state: &AppState) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("Tab/Shift+Tab", Style::default().fg(Color::Green)),
            Span::raw(": 필드 이동  "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(": 편집 시작  "),
            Span::styled("Ctrl+S", Style::default().fg(Color::Green)),
            Span::raw(": 저장 후 나가기"),
        ]),
        Line::from(vec![
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(": 취소  "),
            Span::styled("j/k", Style::default().fg(Color::Green)),
            Span::raw(": 선택 모드에서 이동  "),
            Span::styled("*", Style::default().fg(Color::Red)),
            Span::raw(" 표시는 필수 항목"),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("도움말"))
        .style(Style::default().fg(Color::Gray))
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}
