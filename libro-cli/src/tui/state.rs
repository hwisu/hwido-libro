//! 애플리케이션 상태 관리

use crate::lib::models::ExtendedBook;
use chrono::Datelike;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,      // 일반 네비게이션 모드
    Edit,        // 텍스트 편집 모드 (리뷰, 폼 입력)
    Search,      // 검색 입력 모드
    Confirm,     // 확인 다이얼로그 모드
    FormInput,   // 폼 직접 입력 모드 (도서 추가/편집)
    GenreSelect, // 장르 선택 모드
    YearSelect,  // 출간년도 선택 모드
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    BookList,
    BookDetail,
    AddBook,
    EditBook,
    Review,
    Search,
    Report,
    Help,
    ConfirmDelete,
}

pub struct AppState {
    pub mode: AppMode,
    pub current_screen: Screen,
    pub selected_book_index: usize,
    pub selected_review_index: usize, // 리뷰 화면에서 선택된 리뷰 인덱스
    pub edit_buffer: String,          // 편집 중인 텍스트
    pub cursor_position: usize,       // 편집 모드 커서 위치
    pub search_query: String,
    pub search_selected_index: usize, // 검색 결과에서 선택된 인덱스
    pub should_quit: bool,
    pub previous_screen: Option<Screen>, // 뒤로가기를 위한 이전 화면
    pub books: Vec<crate::lib::models::ExtendedBook>, // 도서 목록
    pub error_message: Option<String>,   // 에러/성공 메시지
    pub message_timer: Option<std::time::Instant>, // 메시지 표시 시간
    pub editing_review_index: Option<usize>, // 편집 중인 리뷰의 인덱스 (None이면 새 리뷰)

    // 도서 추가/편집 폼 관련 필드들
    pub form_field_index: usize,      // 현재 선택된 폼 필드 인덱스
    pub form_title: String,           // 제목
    pub form_authors: String,         // 저자 (쉼표로 구분)
    pub form_translators: String,     // 번역자 (쉼표로 구분, 선택사항)
    pub form_genre: String,           // 장르
    pub form_pages: String,           // 페이지 수 (선택사항)
    pub form_pub_year: String,        // 출간년도 (선택사항)
    pub editing_book_id: Option<u32>, // 편집 중인 도서 ID (None이면 새 도서)

    // 장르 선택 관련
    pub genre_selected_index: usize, // 선택된 장르 인덱스

    // 출간년도 선택 관련
    pub year_selected_index: usize, // 선택된 년도 인덱스
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: AppMode::Normal,
            current_screen: Screen::BookList,
            selected_book_index: 0,
            selected_review_index: 0,
            edit_buffer: String::new(),
            cursor_position: 0,
            search_query: String::new(),
            search_selected_index: 0,
            should_quit: false,
            previous_screen: None,
            books: Vec::new(),
            error_message: None,
            message_timer: None,
            editing_review_index: None,

            // 도서 추가/편집 폼 관련 필드들 초기화
            form_field_index: 0,
            form_title: String::new(),
            form_authors: String::new(),
            form_translators: String::new(),
            form_genre: String::new(),
            form_pages: String::new(),
            form_pub_year: String::new(),
            editing_book_id: None,

            // 장르 선택 관련
            genre_selected_index: 0,

            // 출간년도 선택 관련
            year_selected_index: 0,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// 사용 가능한 장르 목록을 반환합니다
    pub fn get_genres() -> Vec<&'static str> {
        vec!["소설", "에세이", "자기계발", "기술/IT", "기타"]
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.previous_screen = Some(self.current_screen.clone());
        self.current_screen = screen;
    }

    pub fn go_back(&mut self) {
        if let Some(prev_screen) = self.previous_screen.take() {
            self.current_screen = prev_screen;
        }
    }

    pub fn enter_edit_mode(&mut self, initial_text: String) {
        self.mode = AppMode::Edit;
        self.edit_buffer = initial_text;
        self.cursor_position = self.edit_buffer.len();
    }

    pub fn exit_edit_mode(&mut self) -> String {
        self.mode = AppMode::Normal;
        let text = self.edit_buffer.clone();
        self.edit_buffer.clear();
        self.cursor_position = 0;
        text
    }

    pub fn cancel_edit_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.edit_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn set_message(&mut self, message: String) {
        self.error_message = Some(message);
        self.message_timer = Some(std::time::Instant::now());
    }

    pub fn clear_expired_message(&mut self) {
        if let Some(timer) = self.message_timer {
            if timer.elapsed().as_secs() >= 3 {
                self.error_message = None;
                self.message_timer = None;
            }
        }
    }

    /// 폼 필드를 초기화합니다
    pub fn clear_form(&mut self) {
        self.form_field_index = 0;
        self.form_title.clear();
        self.form_authors.clear();
        self.form_translators.clear();
        self.form_genre.clear();
        self.form_pages.clear();
        self.form_pub_year.clear();
        self.editing_book_id = None;
        self.genre_selected_index = 0;
        self.year_selected_index = 0;
    }

    /// 다음 폼 필드로 이동합니다
    pub fn next_form_field(&mut self) {
        self.form_field_index = (self.form_field_index + 1) % 6; // 총 6개 필드
    }

    /// 이전 폼 필드로 이동합니다
    pub fn prev_form_field(&mut self) {
        if self.form_field_index == 0 {
            self.form_field_index = 5; // 마지막 필드로
        } else {
            self.form_field_index -= 1;
        }
    }

    /// 현재 선택된 폼 필드의 값을 반환합니다
    pub fn get_current_form_field_value(&self) -> String {
        match self.form_field_index {
            0 => self.form_title.clone(),
            1 => self.form_authors.clone(),
            2 => self.form_translators.clone(),
            3 => self.form_genre.clone(),
            4 => self.form_pages.clone(),
            5 => self.form_pub_year.clone(),
            _ => String::new(),
        }
    }

    /// 현재 선택된 폼 필드의 값을 설정합니다
    pub fn set_current_form_field_value(&mut self, value: String) {
        match self.form_field_index {
            0 => self.form_title = value,
            1 => self.form_authors = value,
            2 => self.form_translators = value,
            3 => self.form_genre = value,
            4 => self.form_pages = value,
            5 => self.form_pub_year = value,
            _ => {}
        }
    }

    /// 현재 선택된 폼 필드의 이름을 반환합니다
    pub fn get_current_form_field_name(&self) -> &'static str {
        match self.form_field_index {
            0 => "제목",
            1 => "저자",
            2 => "번역자",
            3 => "장르",
            4 => "페이지",
            5 => "출간년도",
            _ => "알 수 없음",
        }
    }

    /// 폼 유효성을 검사합니다
    pub fn validate_form(&self) -> Result<(), String> {
        if self.form_title.trim().is_empty() {
            return Err("제목을 입력해주세요".to_string());
        }
        if self.form_authors.trim().is_empty() {
            return Err("저자를 입력해주세요".to_string());
        }
        if self.form_genre.trim().is_empty() {
            return Err("장르를 입력해주세요".to_string());
        }

        // 페이지 수 유효성 검사 (선택사항이지만 입력된 경우)
        if !self.form_pages.trim().is_empty() {
            if self.form_pages.trim().parse::<u32>().is_err() {
                return Err("페이지 수는 숫자여야 합니다".to_string());
            }
        }

        // 출간년도 유효성 검사 (선택사항이지만 입력된 경우)
        if !self.form_pub_year.trim().is_empty() {
            if self.form_pub_year.trim().parse::<u32>().is_err() {
                return Err("출간년도는 숫자여야 합니다".to_string());
            }
        }

        Ok(())
    }

    /// 장르 선택에서 위로 이동합니다
    pub fn move_genre_up(&mut self) {
        if self.genre_selected_index > 0 {
            self.genre_selected_index -= 1;
        }
    }

    /// 장르 선택에서 아래로 이동합니다
    pub fn move_genre_down(&mut self) {
        let genres = Self::get_genres();
        if self.genre_selected_index + 1 < genres.len() {
            self.genre_selected_index += 1;
        }
    }

    /// 현재 선택된 장르를 반환합니다
    pub fn get_selected_genre(&self) -> &'static str {
        let genres = Self::get_genres();
        genres.get(self.genre_selected_index).unwrap_or(&"Fiction")
    }

    /// 장르를 선택하고 폼에 설정합니다
    pub fn select_current_genre(&mut self) {
        let selected_genre = self.get_selected_genre();
        self.form_genre = selected_genre.to_string();
    }

    /// 현재 필드가 장르 필드인지 확인합니다
    pub fn is_genre_field(&self) -> bool {
        self.form_field_index == 3
    }

    /// 현재 필드가 출간년도 필드인지 확인합니다
    pub fn is_year_field(&self) -> bool {
        self.form_field_index == 5
    }

    /// 사용 가능한 출간년도 목록을 반환합니다 (현재 년도부터 1900년까지)
    pub fn get_years() -> Vec<u32> {
        let current_year = chrono::Utc::now().year() as u32;
        (1900..=current_year).rev().collect()
    }

    /// 출간년도 선택에서 위로 이동합니다
    pub fn move_year_up(&mut self) {
        if self.year_selected_index > 0 {
            self.year_selected_index -= 1;
        }
    }

    /// 출간년도 선택에서 아래로 이동합니다
    pub fn move_year_down(&mut self) {
        let years = Self::get_years();
        if self.year_selected_index + 1 < years.len() {
            self.year_selected_index += 1;
        }
    }

    /// 현재 선택된 출간년도를 반환합니다
    pub fn get_selected_year(&self) -> u32 {
        let years = Self::get_years();
        years.get(self.year_selected_index).copied().unwrap_or(2024)
    }

    /// 출간년도를 선택하고 폼에 설정합니다
    pub fn select_current_year(&mut self) {
        let selected_year = self.get_selected_year();
        self.form_pub_year = selected_year.to_string();
    }
}
