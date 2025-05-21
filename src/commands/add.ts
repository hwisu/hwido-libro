import { Database } from "../db.ts";
import { colors } from "../utils/index.ts";
import { Confirm, Input, Select } from "@cliffy/prompt";
import { addBook, addBookWithReview } from "../lib/db-operations.ts";
import {
  createError,
  errorHandler,
  validateNumber,
  validatePattern,
  validateRequired,
} from "../utils/errors.ts";
import { toInt } from "../utils/fp.ts";

/**
 * Handles the 'add' command to add a new book
 */

// 프롬프트 도우미 함수들 - 화살표 함수와 ES 기능 최대 활용
const promptBook = async () => {
  try {
    // 필수 입력
    const title = await Input.prompt({ message: "Title", minLength: 1 });
    validateRequired(title, "Title");

    // 여러 작가 입력 지원
    const author = await Input.prompt({
      message: "Author(s) (comma-separated)",
      minLength: 1,
    });
    validateRequired(author, "Author");

    // 번역가 입력 추가
    const translator = await Input.prompt({
      // 여러 번역가 입력 지원
      message: "Translator(s) (comma-separated, optional)",
      default: "",
    });

    // 선택 입력
    const pagesStr = await Input.prompt({
      message: "Pages (optional)",
      default: "",
    });
    const pages = toInt(pagesStr);
    if (pagesStr && pages === undefined) {
      throw createError("Pages must be a valid number", "VALIDATION", {
        field: "pages",
      });
    }

    const yearStr = await Input.prompt({
      message: "Publication year (optional)",
      default: "",
    });
    const pub_year = toInt(yearStr);
    if (yearStr && pub_year === undefined) {
      throw createError(
        "Publication year must be a valid number",
        "VALIDATION",
        { field: "year" },
      );
    }

    const genre = await Select.prompt({
      message: "Genre",
      options: [
        { name: "Fiction", value: "Fiction" },
        { name: "Nonfiction", value: "Nonfiction" },
        { name: "Science Fiction", value: "Sci-Fi" },
        { name: "Fantasy", value: "Fantasy" },
        { name: "Mystery", value: "Mystery" },
        { name: "Biography", value: "Biography" },
        { name: "Other", value: "" },
      ],
      default: "Fiction",
    });

    return {
      title,
      author,
      translator,
      pages,
      pub_year,
      genre: genre || undefined,
    };
  } catch (e) {
    if (e instanceof Error && e.name === "AbortError") {
      throw createError("Book input cancelled", "USER_INPUT");
    }
    throw e;
  }
};

// 리뷰 정보 수집
const promptReview = async () => {
  try {
    // 리뷰 추가 여부 확인
    const shouldAddReview = await Confirm.prompt({
      message: "Add a review?",
      default: false,
    });

    if (!shouldAddReview) {
      return null;
    }

    // 날짜 입력
    const dateRead = await Input.prompt({
      message: "Date read (YYYY-MM-DD)",
      default: new Date().toISOString().split("T")[0],
    });
    validatePattern(
      dateRead,
      /^\d{4}-\d{2}-\d{2}$/,
      "Date",
      "Invalid date format. Please use YYYY-MM-DD",
    );

    // 평점 입력
    const ratingStr = await Input.prompt({
      message: "Rating (1-5)",
      default: "3",
    });
    const rating = toInt(ratingStr);
    validateNumber(rating, "Rating", { min: 1, max: 5 });

    // 리뷰 텍스트 입력
    const reviewText = await Input.prompt({
      message: "Review (optional)",
      default: "",
    });

    return {
      date_read: dateRead,
      rating: rating as number, // 이미 검증했으므로 안전
      review: reviewText,
    };
  } catch (e) {
    if (e instanceof Error && e.name === "AbortError") {
      throw createError("Review input cancelled", "USER_INPUT");
    }
    throw e;
  }
};

// 메인 핸들러 함수
export async function handleAddCommand(db: Database): Promise<void> {
  try {
    // 1. 책 정보 수집
    const book = await promptBook(); // book now contains author/translator names string

    // Parse authors and translators strings into arrays
    const authors = book.author.split(",").map((name) => name.trim()).filter(
      (name) => name.length > 0,
    );
    const translators = book.translator
      ? book.translator.split(",").map((name) => name.trim()).filter((name) =>
        name.length > 0
      )
      : [];

    // 2. 리뷰 정보 수집
    const review = await promptReview();

    // 3. DB에 저장
    if (review) {
      // 책과 리뷰 함께 저장
      const { bookId, reviewId } = addBookWithReview(db)({
        book: {
          title: book.title,
          authors: authors,
          translators: translators,
          pages: book.pages,
          pub_year: book.pub_year,
          genre: book.genre,
        },
        review: review,
      });
      console.log(colors.green(`Book added with ID: ${bookId}`));
      if (reviewId) {
        console.log(colors.green(`Review added with ID: ${reviewId}`));
      }
    } else {
      // 책만 저장
      // Add book only (without review)
      const bookId = addBook(db)({
        title: book.title,
        pages: book.pages,
        pub_year: book.pub_year,
        genre: book.genre,
      });

      // Add authors
      for (const authorName of authors) {
        const authorId = db.getOrAddWriter(authorName, "author");
        db.addBookWriterLink(bookId, authorId, "author");
      }

      // Add translators
      for (const translatorName of translators) {
        const translatorId = db.getOrAddWriter(translatorName, "translator");
        db.addBookWriterLink(bookId, translatorId, "translator");
      }

      console.log(colors.green(`Book added with ID: ${bookId}`));
    }
  } catch (error) {
    errorHandler(error);
  }
}
