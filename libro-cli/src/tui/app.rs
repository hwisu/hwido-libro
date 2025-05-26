//! 메인 애플리케이션 구조

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::time::Duration;

use crate::{
    lib::{db_operations::Database, models::BookFilter},
    tui::{
        events::{key_to_action, AppEvent, EventHandler, KeyAction},
        input::TextInput,
        state::{AppMode, AppState, Screen},
        ui::{add_book, book_list, help, review, search},
    },
};

pub struct App {
    state: AppState,
    event_handler: EventHandler,
    text_input: TextInput,
    database: Database,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database = Database::new("libro.db")?;
        let mut app = Self {
            state: AppState::new(),
            event_handler: EventHandler::new(Duration::from_millis(100)),
            text_input: TextInput::new(),
            database,
        };

        // 초기 도서 목록 로드
        app.load_books()?;

        Ok(app)
    }

    /// 데이터베이스에서 도서 목록을 로드합니다
    fn load_books(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = BookFilter::default();
        match self.database.get_books(&filter) {
            Ok(books) => {
                self.state.books = books;
                self.state.error_message = None;

                // 선택된 인덱스가 범위를 벗어나면 조정
                if self.state.selected_book_index >= self.state.books.len()
                    && !self.state.books.is_empty()
                {
                    self.state.selected_book_index = self.state.books.len() - 1;
                } else if self.state.books.is_empty() {
                    self.state.selected_book_index = 0;
                }
            }
            Err(e) => {
                self.state.error_message = Some(format!("도서 로드 실패: {}", e));
            }
        }
        Ok(())
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            // UI 렌더링
            terminal.draw(|f| self.render(f))?;

            // 이벤트 처리
            match self.event_handler.next()? {
                AppEvent::Key(key) => {
                    let action = key_to_action(key, &self.state.mode);
                    self.handle_action(action)?;
                }
                AppEvent::Resize(_, _) => {
                    // 터미널 크기 변경 시 다시 그리기
                }
                AppEvent::Tick => {
                    // 메시지 자동 삭제 처리
                    self.state.clear_expired_message();
                }
            }

            if self.state.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // 헤더
                Constraint::Min(0),    // 메인 컨텐츠
                Constraint::Length(3), // 푸터
            ])
            .split(f.size());

        // 헤더 렌더링
        self.render_header(f, chunks[0]);

        // 메인 컨텐츠 렌더링
        match self.state.current_screen {
            Screen::BookList => book_list::render_book_list(f, chunks[1], &self.state),
            Screen::Help => help::render_help(f, chunks[1], &self.state),
            Screen::Review => {
                review::render_review(f, chunks[1], &self.state, &mut self.text_input)
            }
            Screen::Search => {
                search::render_search(f, chunks[1], &self.state, &mut self.text_input)
            }
            Screen::AddBook => {
                add_book::render_add_book(f, chunks[1], &self.state, &mut self.text_input)
            }
            _ => self.render_placeholder(f, chunks[1], "Coming Soon"),
        }

        // 푸터 렌더링
        self.render_footer(f, chunks[2]);
    }

    fn render_header(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let mode_text = match self.state.mode {
            AppMode::Normal => "일반",
            AppMode::Edit => "편집",
            AppMode::Search => "검색",
            AppMode::Confirm => "확인",
            AppMode::FormInput => "폼입력",
            AppMode::GenreSelect => "장르선택",
            AppMode::YearSelect => "년도선택",
        };

        let screen_text = match self.state.current_screen {
            Screen::BookList => "도서 목록",
            Screen::Help => "도움말",
            Screen::Review => "리뷰",
            Screen::Search => "검색",
            Screen::AddBook => "도서 추가",
            _ => "기타",
        };

        let header_text = format!("📚 Libro TUI | 모드: {} | 화면: {}", mode_text, screen_text);

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));

        f.render_widget(header, area);
    }

    fn render_footer(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let footer_text = match self.state.mode {
            AppMode::Normal => "q: 종료 | ?: 도움말 | Esc: 뒤로가기",
            AppMode::Edit => "Ctrl+S: 저장 | Ctrl+X: 취소 | Ctrl+Q: 강제종료",
            AppMode::Search => "Enter: 검색 | Esc: 취소",
            AppMode::Confirm => "y: 예 | n: 아니오 | Esc: 취소",
            AppMode::FormInput => "Tab: 다음 필드 | Ctrl+S: 저장하고 나가기 | Esc: 취소",
            AppMode::GenreSelect => "j/k: 이동 | Enter: 선택 | Esc: 취소",
            AppMode::YearSelect => "j/k: 이동 | Enter: 선택 | Esc: 취소",
        };

        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));

        f.render_widget(footer, area);
    }

    fn render_placeholder(&self, f: &mut Frame, area: ratatui::layout::Rect, title: &str) {
        let placeholder = Paragraph::new(format!("🚧 {} 화면은 아직 구현되지 않았습니다", title))
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Yellow));

        f.render_widget(placeholder, area);
    }

    fn handle_action(&mut self, action: KeyAction) -> Result<(), Box<dyn std::error::Error>> {
        match action {
            KeyAction::Quit => {
                self.state.should_quit = true;
            }
            KeyAction::Help => {
                if self.state.current_screen == Screen::Help {
                    self.state.go_back();
                } else {
                    self.state.set_screen(Screen::Help);
                }
            }
            KeyAction::Back => {
                if self.state.mode == AppMode::Edit {
                    self.state.cancel_edit_mode();
                    self.state.editing_review_index = None;
                } else if self.state.mode == AppMode::Search {
                    // 검색 모드에서 Esc를 누르면 Normal 모드로 돌아가기
                    self.state.mode = AppMode::Normal;
                } else if self.state.mode == AppMode::FormInput {
                    // 폼 입력 모드에서 Esc를 누르면 현재 필드 값을 저장하고 Normal 모드로
                    let text = self.text_input.get_text();
                    self.state.set_current_form_field_value(text);
                    self.state.mode = AppMode::Normal;
                } else if self.state.mode == AppMode::GenreSelect {
                    // 장르 선택 모드에서 Esc를 누르면 FormInput 모드로 돌아가기
                    self.state.mode = AppMode::FormInput;
                    let current_value = self.state.get_current_form_field_value();
                    self.text_input = TextInput::with_text(current_value);
                } else if self.state.mode == AppMode::YearSelect {
                    // 출간년도 선택 모드에서 Esc를 누르면 FormInput 모드로 돌아가기
                    self.state.mode = AppMode::FormInput;
                    let current_value = self.state.get_current_form_field_value();
                    self.text_input = TextInput::with_text(current_value);
                } else {
                    // Normal 모드에서 Esc를 누르면 화면별 처리
                    match self.state.current_screen {
                        Screen::Search => {
                            // 검색 화면에서는 BookList로 돌아가기
                            self.state.current_screen = Screen::BookList;
                            self.state.search_query.clear(); // 검색어도 초기화
                            self.state.search_selected_index = 0;
                        }
                        Screen::AddBook => {
                            // 도서 추가 화면에서는 BookList로 돌아가기
                            self.state.current_screen = Screen::BookList;
                            self.state.clear_form(); // 폼 초기화
                            self.state.mode = AppMode::Normal; // Normal 모드로 복원
                        }
                        _ => {
                            self.state.go_back();
                        }
                    }
                }
            }
            KeyAction::AddBook => {
                if self.state.mode == AppMode::Normal {
                    self.state.set_screen(Screen::AddBook);
                    self.state.clear_form(); // 폼 초기화
                    self.state.mode = AppMode::FormInput; // 바로 입력 모드로 전환
                                                          // 첫 번째 필드(제목)의 현재 값으로 텍스트 입력 초기화
                    let current_value = self.state.get_current_form_field_value();
                    self.text_input = TextInput::with_text(current_value);
                }
            }
            KeyAction::Search => {
                if self.state.mode == AppMode::Normal {
                    self.state.set_screen(Screen::Search);
                    // 검색 모드로 전환하고 텍스트 입력 초기화
                    self.state.mode = AppMode::Search;
                    self.state.search_selected_index = 0; // 검색 선택 인덱스 초기화
                    self.text_input = TextInput::with_text(self.state.search_query.clone());
                }
            }
            KeyAction::Report => {
                if self.state.mode == AppMode::Normal {
                    self.state.set_screen(Screen::Report);
                }
            }
            KeyAction::AddReview => {
                if self.state.mode == AppMode::Normal {
                    if self.state.current_screen == Screen::Review {
                        // 리뷰 화면에서 v를 누르면 편집 모드로 전환
                        self.state.mode = AppMode::Edit;

                        // 선택된 도서의 기존 리뷰가 있으면 선택된 리뷰 내용을 로드하고 편집 인덱스 설정
                        let (initial_text, review_index) = if let Some(book) =
                            self.state.books.get(self.state.selected_book_index)
                        {
                            if !book.reviews.is_empty()
                                && self.state.selected_review_index < book.reviews.len()
                            {
                                // 기존 리뷰 편집 (선택된 리뷰)
                                (
                                    book.reviews[self.state.selected_review_index]
                                        .review
                                        .clone(),
                                    Some(self.state.selected_review_index),
                                )
                            } else {
                                // 새 리뷰 작성
                                (String::new(), None)
                            }
                        } else {
                            (String::new(), None)
                        };

                        self.state.editing_review_index = review_index;
                        self.text_input = TextInput::with_text(initial_text);
                    } else {
                        // 다른 화면에서 v를 누르면 리뷰 화면으로 이동
                        self.state.set_screen(Screen::Review);
                        // 리뷰 선택 인덱스 초기화
                        self.state.selected_review_index = 0;
                    }
                }
            }
            KeyAction::NewReview => {
                if self.state.mode == AppMode::Normal && self.state.current_screen == Screen::Review
                {
                    // 리뷰 화면에서 n을 누르면 새 리뷰 작성 모드로 전환
                    self.state.mode = AppMode::Edit;
                    self.state.editing_review_index = None; // 새 리뷰
                    self.text_input = TextInput::with_text(String::new());
                }
            }
            KeyAction::DeleteBook => {
                if self.state.mode == AppMode::Normal {
                    match self.state.current_screen {
                        Screen::Review => {
                            // 리뷰 화면에서 d 키는 리뷰 삭제
                            self.handle_delete_review();
                        }
                        Screen::BookList => {
                            // 도서 목록에서 d 키는 도서 삭제
                            self.handle_delete_book();
                        }
                        _ => {
                            // 다른 화면에서는 무시
                        }
                    }
                }
            }
            // 편집 모드 액션들
            KeyAction::SaveEdit => {
                match self.state.mode {
                    AppMode::Edit => {
                        match self.state.current_screen {
                            Screen::AddBook => {
                                // 도서 추가 화면에서 Ctrl+S: 현재 필드 저장 후 Normal 모드로
                                let text = self.text_input.get_text();
                                self.state.set_current_form_field_value(text);
                                self.state.mode = AppMode::Normal;
                            }
                            Screen::Review => {
                                // 리뷰 화면에서 Ctrl+S: 리뷰 저장
                                let text = self.text_input.get_text();
                                self.state.mode = AppMode::Normal;
                                self.handle_save_review(text);
                            }
                            _ => {
                                // 다른 화면에서는 무시
                            }
                        }
                    }
                    AppMode::FormInput => {
                        if self.state.current_screen == Screen::AddBook {
                            // FormInput 모드에서 Ctrl+S: 현재 필드 저장 후 전체 도서 저장하고 나가기
                            let text = self.text_input.get_text();
                            self.state.set_current_form_field_value(text);
                            self.handle_save_book_and_exit();
                        }
                    }
                    AppMode::Normal => {
                        if self.state.current_screen == Screen::AddBook {
                            // Normal 모드에서 도서 추가 화면에서 Ctrl+S: 도서 저장하고 나가기
                            self.handle_save_book_and_exit();
                        }
                    }
                    _ => {}
                }
            }
            KeyAction::CancelEdit => {
                if self.state.mode == AppMode::Edit {
                    self.state.cancel_edit_mode();
                    match self.state.current_screen {
                        Screen::Review => {
                            self.state.editing_review_index = None;
                        }
                        _ => {}
                    }
                }
            }
            KeyAction::ForceQuit => {
                if self.state.mode == AppMode::Edit {
                    self.state.cancel_edit_mode();
                    self.state.editing_review_index = None;
                    self.state.should_quit = true;
                }
            }
            // 폼 네비게이션
            KeyAction::NextField => {
                if self.state.current_screen == Screen::AddBook {
                    match self.state.mode {
                        AppMode::FormInput => {
                            // 현재 필드 값 저장
                            let text = self.text_input.get_text();
                            self.state.set_current_form_field_value(text);
                            // 다음 필드로 이동
                            self.state.next_form_field();
                            // 새 필드 값으로 텍스트 입력 초기화
                            let current_value = self.state.get_current_form_field_value();
                            self.text_input = TextInput::with_text(current_value);
                        }
                        AppMode::Normal => {
                            self.state.next_form_field();
                            // FormInput 모드로 전환
                            self.state.mode = AppMode::FormInput;
                            let current_value = self.state.get_current_form_field_value();
                            self.text_input = TextInput::with_text(current_value);
                        }
                        _ => {}
                    }
                }
            }
            KeyAction::PrevField => {
                if self.state.current_screen == Screen::AddBook {
                    match self.state.mode {
                        AppMode::FormInput => {
                            // 현재 필드 값 저장
                            let text = self.text_input.get_text();
                            self.state.set_current_form_field_value(text);
                            // 이전 필드로 이동
                            self.state.prev_form_field();
                            // 새 필드 값으로 텍스트 입력 초기화
                            let current_value = self.state.get_current_form_field_value();
                            self.text_input = TextInput::with_text(current_value);
                        }
                        AppMode::Normal => {
                            self.state.prev_form_field();
                            // FormInput 모드로 전환
                            self.state.mode = AppMode::FormInput;
                            let current_value = self.state.get_current_form_field_value();
                            self.text_input = TextInput::with_text(current_value);
                        }
                        _ => {}
                    }
                }
            }
            // 텍스트 편집 액션들
            KeyAction::InsertChar(c) => {
                match self.state.mode {
                    AppMode::Edit | AppMode::Search => {
                        self.text_input.insert_char(c);
                    }
                    AppMode::FormInput => {
                        if self.state.current_screen == Screen::AddBook {
                            if self.state.is_genre_field() {
                                // 장르 필드에서는 문자 입력 시 장르 선택 모드로 전환
                                self.state.mode = AppMode::GenreSelect;
                                // 현재 장르 값에 맞는 인덱스 찾기
                                let genres = AppState::get_genres();
                                if let Some(index) =
                                    genres.iter().position(|&g| g == self.state.form_genre)
                                {
                                    self.state.genre_selected_index = index;
                                }
                            } else if self.state.is_year_field() {
                                // 출간년도 필드에서는 문자 입력 시 년도 선택 모드로 전환
                                self.state.mode = AppMode::YearSelect;
                                // 현재 년도 값에 맞는 인덱스 찾기
                                let years = AppState::get_years();
                                if let Ok(current_year) = self.state.form_pub_year.parse::<u32>() {
                                    if let Some(index) =
                                        years.iter().position(|&y| y == current_year)
                                    {
                                        self.state.year_selected_index = index;
                                    }
                                }
                            } else {
                                // 다른 필드에서는 정상적으로 문자 입력
                                self.text_input.insert_char(c);
                            }
                        }
                    }
                    AppMode::Normal => {
                        // Normal 모드에서 도서 추가 화면에서 문자를 입력하면 FormInput 모드로 전환
                        if self.state.current_screen == Screen::AddBook {
                            if self.state.is_genre_field() {
                                // 장르 필드에서는 문자 입력 시 장르 선택 모드로 전환
                                self.state.mode = AppMode::GenreSelect;
                                // 현재 장르 값에 맞는 인덱스 찾기
                                let genres = AppState::get_genres();
                                if let Some(index) =
                                    genres.iter().position(|&g| g == self.state.form_genre)
                                {
                                    self.state.genre_selected_index = index;
                                }
                            } else if self.state.is_year_field() {
                                // 출간년도 필드에서는 문자 입력 시 년도 선택 모드로 전환
                                self.state.mode = AppMode::YearSelect;
                                // 현재 년도 값에 맞는 인덱스 찾기
                                let years = AppState::get_years();
                                if let Ok(current_year) = self.state.form_pub_year.parse::<u32>() {
                                    if let Some(index) =
                                        years.iter().position(|&y| y == current_year)
                                    {
                                        self.state.year_selected_index = index;
                                    }
                                }
                            } else {
                                // 다른 필드에서는 FormInput 모드로 전환
                                self.state.mode = AppMode::FormInput;
                                let current_value = self.state.get_current_form_field_value();
                                self.text_input = TextInput::with_text(current_value);
                                self.text_input.insert_char(c);
                            }
                        }
                    }
                    _ => {}
                }
            }
            KeyAction::DeleteChar => match self.state.mode {
                AppMode::Edit | AppMode::Search => {
                    self.text_input.delete_char();
                }
                AppMode::FormInput => {
                    if !(self.state.current_screen == Screen::AddBook
                        && (self.state.is_genre_field() || self.state.is_year_field()))
                    {
                        self.text_input.delete_char();
                    }
                }
                _ => {}
            },
            KeyAction::Backspace => match self.state.mode {
                AppMode::Edit | AppMode::Search => {
                    self.text_input.backspace();
                }
                AppMode::FormInput => {
                    if !(self.state.current_screen == Screen::AddBook
                        && (self.state.is_genre_field() || self.state.is_year_field()))
                    {
                        self.text_input.backspace();
                    }
                }
                _ => {}
            },
            KeyAction::NewLine => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.insert_newline();
                }
            }
            // 커서 이동 (편집 모드)
            KeyAction::CursorLeft => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.move_cursor_left();
                }
            }
            KeyAction::CursorRight => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.move_cursor_right();
                }
            }
            KeyAction::CursorUp => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.move_cursor_up();
                }
            }
            KeyAction::CursorDown => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.move_cursor_down();
                }
            }
            // 라인 편집
            KeyAction::LineStart => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.move_to_line_start();
                }
            }
            KeyAction::LineEnd => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.move_to_line_end();
                }
            }
            KeyAction::ClearLine => match self.state.mode {
                AppMode::Edit | AppMode::Search => {
                    self.text_input.clear_current_line();
                }
                AppMode::FormInput => {
                    if !(self.state.current_screen == Screen::AddBook
                        && (self.state.is_genre_field() || self.state.is_year_field()))
                    {
                        self.text_input.clear_current_line();
                    }
                }
                _ => {}
            },
            KeyAction::DeleteToEnd => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.delete_to_line_end();
                }
            }
            KeyAction::DeleteWord => {
                if self.state.mode == AppMode::Edit {
                    self.text_input.delete_word_backward();
                }
            }
            // 네비게이션
            KeyAction::MoveUp => {
                if self.state.mode == AppMode::GenreSelect {
                    // 장르 선택 모드에서 위로 이동
                    self.state.move_genre_up();
                } else if self.state.mode == AppMode::YearSelect {
                    // 출간년도 선택 모드에서 위로 이동
                    self.state.move_year_up();
                } else if self.state.mode == AppMode::Normal {
                    match self.state.current_screen {
                        Screen::BookList => {
                            // 도서 목록에서 위로 이동 (k 키)
                            if self.state.selected_book_index > 0 {
                                self.state.selected_book_index -= 1;
                            }
                        }
                        Screen::Review => {
                            // 리뷰 화면에서 위로 이동 (k 키)
                            if let Some(book) = self.state.books.get(self.state.selected_book_index)
                            {
                                if !book.reviews.is_empty() && self.state.selected_review_index > 0
                                {
                                    self.state.selected_review_index -= 1;
                                }
                            }
                        }
                        Screen::Search => {
                            // 검색 화면에서 위로 이동 (k 키)
                            if self.state.search_selected_index > 0 {
                                self.state.search_selected_index -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyAction::MoveDown => {
                if self.state.mode == AppMode::GenreSelect {
                    // 장르 선택 모드에서 아래로 이동
                    self.state.move_genre_down();
                } else if self.state.mode == AppMode::YearSelect {
                    // 출간년도 선택 모드에서 아래로 이동
                    self.state.move_year_down();
                } else if self.state.mode == AppMode::Normal {
                    match self.state.current_screen {
                        Screen::BookList => {
                            // 도서 목록에서 아래로 이동 (j 키)
                            if self.state.selected_book_index + 1 < self.state.books.len() {
                                self.state.selected_book_index += 1;
                            }
                        }
                        Screen::Review => {
                            // 리뷰 화면에서 아래로 이동 (j 키)
                            if let Some(book) = self.state.books.get(self.state.selected_book_index)
                            {
                                if self.state.selected_review_index + 1 < book.reviews.len() {
                                    self.state.selected_review_index += 1;
                                }
                            }
                        }
                        Screen::Search => {
                            // 검색 화면에서 아래로 이동 (j 키)
                            // 검색 결과 개수를 계산해서 범위 체크
                            let search_result_count = if self.state.search_query.is_empty() {
                                0
                            } else {
                                self.state
                                    .books
                                    .iter()
                                    .filter(|book| {
                                        let query = self.state.search_query.to_lowercase();
                                        book.book.title.to_lowercase().contains(&query)
                                            || book.authors.iter().any(|author| {
                                                author.name.to_lowercase().contains(&query)
                                            })
                                            || book.book.genre.to_lowercase().contains(&query)
                                            || book.reviews.iter().any(|review| {
                                                review.review.to_lowercase().contains(&query)
                                            })
                                    })
                                    .count()
                            };

                            if self.state.search_selected_index + 1 < search_result_count {
                                self.state.search_selected_index += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            KeyAction::Select => {
                match self.state.mode {
                    AppMode::Search => {
                        // 검색 모드에서 Enter를 누르면 검색 실행
                        let query = self.text_input.get_text().trim().to_string();
                        self.state.search_query = query;
                        self.state.mode = AppMode::Normal;

                        // 검색 결과 선택 인덱스 초기화
                        self.state.search_selected_index = 0;

                        if !self.state.search_query.is_empty() {
                            self.state.set_message(format!(
                                "✅ \"{}\"에 대한 검색이 완료되었습니다",
                                self.state.search_query
                            ));
                        }
                    }
                    AppMode::FormInput => {
                        // FormInput 모드에서 Enter: 장르/년도 필드면 선택 모드로, 아니면 Edit 모드로
                        if self.state.current_screen == Screen::AddBook {
                            if self.state.is_genre_field() {
                                // 장르 필드에서 Enter를 누르면 장르 선택 모드로 전환
                                self.state.mode = AppMode::GenreSelect;
                                // 현재 장르 값에 맞는 인덱스 찾기
                                let genres = AppState::get_genres();
                                if let Some(index) =
                                    genres.iter().position(|&g| g == self.state.form_genre)
                                {
                                    self.state.genre_selected_index = index;
                                }
                            } else if self.state.is_year_field() {
                                // 출간년도 필드에서 Enter를 누르면 년도 선택 모드로 전환
                                self.state.mode = AppMode::YearSelect;
                                // 현재 년도 값에 맞는 인덱스 찾기
                                let years = AppState::get_years();
                                if let Ok(current_year) = self.state.form_pub_year.parse::<u32>() {
                                    if let Some(index) =
                                        years.iter().position(|&y| y == current_year)
                                    {
                                        self.state.year_selected_index = index;
                                    }
                                }
                            } else {
                                // 다른 필드에서는 Edit 모드로 전환
                                self.state.mode = AppMode::Edit;
                            }
                        }
                    }
                    AppMode::GenreSelect => {
                        // 장르 선택 모드에서 Enter: 선택된 장르를 폼에 설정하고 FormInput 모드로 돌아가기
                        self.state.select_current_genre();
                        self.state.mode = AppMode::FormInput;
                        let current_value = self.state.get_current_form_field_value();
                        self.text_input = TextInput::with_text(current_value);
                    }
                    AppMode::YearSelect => {
                        // 년도 선택 모드에서 Enter: 선택된 년도를 폼에 설정하고 FormInput 모드로 돌아가기
                        self.state.select_current_year();
                        self.state.mode = AppMode::FormInput;
                        let current_value = self.state.get_current_form_field_value();
                        self.text_input = TextInput::with_text(current_value);
                    }
                    AppMode::Normal => {
                        match self.state.current_screen {
                            Screen::AddBook => {
                                // 도서 추가 화면에서 Enter: 현재 필드 편집 시작
                                if self.state.is_genre_field() {
                                    // 장르 필드에서 Enter를 누르면 장르 선택 모드로 전환
                                    self.state.mode = AppMode::GenreSelect;
                                    // 현재 장르 값에 맞는 인덱스 찾기
                                    let genres = AppState::get_genres();
                                    if let Some(index) =
                                        genres.iter().position(|&g| g == self.state.form_genre)
                                    {
                                        self.state.genre_selected_index = index;
                                    }
                                } else if self.state.is_year_field() {
                                    // 출간년도 필드에서 Enter를 누르면 년도 선택 모드로 전환
                                    self.state.mode = AppMode::YearSelect;
                                    // 현재 년도 값에 맞는 인덱스 찾기
                                    let years = AppState::get_years();
                                    if let Ok(current_year) =
                                        self.state.form_pub_year.parse::<u32>()
                                    {
                                        if let Some(index) =
                                            years.iter().position(|&y| y == current_year)
                                        {
                                            self.state.year_selected_index = index;
                                        }
                                    }
                                } else {
                                    // 다른 필드에서는 Edit 모드로 전환
                                    let current_value = self.state.get_current_form_field_value();
                                    self.state.mode = AppMode::Edit;
                                    self.text_input = TextInput::with_text(current_value);
                                }
                            }
                            Screen::Search => {
                                // 검색 화면에서 Normal 모드일 때 Enter를 누르면 선택된 도서의 리뷰 화면으로 이동
                                if !self.state.search_query.is_empty() {
                                    // 검색 결과에서 선택된 도서의 실제 인덱스 찾기
                                    let search_results: Vec<(
                                        usize,
                                        &crate::lib::models::ExtendedBook,
                                    )> = self
                                        .state
                                        .books
                                        .iter()
                                        .enumerate()
                                        .filter(|(_, book)| {
                                            let query = self.state.search_query.to_lowercase();
                                            book.book.title.to_lowercase().contains(&query)
                                                || book.authors.iter().any(|author| {
                                                    author.name.to_lowercase().contains(&query)
                                                })
                                                || book.book.genre.to_lowercase().contains(&query)
                                                || book.reviews.iter().any(|review| {
                                                    review.review.to_lowercase().contains(&query)
                                                })
                                        })
                                        .collect();

                                    if let Some((actual_index, _)) =
                                        search_results.get(self.state.search_selected_index)
                                    {
                                        self.state.selected_book_index = *actual_index;
                                        self.state.set_screen(Screen::Review);
                                        self.state.selected_review_index = 0;
                                    }
                                }
                            }
                            _ => {
                                // 다른 화면에서는 아직 구현하지 않음
                            }
                        }
                    }
                    _ => {
                        // 다른 모드에서는 무시
                    }
                }
            }
            _ => {
                // 다른 액션들은 무시하거나 나중에 구현
            }
        }

        Ok(())
    }

    /// 리뷰 저장을 처리합니다
    fn handle_save_review(&mut self, text: String) {
        if let Some(book) = self.state.books.get(self.state.selected_book_index) {
            if let Some(book_id) = book.book.id {
                if let Some(review_index) = self.state.editing_review_index {
                    // 기존 리뷰 수정
                    if let Some(existing_review) = book.reviews.get(review_index) {
                        let updated_review = crate::lib::models::Review {
                            id: existing_review.id,
                            book_id: existing_review.book_id,
                            date_read: existing_review.date_read,
                            rating: existing_review.rating,
                            review: text.trim().to_string(),
                        };

                        match self
                            .database
                            .update_review(existing_review.id.unwrap(), &updated_review)
                        {
                            Ok(_) => {
                                if let Err(e) = self.load_books() {
                                    self.state
                                        .set_message(format!("도서 목록 로드 실패: {}", e));
                                } else {
                                    self.state
                                        .set_message("✅ 리뷰가 수정되었습니다!".to_string());
                                }
                            }
                            Err(e) => {
                                self.state.set_message(format!("❌ 리뷰 수정 실패: {}", e));
                            }
                        }
                    } else {
                        self.state
                            .set_message("❌ 수정할 리뷰를 찾을 수 없습니다".to_string());
                    }
                } else {
                    // 새 리뷰 생성
                    let new_review = crate::lib::models::NewReview {
                        book_id,
                        date_read: Some(chrono::Utc::now().date_naive()),
                        rating: 5, // 기본값, 나중에 UI에서 입력받도록 개선
                        review: text.trim().to_string(),
                    };

                    match self.database.add_review(&new_review) {
                        Ok(review_id) => {
                            if let Err(e) = self.load_books() {
                                self.state
                                    .set_message(format!("도서 목록 로드 실패: {}", e));
                            } else {
                                self.state.set_message(format!(
                                    "✅ 새 리뷰가 저장되었습니다! (ID: {})",
                                    review_id
                                ));
                            }
                        }
                        Err(e) => {
                            self.state.set_message(format!("❌ 리뷰 저장 실패: {}", e));
                        }
                    }
                }
            } else {
                self.state
                    .set_message("❌ 선택된 도서의 ID가 없습니다".to_string());
            }
        } else {
            self.state.set_message("❌ 도서를 선택해주세요".to_string());
        }

        // 편집 상태 초기화
        self.state.editing_review_index = None;
    }

    /// 도서 저장하고 나가기를 처리합니다
    fn handle_save_book_and_exit(&mut self) {
        // 폼 유효성 검사
        if let Err(error_msg) = self.state.validate_form() {
            self.state.set_message(error_msg);
            return;
        }

        // 저자 파싱 (쉼표로 구분)
        let authors: Vec<String> = self
            .state
            .form_authors
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // 번역자 파싱 (쉼표로 구분, 선택사항)
        let translators: Vec<String> = if self.state.form_translators.trim().is_empty() {
            Vec::new()
        } else {
            self.state
                .form_translators
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        // 새 도서 생성
        let new_book = crate::lib::models::NewBook {
            title: self.state.form_title.trim().to_string(),
            authors,
            translators,
            genre: self.state.form_genre.trim().to_string(),
            pages: if self.state.form_pages.trim().is_empty() {
                None
            } else {
                self.state.form_pages.trim().parse().ok()
            },
            pub_year: if self.state.form_pub_year.trim().is_empty() {
                None
            } else {
                self.state.form_pub_year.trim().parse().ok()
            },
        };

        // 데이터베이스에 저장
        match self.database.add_book(&new_book) {
            Ok(book_id) => {
                if let Err(e) = self.load_books() {
                    self.state
                        .set_message(format!("도서 목록 로드 실패: {}", e));
                } else {
                    // 새로 추가된 도서를 찾아서 선택
                    if let Some(new_book_index) = self
                        .state
                        .books
                        .iter()
                        .position(|book| book.book.id == Some(book_id))
                    {
                        self.state.selected_book_index = new_book_index;
                    }

                    self.state
                        .set_message(format!("✅ 새 도서가 저장되었습니다! (ID: {})", book_id));
                    // 성공 시 도서 목록으로 돌아가기
                    self.state.current_screen = Screen::BookList;
                    self.state.clear_form();
                    self.state.mode = AppMode::Normal; // Normal 모드로 복원
                }
            }
            Err(e) => {
                self.state.set_message(format!("❌ 도서 저장 실패: {}", e));
            }
        }
    }

    /// 도서 저장을 처리합니다 (기존 메서드 유지)
    fn handle_save_book(&mut self) {
        // 폼 유효성 검사
        if let Err(error_msg) = self.state.validate_form() {
            self.state.set_message(error_msg);
            return;
        }

        // 저자 파싱 (쉼표로 구분)
        let authors: Vec<String> = self
            .state
            .form_authors
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // 번역자 파싱 (쉼표로 구분, 선택사항)
        let translators: Vec<String> = if self.state.form_translators.trim().is_empty() {
            Vec::new()
        } else {
            self.state
                .form_translators
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        // 새 도서 생성
        let new_book = crate::lib::models::NewBook {
            title: self.state.form_title.trim().to_string(),
            authors,
            translators,
            genre: self.state.form_genre.trim().to_string(),
            pages: if self.state.form_pages.trim().is_empty() {
                None
            } else {
                self.state.form_pages.trim().parse().ok()
            },
            pub_year: if self.state.form_pub_year.trim().is_empty() {
                None
            } else {
                self.state.form_pub_year.trim().parse().ok()
            },
        };

        // 데이터베이스에 저장
        match self.database.add_book(&new_book) {
            Ok(book_id) => {
                if let Err(e) = self.load_books() {
                    self.state
                        .set_message(format!("도서 목록 로드 실패: {}", e));
                } else {
                    // 새로 추가된 도서를 찾아서 선택
                    if let Some(new_book_index) = self
                        .state
                        .books
                        .iter()
                        .position(|book| book.book.id == Some(book_id))
                    {
                        self.state.selected_book_index = new_book_index;
                    }

                    self.state
                        .set_message(format!("✅ 새 도서가 저장되었습니다! (ID: {})", book_id));
                    // 성공 시 도서 목록으로 돌아가기
                    self.state.current_screen = Screen::BookList;
                    self.state.clear_form();
                }
            }
            Err(e) => {
                self.state.set_message(format!("❌ 도서 저장 실패: {}", e));
            }
        }
    }

    /// 선택된 도서를 삭제합니다
    fn handle_delete_book(&mut self) {
        if self.state.books.is_empty() {
            self.state
                .set_message("❌ 삭제할 도서가 없습니다".to_string());
            return;
        }

        if let Some(book) = self.state.books.get(self.state.selected_book_index) {
            if let Some(book_id) = book.book.id {
                let book_title = book.book.title.clone();

                match self.database.delete_book(book_id as i64) {
                    Ok(_) => {
                        // 삭제 성공 시 도서 목록 다시 로드
                        if let Err(e) = self.load_books() {
                            self.state
                                .set_message(format!("도서 목록 로드 실패: {}", e));
                        } else {
                            self.state.set_message(format!(
                                "✅ \"{}\" 도서가 삭제되었습니다!",
                                book_title
                            ));

                            // 선택된 도서 인덱스 조정
                            if self.state.selected_book_index >= self.state.books.len()
                                && !self.state.books.is_empty()
                            {
                                self.state.selected_book_index = self.state.books.len() - 1;
                            } else if self.state.books.is_empty() {
                                self.state.selected_book_index = 0;
                            }
                        }
                    }
                    Err(e) => {
                        self.state.set_message(format!("❌ 도서 삭제 실패: {}", e));
                    }
                }
            } else {
                self.state.set_message("❌ 도서 ID가 없습니다".to_string());
            }
        } else {
            self.state
                .set_message("❌ 선택된 도서가 없습니다".to_string());
        }
    }

    /// 선택된 리뷰를 삭제합니다
    fn handle_delete_review(&mut self) {
        if let Some(book) = self.state.books.get(self.state.selected_book_index) {
            if self.state.selected_review_index < book.reviews.len() {
                let review_to_delete = &book.reviews[self.state.selected_review_index];

                if let Some(review_id) = review_to_delete.id {
                    match self.database.delete_review(review_id) {
                        Ok(_) => {
                            // 삭제 성공 시 도서 목록 다시 로드
                            if let Err(e) = self.load_books() {
                                self.state
                                    .set_message(format!("도서 목록 로드 실패: {}", e));
                            } else {
                                self.state
                                    .set_message("✅ 리뷰가 삭제되었습니다!".to_string());

                                // 선택된 리뷰 인덱스 조정
                                if let Some(updated_book) =
                                    self.state.books.get(self.state.selected_book_index)
                                {
                                    if self.state.selected_review_index
                                        >= updated_book.reviews.len()
                                        && !updated_book.reviews.is_empty()
                                    {
                                        self.state.selected_review_index =
                                            updated_book.reviews.len() - 1;
                                    } else if updated_book.reviews.is_empty() {
                                        self.state.selected_review_index = 0;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            self.state.set_message(format!("❌ 리뷰 삭제 실패: {}", e));
                        }
                    }
                } else {
                    self.state.set_message("❌ 리뷰 ID가 없습니다".to_string());
                }
            } else {
                self.state
                    .set_message("❌ 선택된 리뷰가 없습니다".to_string());
            }
        } else {
            self.state.set_message("❌ 도서를 선택해주세요".to_string());
        }
    }
}
