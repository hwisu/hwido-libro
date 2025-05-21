/**
 * 마크다운 파일에서 책 리뷰 데이터를 파싱하는 유틸리티
 *
 * 수정된 마크다운 형식:
 * ```
 * # 도서 정보 (수정 불가)
 *
 * 제목: 책 제목
 * 작가:
 * - 작가1
 * - 작가2
 * 번역:
 * - 번역가1
 * - 번역가2
 * 장르: Fiction|Nonfiction
 * 출판년도: 2023
 * 페이지: 320
 *
 * # 리뷰 정보
 *
 * 읽은 날짜: 2023-04-15
 * 평점: 4.5
 *
 * # 리뷰 내용
 *
 * 책에 대한 리뷰 내용...
 * ```
 */

/**
 * 책 리뷰 정보를 위한 인터페이스
 */
export interface Writer {
  name: string;
  type: "author" | "translator";
}

export interface BookReview {
  title?: string;
  writers?: Writer[];
  genre?: string;
  pub_year?: number;
  pages?: number;
  date_read?: string;
  rating?: number;
  review?: string;
}

/**
 * 마크다운 텍스트에서 도서 리뷰 데이터를 추출합니다
 *
 * @param markdown 파싱할 마크다운 텍스트
 * @returns 파싱된 책 리뷰 데이터
 */
export function parseMarkdownReview(markdown: string): BookReview {
  const result: BookReview = {
    writers: [],
  };

  // 섹션별로 분리
  const sections = markdown.split(/(?=# )/);

  // 도서 정보 섹션 파싱
  const bookInfoSection =
    sections.find((s) => s.startsWith("# 도서 정보"))?.trim() || "";
  if (bookInfoSection) {
    // 제목 파싱
    const titleMatch = bookInfoSection.match(/제목:\s*(.+)$/m);
    if (titleMatch) result.title = titleMatch[1].trim();

    // 작가 파싱
    const authorSection = bookInfoSection.match(/작가:\n((?:-[^\n]+\n?)+)/m);
    if (authorSection) {
      const authors = authorSection[1].match(/-\s*([^\n]+)/g) || [];
      authors.forEach((author) => {
        result.writers?.push({
          name: author.replace(/^-\s*/, "").trim(),
          type: "author",
        });
      });
    }

    // 번역가 파싱
    const translatorSection = bookInfoSection.match(
      /번역:\n((?:-[^\n]+\n?)+)/m,
    );
    if (translatorSection) {
      const translators = translatorSection[1].match(/-\s*([^\n]+)/g) || [];
      translators.forEach((translator) => {
        result.writers?.push({
          name: translator.replace(/^-\s*/, "").trim(),
          type: "translator",
        });
      });
    }

    // 기타 메타데이터 파싱
    const genreMatch = bookInfoSection.match(/장르:\s*(.+)$/m);
    const pubYearMatch = bookInfoSection.match(/출판년도:\s*(\d+)$/m);
    const pagesMatch = bookInfoSection.match(/페이지:\s*(\d+)$/m);

    if (genreMatch) result.genre = genreMatch[1].trim();
    if (pubYearMatch) result.pub_year = parseInt(pubYearMatch[1]);
    if (pagesMatch) result.pages = parseInt(pagesMatch[1]);
  }

  // 리뷰 정보 섹션 찾기
  const reviewInfoSection =
    sections.find((s) => s.startsWith("# 리뷰 정보"))?.trim() || "";
  if (reviewInfoSection) {
    const dateMatch = reviewInfoSection.match(/읽은 날짜:\s*(.+)$/m);
    const ratingMatch = reviewInfoSection.match(/평점:\s*(\d+(\.\d+)?)$/m);

    if (dateMatch) result.date_read = dateMatch[1].trim();
    if (ratingMatch) result.rating = parseFloat(ratingMatch[1]);
  }

  // 리뷰 내용 섹션 찾기
  const reviewContentSection =
    sections.find((s) => s.startsWith("# 리뷰 내용"))?.trim() || "";
  if (reviewContentSection) {
    const content = reviewContentSection.replace(/# 리뷰 내용\s*/, "").trim();
    if (content) result.review = content;
  }

  return result;
}

/**
 * 책 리뷰 데이터를 마크다운 형식으로 변환합니다
 *
 * @param review 마크다운으로 변환할 책 리뷰 데이터
 * @returns 생성된 마크다운 문자열
 */
export function bookReviewToMarkdown(review: BookReview): string {
  const sections = [];

  // 도서 정보 섹션 (읽기 전용)
  sections.push("# 도서 정보 (수정 불가)\n");
  const bookInfo = [];
  if (review.title) bookInfo.push(`제목: ${review.title}`);

  // 작가 정보
  if (review.writers && review.writers.length > 0) {
    const authors = review.writers.filter((w) => w.type === "author");
    if (authors.length > 0) {
      bookInfo.push("작가:");
      authors.forEach((author) => bookInfo.push(`- ${author.name}`));
    }

    const translators = review.writers.filter((w) => w.type === "translator");
    if (translators.length > 0) {
      bookInfo.push("번역:");
      translators.forEach((translator) =>
        bookInfo.push(`- ${translator.name}`)
      );
    }
  }

  if (review.genre) bookInfo.push(`장르: ${review.genre}`);
  if (review.pub_year) bookInfo.push(`출판년도: ${review.pub_year}`);
  if (review.pages) bookInfo.push(`페이지: ${review.pages}`);
  sections.push(bookInfo.join("\n"));

  // 리뷰 정보 섹션 (수정 가능)
  sections.push("\n# 리뷰 정보\n");
  sections.push(`읽은 날짜: ${review.date_read || "____-__-__"}`);
  sections.push(`평점: ${review.rating || "_"}`);

  // 리뷰 내용 섹션 (수정 가능)
  sections.push("\n# 리뷰 내용\n");
  sections.push(review.review || "이 책에 대한 리뷰를 작성하세요.");

  return sections.join("\n");
}
