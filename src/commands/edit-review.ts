import { Database } from "../db.ts";
import { colors, editWithSystemEditor, bookReviewToMarkdown, parseMarkdownReview } from "../utils/index.ts";

/**
 * vim을 사용하여 리뷰를 편집하는 명령어 구현
 */
export async function handleEditReviewCommand(db: Database, bookId: number): Promise<void> {
  // 책 정보 가져오기
  const books = db.getBooks({ id: bookId });

  if (books.length === 0) {
    console.log(colors.red(`오류: ID가 ${bookId}인 책을 찾을 수 없습니다.`));
    return;
  }

  const book = books[0];

  // 리뷰 가져오기
  const reviews = db.getReviews(bookId);

  if (reviews.length === 0) {
    console.log(colors.yellow(`책 '${book.title}'에는 리뷰가 없습니다. 'libro review ${bookId}' 명령을 사용해 리뷰를 추가하세요.`));
    return;
  }

  // 첫 번째 리뷰 선택 (일반적으로 책당 하나의 리뷰만 있음)
  const review = reviews[0];

  // 마크다운 형식으로 리뷰 데이터 생성
  const reviewData = {
    title: book.title,
    author: book.author,
    genre: book.genre,
    pub_year: book.pub_year,
    pages: book.pages,
    date_read: review.date_read,
    rating: review.rating,
    review: review.review
  };

  const markdownText = bookReviewToMarkdown(reviewData);

  console.log(colors.cyan(`에디터를 시작하여 '${book.title}'의 리뷰를 편집합니다...`));

  // vim을 사용하여 마크다운 편집
  const editedText = await editWithSystemEditor(markdownText);

  if (editedText === null) {
    console.log(colors.red("리뷰 편집이 취소되었습니다."));
    return;
  }

  if (editedText === markdownText) {
    console.log(colors.yellow("변경사항 없음: 리뷰가 수정되지 않았습니다."));
    return;
  }

  // 마크다운에서 데이터 파싱
  const updatedReviewData = parseMarkdownReview(editedText);

  // 리뷰 업데이트
  if (updatedReviewData.rating && updatedReviewData.review) {
    db.updateReview(review.id, {
      rating: updatedReviewData.rating,
      review: updatedReviewData.review || "",
      date_read: updatedReviewData.date_read
    });

    // 책 정보도 업데이트 (선택적으로 변경 가능)
    if (updatedReviewData.title ||
        updatedReviewData.author ||
        updatedReviewData.genre ||
        updatedReviewData.pub_year ||
        updatedReviewData.pages) {
      db.updateBook(bookId, {
        title: updatedReviewData.title,
        author: updatedReviewData.author,
        genre: updatedReviewData.genre,
        pub_year: updatedReviewData.pub_year,
        pages: updatedReviewData.pages
      });
    }

    console.log(colors.green(`리뷰가 성공적으로 업데이트되었습니다.`));
  } else {
    console.log(colors.red("오류: 업데이트된 리뷰에 필수 필드(평점 또는 내용)가 누락되었습니다."));
  }
}
