//! 검색 화면 UI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::tui::input::TextInput;
use crate::tui::state::{AppMode, AppState};
use crate::tui::ui::book_list;

/// 검색 화면을 렌더링합니다
pub fn render_search(f: &mut Frame, area: Rect, state: &AppState, text_input: &mut TextInput) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 헤더
            Constraint::Length(3), // 검색 입력
            Constraint::Min(0),    // 검색 결과
            Constraint::Length(3), // 상태바
        ])
        .split(area);

    // 헤더
    render_header(f, chunks[0]);

    // 검색 입력 필드
    if state.mode == AppMode::Search {
        render_search_input(f, chunks[1], text_input);
    } else {
        render_search_query_display(f, chunks[1], state);
    }

    // 검색 결과
    render_search_results(f, chunks[2], state);

    // 상태바
    render_status_bar(f, chunks[3], state);
}

/// 헤더를 렌더링합니다
fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("🔍 도서 검색")
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(header, area);
}

/// 검색 입력 필드를 렌더링합니다 (Search 모드)
fn render_search_input(f: &mut Frame, area: Rect, text_input: &mut TextInput) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("🔍 검색어 입력 (Enter: 검색, Esc: 취소)")
        .style(Style::default().fg(Color::Green));

    text_input.render(f, area, block, true);
}

/// 검색어 표시 (Normal 모드)
fn render_search_query_display(f: &mut Frame, area: Rect, state: &AppState) {
    let query_text = if state.search_query.is_empty() {
        "검색어를 입력하려면 '/' 키를 누르세요".to_string()
    } else {
        format!("검색어: \"{}\"", state.search_query)
    };

    let search_display = Paragraph::new(query_text)
        .block(Block::default().borders(Borders::ALL).title("검색어"))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(search_display, area);
}

/// 검색 결과를 렌더링합니다
fn render_search_results(f: &mut Frame, area: Rect, state: &AppState) {
    if state.search_query.is_empty() {
        // 검색어가 없는 경우
        let content = Paragraph::new(
            "검색어를 입력하면 결과가 여기에 표시됩니다.\n\n'/' 키를 눌러 검색을 시작하세요!",
        )
        .block(Block::default().borders(Borders::ALL).title("검색 결과"))
        .style(Style::default().fg(Color::Gray));
        f.render_widget(content, area);
        return;
    }

    // 검색 결과 필터링
    let search_results: Vec<(usize, &crate::lib::models::ExtendedBook)> = state
        .books
        .iter()
        .enumerate()
        .filter(|(_, book)| {
            let query = state.search_query.to_lowercase();
            book.book.title.to_lowercase().contains(&query)
                || book
                    .authors
                    .iter()
                    .any(|author| author.name.to_lowercase().contains(&query))
                || book.book.genre.to_lowercase().contains(&query)
                || book
                    .reviews
                    .iter()
                    .any(|review| review.review.to_lowercase().contains(&query))
        })
        .collect();

    if search_results.is_empty() {
        // 검색 결과가 없는 경우
        let content = Paragraph::new(format!(
            "\"{}\"에 대한 검색 결과가 없습니다.\n\n다른 검색어를 시도해보세요.",
            state.search_query
        ))
        .block(Block::default().borders(Borders::ALL).title("검색 결과"))
        .style(Style::default().fg(Color::Red));
        f.render_widget(content, area);
        return;
    }

    // 검색 결과 목록 생성 (도서 목록과 동일한 함수 사용)
    let result_items: Vec<ListItem> = search_results
        .iter()
        .enumerate()
        .map(|(i, (_, book))| {
            let is_selected = i == state.search_selected_index;
            book_list::create_book_item(book, is_selected)
        })
        .collect();

    let results_list = List::new(result_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("🔍 검색 결과 ({} 개)", search_results.len())),
        )
        .style(Style::default())
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    // 선택된 검색 결과 하이라이트
    let mut list_state = ListState::default();
    if state.search_selected_index < search_results.len() {
        list_state.select(Some(state.search_selected_index));
    }

    f.render_stateful_widget(results_list, area, &mut list_state);
}

/// 상태바를 렌더링합니다
fn render_status_bar(f: &mut Frame, area: Rect, state: &AppState) {
    let status_text = match state.mode {
        AppMode::Search => "💡 검색 모드: 검색어를 입력하고 Enter를 누르세요",
        AppMode::Normal => {
            if state.search_query.is_empty() {
                "💡 '/' 키를 눌러 검색을 시작하세요"
            } else {
                "💡 Enter로 선택, '/' 키로 새 검색, Esc로 돌아가기"
            }
        }
        _ => "검색 화면",
    };

    let status_style = match state.mode {
        AppMode::Search => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::Cyan),
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(status_style);

    f.render_widget(status, area);
}
