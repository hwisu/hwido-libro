import { Database } from "../db.ts";
import { colors } from "../utils/index.ts";
import { Table } from "@cliffy/table";
import { ExtendedBook } from "../lib/db-operations.ts";

export interface ShowOptions {
  id?: number;
  year?: number;
  json?: boolean;
}

type RowType = [string, string, string, string, string, string, string, string, string];

/**
 * Handles the 'show' command to display books
 * Can show all books, books from a specific year, or details of a specific book
 */
export function handleShowCommand(db: Database, options: ShowOptions): void {
  const { id, year, json } = options;

  const books = db.getBooks({ id, year });

  if (books.length === 0) {
    console.log("No books found matching the criteria.");
    return;
  }

  if (json) {
    // Output as JSON
    console.log(JSON.stringify(books, null, 2));
    return;
  }

  // Detailed view for a single book
  if (id) {
    displayDetailedBook(books[0]);
    return;
  }

  displayBookList(books, year);
}

// 단일 책의 상세 정보 표시
const displayDetailedBook = (book: ExtendedBook): void => {
  console.log(
    colors.bold(
      colors.green(`${book.title} (${book.pub_year || "Unknown Year"})`),
    ),
  );

  if (book.authors && book.authors.length > 0) {
    const authorNames = book.authors.map((a) => a.name).join(", ");
    console.log(colors.italic(`by ${authorNames}`));
  }

  if (book.translators && book.translators.length > 0) {
    const translatorNames = book.translators.map((t) => t.name).join(", ");
    console.log(colors.italic(`translated by ${translatorNames}`));
  }

  if (book.pages) {
    console.log(`Pages: ${book.pages}`);
  }

  if (book.genre) {
    console.log(`Genre: ${book.genre}`);
  }

  if (book.reviews && book.reviews.length > 0) {
    console.log("\nReviews:");
    book.reviews.forEach((review, index) => {
      console.log(colors.bold(`Review ${index + 1}:`));
      console.log(`  Date: ${review.date_read}`);
      console.log(
        `  Rating: ${"★".repeat(review.rating)}${
          "☆".repeat(5 - review.rating)
        }`,
      );
      console.log(`  Review: ${review.review}`);
      if (book.reviews && index < book.reviews.length - 1) {
        console.log(""); // Add a blank line between reviews
      }
    });
  } else {
    console.log("\nNo reviews yet.");
  }
};

// 여러 책의 목록 표시
const displayBookList = (books: ExtendedBook[], year?: number): void => {
  const yearTitle = year ? `Books Read in ${year}` : "Books";
  console.log(colors.bold(colors.green(yearTitle)));

  // 장르별로 책 분류
  const isFiction = (book: ExtendedBook): boolean =>
    !book.genre ||
    book.genre.toLowerCase() === "fiction" ||
    book.genre.toLowerCase().includes("sci-fi") ||
    book.genre.toLowerCase() === "fantasy" ||
    book.genre.toLowerCase() === "mystery";

  const isNonfiction = (book: ExtendedBook): boolean =>
    book.genre !== undefined &&
    book.genre.toLowerCase() === "nonfiction";

  const fiction = books.filter(isFiction);
  const nonfiction = books.filter(isNonfiction);

  // Fiction 책 표시
  if (fiction.length > 0) {
    const fictionTable = createBooksTable("Fiction")(fiction);
    console.log(fictionTable.toString());
  }

  // Nonfiction 책 표시
  if (nonfiction.length > 0) {
    const nonfictionTable = createBooksTable("Nonfiction")(nonfiction);
    console.log(nonfictionTable.toString());
  }

  // 어떤 장르에도 맞지 않는 경우 모든 책 표시
  if (fiction.length === 0 && nonfiction.length === 0) {
    const allBooksTable = createBooksTable()(books);
    console.log(allBooksTable.toString());
  }
};

// 테이블 생성 함수
const createBooksTable = (title?: string) => (books: ExtendedBook[]) => {
  if (title) {
    console.log(`\n${title}`);
  }

  const table = new Table()
    .header(["id", "Title", "Genre", "Year", "Pages", "Author", "Rating", "Date Read", "Review"])
    .border(true)
    .padding(1);

  if (title) {
    table.indent(1);
  }

  // 책 정보를 테이블에 추가
  const rows = books.map((book) => {
    // Get the last review if reviews exist
    const review = (book.reviews && book.reviews.length > 0)
      ? book.reviews[book.reviews.length - 1]
      : null;

    // Combine authors and translators for display
    const authorNames = book.authors.map((a) => a.name).join(", ");
    const translatorNames = book.translators.map((t) => t.name).join(", ");
    const writerDisplay = authorNames +
      (translatorNames ? ` / ${translatorNames}` : "");

    return [
      book.id?.toString() ?? "", // Safely access and convert id, default to empty string if undefined
      book.title,
      book.genre ?? "unknown",
      book.pub_year ? book.pub_year.toString() : "unknown",
      book.pages ? book.pages.toString() : "unknown",
      writerDisplay,
      review ? review.rating.toString() : "",
      review
        ? new Date(review.date_read).toLocaleDateString("en-US", {
          month: "short",
          day: "2-digit",
          year: "numeric",
        })
        : "",
      review ? review.review : "",
    ] as RowType;
  });

  rows.forEach((row) => table.push(row));

  return table;
};
