import { Database } from "../db.ts";
import { createError } from "../utils/errors.ts";

// 모델 타입 정의
export interface Book {
  id?: number;
  title: string;
  pages?: number;
  pub_year?: number;
  genre?: string;
  reviews?: Review[];
}

export interface Review {
  id?: number;
  book_id: number;
  date_read: string;
  rating: number;
  review: string;
}

// 데이터 조회 시 사용될 확장된 Book 타입 (이름 포함)
export interface ExtendedBook extends Book {
  authors: { id: number; name: string }[];
  translators: { id: number; name: string }[];
}

// 책 추가 함수
export const addBook =
  (db: Database) => (book: Omit<Book, "id" | "reviews">): number => {
    try {
      const id = db.addBook({
        title: book.title,
        pages: book.pages,
        pub_year: book.pub_year,
        genre: book.genre,
      });
      return id;
    } catch (error) {
      throw createError(
        `Failed to add book: ${
          error instanceof Error ? error.message : String(error)
        }`,
        "DATABASE",
      );
    }
  };

// 리뷰 추가 함수
export const addReview = (db: Database) => (review: Review): number => {
  try {
    const id = db.addReview(review);
    return id;
  } catch (error) {
    throw createError(
      `Failed to add review: ${
        error instanceof Error ? error.message : String(error)
      }`,
      "DATABASE",
    );
  }
};

// 트랜잭션으로 책과 리뷰 한 번에 추가
export const addBookWithReview = (db: Database) =>
({ book, review }: {
  book: {
    title: string;
    authors: string[];
    translators?: string[];
    pages?: number;
    pub_year?: number;
    genre?: string;
  };
  review?: Omit<Review, "book_id">;
}): { bookId: number; reviewId?: number } => {
  try {
    // 책 추가
    // Get or add author and translator IDs
    const bookId = db.addBook({
      title: book.title,
      pages: book.pages,
      pub_year: book.pub_year,
      genre: book.genre,
    });

    // Add authors
    for (const authorName of book.authors) {
      const authorId = db.getOrAddWriter(authorName, "author");
      db.addBookWriterLink(bookId, authorId, "author");
    }

    // Add translators
    if (book.translators) {
      for (const translatorName of book.translators) {
        const translatorId = db.getOrAddWriter(translatorName, "translator");
        db.addBookWriterLink(bookId, translatorId, "translator");
      }
    }

    let reviewId;
    // 리뷰가 있으면 추가
    if (review) {
      reviewId = db.addReview({
        ...review,
        book_id: bookId,
      });
    }

    return { bookId, reviewId };
  } catch (error) {
    throw createError(
      `Transaction failed: ${
        error instanceof Error ? error.message : String(error)
      }`,
      "DATABASE",
    );
  }
};

// 책 조회 함수
export const getBooks = (db: Database) =>
(
  filter?: { id?: number; year?: number },
): ExtendedBook[] => {
  try {
    return db.getBooks(filter);
  } catch (error) {
    throw createError(
      `Failed to get books: ${
        error instanceof Error ? error.message : String(error)
      }`,
      "DATABASE",
    );
  }
};

// 리뷰 가져오기
export const getReviews = (db: Database) => (bookId: number): Review[] => {
  try {
    return db.getReviews(bookId);
  } catch (error) {
    throw createError(
      `Failed to get reviews: ${
        error instanceof Error ? error.message : String(error)
      }`,
      "DATABASE",
    );
  }
};
