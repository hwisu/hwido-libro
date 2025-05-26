# 📋 Libro CLI → TUI 변환 계획

## 🎯 프로젝트 개요

- **목표**: 기존 CLI 도서 관리 도구를 TUI(Terminal User Interface)로 변환
- **키보드**: 해피해킹 키보드 최적화 (Fn 키 없이 사용 가능)
- **호환성**: 기존 CLI 모드 유지 (`--cli` 플래그)

## 🎮 키매핑 설계 (모드별 분리)

### 애플리케이션 모드 정의

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,      // 일반 네비게이션 모드
    Edit,        // 텍스트 편집 모드 (리뷰, 폼 입력)
    Search,      // 검색 입력 모드
    Confirm,     // 확인 다이얼로그 모드
}
```

### Normal 모드 키매핑 (전역 키매핑)

- `q` - Quit (종료)
- `Esc` - Back/Cancel (뒤로가기/취소)
- `?` - Help (도움말)
- `j` - Move down (아래로)
- `k` - Move up (위로)
- `h` - Move left (왼쪽으로, 폼에서만)
- `l` - Move right (오른쪽으로, 폼에서만)
- `Enter` - Select/Confirm (선택/확인)
- `a` - Add book (도서 추가)
- `e` - Edit book (도서 편집)
- `d` - Delete book/review (도서/리뷰 삭제 - 컨텍스트 의존)
- `v` - View/Review (리뷰 보기/편집)
- `n` - New review (새 리뷰 작성)
- `/` - Search (검색)
- `r` - Report (리포트)
- `Tab` - Next field (다음 필드)
- `Shift+Tab` - Previous field (이전 필드)
- `Space` - Toggle selection (선택 토글)
- `1` - Author statistics (작가 통계)
- `2` - Year statistics (연도별 통계)
- `3` - Recent books (최근 도서)

### Edit 모드 키매핑 (편집 전용 - 모든 전역 키 무시)

**편집 모드에서는 아래 키들만 작동하고, 나머지 모든 키는 텍스트 입력으로 처리**

#### 편집 제어 키

- `Ctrl+S` - Save and exit edit mode (저장 후 편집 모드 종료)
- `Ctrl+X` - Cancel edit mode (편집 취소 후 모드 종료)
- `Ctrl+Q` - Force quit edit mode (강제 종료)

#### 텍스트 편집 키

- `Enter` - New line (새 줄 - 일반적인 Enter 동작)
- `Backspace` - Delete previous character (이전 문자 삭제)
- `Delete` - Delete next character (다음 문자 삭제)
- `Ctrl+A` - Move to beginning of line (줄 시작으로)
- `Ctrl+E` - Move to end of line (줄 끝으로)
- `Ctrl+U` - Clear current line (현재 줄 삭제)
- `Ctrl+K` - Delete from cursor to end of line (커서부터 줄 끝까지 삭제)
- `Ctrl+W` - Delete previous word (이전 단어 삭제)

#### 커서 이동 (편집 모드 전용)

- `Ctrl+H` - Move left (왼쪽으로)
- `Ctrl+L` - Move right (오른쪽으로)
- `Ctrl+J` - Move down (아래로)
- `Up/Down` - Move up/down (위/아래로)
- `Left/Right` - Move left/right (좌/우로)

### Search 모드 키매핑

- `Enter` - Execute search (검색 실행)
- `Esc` - Exit search mode (검색 모드 종료)
- `Ctrl+U` - Clear search input (검색어 삭제)
- 나머지는 텍스트 입력으로 처리

### Confirm 모드 키매핑 (삭제 확인 등)

- `y` / `Y` - Yes (확인)
- `n` / `N` - No (취소)
- `Enter` - Confirm selected option (선택된 옵션 확인)
- `Esc` - Cancel (취소)

## 🏗️ 아키텍처 설계

### 디렉토리 구조

```
libro-cli/src/
├── main.rs                 # 진입점 (CLI/TUI 모드 선택)
├── lib.rs                  # 공통 라이브러리
├── cli/                    # 기존 CLI 모드 (유지)
│   ├── mod.rs
│   └── commands/           # 기존 commands 디렉토리 이동
│       ├── mod.rs
│       ├── add.rs
│       ├── browse.rs
│       ├── report.rs
│       └── review.rs
├── tui/                    # 새로운 TUI 모드
│   ├── mod.rs
│   ├── app.rs             # 메인 애플리케이션 상태
│   ├── state.rs           # 애플리케이션 상태 관리
│   ├── events.rs          # 이벤트 처리 및 키매핑
│   ├── input.rs           # 텍스트 입력 위젯
│   └── ui/                # UI 컴포넌트들
│       ├── mod.rs
│       ├── book_list.rs   # 도서 목록 화면
│       ├── review.rs      # 리뷰 화면 (완전 구현)
│       └── help.rs        # 도움말 화면
├── lib/                   # 기존 비즈니스 로직 (공통 사용)
└── utils/                 # 유틸리티 (공통 사용)
```

### 핵심 상태 관리 구조

```rust
pub struct AppState {
    pub mode: AppMode,
    pub current_screen: Screen,
    pub selected_book_index: usize,
    pub selected_review_index: usize,    // 리뷰 선택 인덱스
    pub edit_buffer: String,             // 편집 중인 텍스트
    pub cursor_position: usize,          // 편집 모드 커서 위치
    pub search_query: String,
    pub should_quit: bool,
    pub previous_screen: Option<Screen>, // 뒤로가기를 위한 이전 화면
    pub books: Vec<ExtendedBook>,        // 도서 목록 (리뷰 포함)
    pub error_message: Option<String>,   // 에러/성공 메시지
    pub message_timer: Option<Instant>,  // 메시지 표시 시간
    pub editing_review_index: Option<usize>, // 편집 중인 리뷰 인덱스
}

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
```

## 📦 의존성 추가

### Cargo.toml 수정사항

```toml
[dependencies]
# 기존 의존성 유지
clap      = { version = "4.2", features = ["derive"] }
rusqlite  = "0.29"
chrono    = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
serde     = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dialoguer = "0.11"
console   = "0.15"
tabled    = "0.15"

# 새로운 TUI 의존성
ratatui = "0.24"
crossterm = "0.27"
tokio = { version = "1.0", features = ["full"] }
unicode-width = "0.1"  # 텍스트 편집용
```

## 🚀 구현 단계별 체크리스트

### Phase 1: 프로젝트 구조 재구성 ✅

- [x] `cli/` 디렉토리 생성
- [x] 기존 `commands/` 디렉토리를 `cli/commands/`로 이동
- [x] `cli/mod.rs` 생성 및 기존 CLI 로직 모듈화
- [x] `tui/` 디렉토리 생성
- [x] `main.rs` 수정하여 CLI/TUI 모드 선택 로직 추가
- [x] Cargo.toml에 TUI 의존성 추가

### Phase 2: 기본 TUI 프레임워크 및 모드 시스템 설정 ✅

- [x] `tui/mod.rs` 생성
- [x] `tui/app.rs` - 메인 애플리케이션 상태 구조체 정의
- [x] `tui/state.rs` - 애플리케이션 상태 관리 (AppMode, Screen 포함)
- [x] `tui/events.rs` - 키보드 이벤트 처리 및 모드별 키매핑
- [x] 기본 TUI 루프 구현 (터미널 초기화, 이벤트 루프, 정리)
- [x] 모드별 키 핸들러 통합 구현

### Phase 3: 텍스트 입력 시스템 구현 ✅

- [x] `tui/input.rs` - 텍스트 입력 위젯 구현
  - [x] 커서 위치 관리
  - [x] 텍스트 삽입/삭제
  - [x] 멀티라인 지원
  - [x] 유니코드 문자 지원 (한글 등)
- [x] Edit 모드 전환 시스템
  - [x] Normal → Edit 모드 전환
  - [x] Edit → Normal 모드 전환
  - [x] 편집 버퍼 관리

### Phase 4: 핵심 UI 컴포넌트 구현 ✅

- [x] `tui/ui/mod.rs` - UI 모듈 정의
- [x] 기본 레이아웃 (헤더, 메인, 푸터)
  - [x] 현재 모드 표시
  - [x] 모드별 키 힌트 표시
  - [x] 에러/성공 메시지 표시 (3초 자동 삭제)
- [x] `tui/ui/book_list.rs` - 도서 목록 화면
  - [x] 도서 목록 표시
  - [x] 선택된 항목 하이라이트
  - [x] j/k 네비게이션 (Normal 모드에서만)
  - [x] 상태 표시 (총 도서 수, 현재 선택)
  - [x] 데이터베이스 연동 (자동 로드)
- [x] `tui/ui/help.rs` - 도움말 화면
  - [x] 모드별 키매핑 설명
  - [x] 편집 모드 사용법 안내

### Phase 5: 도서 상세 및 네비게이션 🔄

- [ ] `tui/ui/book_detail.rs` - 도서 상세 화면
  - [ ] 도서 정보 표시 (제목, 저자, 연도, 페이지, 장르)
  - [ ] 리뷰 정보 표시 (평점, 리뷰 텍스트, 읽은 날짜)
  - [ ] 스크롤 기능 (긴 리뷰의 경우)
- [x] 화면 간 네비게이션 구현
  - [x] 메인 → 리뷰 → 메인
  - [x] Esc 키로 뒤로가기 (Normal 모드에서만)
  - [x] Enter 키로 선택 (Normal 모드에서만)

### Phase 6: 검색 기능 (Search 모드) ✅

- [x] `tui/ui/search.rs` - 검색 화면
  - [x] 검색어 입력 필드 (Search 모드 진입)
  - [x] 실시간 검색 결과 표시
  - [x] 검색 결과에서 도서 선택 (Enter로 리뷰 화면 이동)
  - [x] Search 모드에서 Normal 모드로 전환
  - [x] 포괄적 검색 (제목, 저자, 장르, 리뷰 내용)
  - [x] j/k 키로 검색 결과 네비게이션
  - [x] Esc로 도서 목록으로 돌아가기

### Phase 7: 도서 추가 기능 (Edit 모드 활용) ⏳

- [ ] `tui/ui/add_book.rs` - 도서 추가 화면
  - [ ] 입력 폼 구현 (제목, 저자, 연도, 페이지, 장르)
  - [ ] Tab/Shift+Tab 필드 네비게이션 (Normal 모드)
  - [ ] 필드 편집 시 Edit 모드 진입
  - [ ] 입력 유효성 검사
  - [ ] 장르 드롭다운 선택
  - [ ] 저장/취소 기능

### Phase 8: 도서 편집 기능 ⏳

- [ ] `tui/ui/edit_book.rs` - 도서 편집 화면
  - [ ] 기존 정보로 폼 미리 채우기
  - [ ] add_book.rs와 유사한 폼 구조
  - [ ] Edit 모드 활용
  - [ ] 변경사항 저장/취소

### Phase 9: 리뷰 관리 (완전한 Edit 모드 구현) ✅

- [x] `tui/ui/review.rs` - 리뷰 화면
  - [x] 리뷰 목록 표시 (기존 리뷰들)
  - [x] 리뷰 선택 네비게이션 (j/k 키)
  - [x] 선택된 리뷰 하이라이트
  - [x] 리뷰 텍스트 편집기 구현 (Edit 모드)
  - [x] 멀티라인 텍스트 입력
  - [x] 기존 리뷰 편집 vs 새 리뷰 작성 구분
  - [x] 리뷰 삭제 기능 (d 키)
  - [x] Edit 모드 키매핑 완전 구현:
    - [x] `Ctrl+S` - 리뷰 저장 후 Normal 모드
    - [x] `Ctrl+X` - 편집 취소 후 Normal 모드
    - [x] `Ctrl+Q` - 강제 종료
    - [x] `Enter` - 새 줄 추가 (일반적인 동작)
    - [x] `Ctrl+A`, `Ctrl+E` - 줄 시작/끝 이동
    - [x] `Ctrl+U`, `Ctrl+K` - 줄 삭제 기능
    - [x] `Ctrl+W` - 단어 삭제
    - [x] 화살표 키 지원
  - [x] **전역 키 완전 무시** (q, esc, j, k, a, e, d, v, /, r 등)
  - [x] 유니코드 문자열 안전 처리
  - [x] 데이터베이스 연동 (저장/수정/삭제)

### Phase 10: 리포트 기능 ⏳

- [ ] `tui/ui/report.rs` - 리포트 화면
  - [ ] 작가별 통계 (1키)
  - [ ] 연도별 통계 (2키)
  - [ ] 최근 도서 목록 (3키)
  - [ ] 차트/그래프 표시 (ASCII 아트 또는 간단한 바 차트)
  - [ ] 뷰 전환 애니메이션

### Phase 11: 도서 삭제 기능 (Confirm 모드) ⏳

- [ ] Confirm 모드 구현
- [ ] 삭제 확인 다이얼로그
- [ ] y/n 키로 확인/취소
- [ ] 안전한 삭제 (관련 리뷰도 함께 삭제)
- [ ] 삭제 후 목록 새로고침

### Phase 12: 고급 기능 및 최적화 ⏳

- [ ] 반응형 레이아웃 (터미널 크기 변경 대응)
- [ ] 테마 지원 (다크/라이트 모드)
- [ ] 설정 파일 지원
- [ ] 모드 전환 애니메이션
- [ ] 에러 처리 및 사용자 피드백

## 🔒 모드 안전성 보장 ✅

### Edit 모드 격리 원칙

1. **완전한 키 격리**: Edit 모드에서는 Ctrl 조합키와 특수 키(Enter, Backspace
   등)만 처리 ✅
2. **전역 키 무시**: q, esc, j, k, a, e, d, v, /, r, Tab, Space 등 모든 전역
   키는 텍스트로 처리 ✅
3. **명시적 모드 전환**: Edit 모드 진입/탈출은 명시적 키 조합으로만 가능 ✅
4. **상태 보존**: Edit 모드 취소 시 이전 상태로 완전 복원 ✅

### 키 충돌 방지 체크리스트

- [x] Edit 모드에서 전역 키매핑 완전 비활성화 확인
- [x] 모드별 키 핸들러 분리 확인
- [x] 텍스트 입력 중 의도하지 않은 동작 방지 확인
- [x] 모드 전환 시 상태 일관성 확인

## 🎯 현재 상태 및 우선순위

### ✅ 완료된 기능들

1. **기본 TUI 프레임워크**: 완전 구현
2. **모드 시스템**: Normal/Edit/Search 모드 완전 분리
3. **도서 목록**: j/k 네비게이션, 데이터베이스 연동
4. **리뷰 관리**: 완전한 CRUD 기능
   - 리뷰 목록 표시 및 선택
   - 기존 리뷰 편집 vs 새 리뷰 작성
   - 리뷰 삭제
   - 완전한 Edit 모드 구현 (전역 키 격리)
   - 유니코드 안전 처리
5. **검색 기능**: 완전한 Search 모드 구현
   - 포괄적 검색 (제목, 저자, 장르, 리뷰 내용)
   - 검색 결과 네비게이션 및 선택
   - 검색 결과에서 리뷰 화면으로 이동
6. **텍스트 입력**: 멀티라인, 유니코드 지원
7. **메시지 시스템**: 성공/실패 메시지 자동 표시

### 🔄 다음 우선순위

1. **Phase 7-8**: 도서 추가/편집 (Edit 모드 재활용)
2. **Phase 10**: 리포트 기능
3. **Phase 11**: 도서 삭제 (Confirm 모드)
4. **Phase 12**: 고급 기능 및 최적화

### 🎮 현재 작동하는 키매핑

#### 도서 목록 화면 (Normal 모드)

- `j/k`: 도서 선택
- `v`: 리뷰 화면으로 이동
- `/`: 검색 화면으로 이동
- `?`: 도움말
- `q`: 종료

#### 리뷰 화면 (Normal 모드)

- `j/k`: 리뷰 선택
- `v`: 선택된 리뷰 편집
- `n`: 새 리뷰 작성
- `d`: 선택된 리뷰 삭제
- `Esc`: 도서 목록으로 돌아가기

#### 검색 화면 (Normal/Search 모드)

- `/`: 검색 모드 진입
- `j/k`: 검색 결과 선택 (Normal 모드)
- `Enter`: 선택된 도서의 리뷰 화면으로 이동 (Normal 모드)
- `Esc`: 도서 목록으로 돌아가기

#### 검색 입력 (Search 모드)

- `Enter`: 검색 실행
- `Esc`: 검색 취소
- `Ctrl+U`: 검색어 삭제
- 모든 텍스트 입력 키 지원

#### 리뷰 편집 (Edit 모드)

- `Ctrl+S`: 저장
- `Ctrl+X`: 취소
- `Ctrl+Q`: 강제 종료
- 모든 텍스트 편집 키 지원
- **모든 전역 키 무시** (완전한 격리)

## 📈 프로젝트 진행률

- **전체 진행률**: 약 70% 완료
- **핵심 기능**: 90% 완료 (검색 기능까지 완전 구현)
- **기본 TUI**: 100% 완료
- **모드 시스템**: 100% 완료 (Normal/Edit/Search 모드 완전 분리)
- **데이터베이스 연동**: 90% 완료 (리뷰 CRUD, 검색 완료)

해피해킹 키보드에 최적화된 핵심 기능들(도서 목록, 리뷰 관리, 검색)이 완전히
구현되어 실용적으로 사용 가능한 상태입니다! 🎉
