import { Database } from "../db.ts";
import { colors } from "../utils/index.ts";
import { Table } from "@cliffy/table";
import { barChart } from "../utils/index.ts";

export interface ReportOptions {
  author?: string | boolean;
  year?: number | boolean;
  years?: boolean;
}

// Define book and review interfaces
interface BookReview {
  id: number;
  book_id: number;
  date_read: string;
  rating: number;
  review: string;
}

interface Book {
  id: number;
  title: string;
  author: string;
  pages?: number;
  pub_year?: number;
  genre?: string;
  reviews?: BookReview[];
}

/**
 * Handles the 'report' command to generate statistics about the books
 */
export function handleReportCommand(
  db: Database,
  options: ReportOptions,
): void {
  const { author, year, years } = options;

  // Get all books
  const books = db.getBooks() as Book[];

  if (books.length === 0) {
    console.log("No books in the database to generate reports.");
    return;
  }

  console.log(
    colors.bold(colors.green(`Library Statistics (${books.length} books)`)),
  );

  if (author) {
    authorReport(books, typeof author === "string" ? author : undefined);
  } else if (year) {
    yearReport(books, typeof year === "number" ? year : undefined);
  } else if (years) {
    yearReport(books); // Show years chart without specific year filter
  } else {
    // Default to a summary report with tables
    console.log(`\nTotal books: ${books.length}`);

    // Count books with reviews
    const booksWithReviews = books.filter((book) =>
      book.reviews && book.reviews.length > 0
    );
    console.log(`Books with reviews: ${booksWithReviews.length}`);

    // Average rating
    const allReviews = books.flatMap((book) => book.reviews || []);
    const totalRating = allReviews.reduce(
      (sum, review) => sum + review.rating,
      0,
    );
    const avgRating = allReviews.length
      ? (totalRating / allReviews.length).toFixed(1)
      : "N/A";
    console.log(`Average rating: ${avgRating}`);

    // Authors count
    const uniqueAuthors = new Set(books.map((book) => book.author));
    console.log(`Unique authors: ${uniqueAuthors.size}`);

    // Genre breakdown if available
    const genreCounts = books.reduce((counts, book) => {
      if (book.genre) {
        counts[book.genre] = (counts[book.genre] || 0) + 1;
      }
      return counts;
    }, {} as Record<string, number>);

    if (Object.keys(genreCounts).length > 0) {
      console.log("\nGenre breakdown:");

      // Use barChart to display genre distribution
      console.log(barChart(genreCounts, {
        maxBarWidth: 20,
        sort: "desc",
      }));
    }
  }
}

/**
 * Generate a report of books grouped by author
 */
function authorReport(books: Book[], filterAuthor?: string): void {
  // If filterAuthor is provided, only show books by that author
  const filteredBooks = filterAuthor
    ? books.filter((book) =>
      book.author.toLowerCase().includes(filterAuthor.toLowerCase())
    )
    : books;

  if (filterAuthor && filteredBooks.length === 0) {
    console.log(`No books found by author matching "${filterAuthor}".`);
    return;
  }

  if (filterAuthor) {
    console.log(`\nBooks by author matching "${filterAuthor}":`);

    const authorBooksTable = new Table()
      .header(["Title", "Year", "Genre"])
      .border(true)
      .padding(1);

    filteredBooks.forEach((book) => {
      authorBooksTable.push([
        book.title,
        book.pub_year?.toString() || "Unknown",
        book.genre || "Unknown",
      ]);
    });

    console.log(authorBooksTable.toString());
    return;
  }

  // Group books by author
  const authorBooks = filteredBooks.reduce((acc, book) => {
    acc[book.author] = acc[book.author] || [];
    acc[book.author].push(book);
    return acc;
  }, {} as Record<string, Book[]>);

  // Sort authors by number of books (descending)
  const sortedAuthors = Object.entries(authorBooks)
    .sort((a, b) => b[1].length - a[1].length)
    // Only include authors with multiple books
    .filter(([_, books]) => books.length > 1);

  // Display a simple table in the style of the original Libro
  console.log("\n         Most Read Authors\n");
  console.log("  Author                Books Read");
  console.log(" ──────────────────────────────────");

  sortedAuthors.forEach(([author, books]) => {
    // Format to match original Libro with proper padding
    const authorStr = author.padEnd(20, " ").substring(0, 20);
    const countStr = books.length.toString();
    console.log(`  ${authorStr}  ${countStr}`);
  });

  console.log("");
}

/**
 * Generate a report of books read by year
 */
function yearReport(books: Book[], filterYear?: number): void {
  // Get all books with reviews to determine read dates
  const booksWithReviews = books.filter((book) =>
    book.reviews && book.reviews.length > 0
  );

  if (booksWithReviews.length === 0) {
    console.log("No books with review data to show read dates.");
    return;
  }

  // If a specific year is requested, show details for that year
  if (filterYear) {
    const booksReadInYear = booksWithReviews.filter((book) => {
      const reviewDates = book.reviews?.map((r) => r.date_read) || [];
      return reviewDates.some((date) => date.startsWith(String(filterYear)));
    });

    if (booksReadInYear.length === 0) {
      console.log(`No books read in ${filterYear}.`);
      return;
    }

    console.log(`\nBooks Read in ${filterYear}:`);

    const yearBooksTable = new Table()
      .header(["Title", "Author", "Rating", "Date Read"])
      .border(true)
      .padding(1);

    booksReadInYear.forEach((book) => {
      const review = book.reviews?.[0];
      yearBooksTable.push([
        book.title,
        book.author,
        review?.rating.toString() || "-",
        review?.date_read || "-",
      ]);
    });

    console.log(yearBooksTable.toString());
    return;
  }

  // Group books by year read based on review dates
  const booksByYearRead: Record<string, number> = {};

  booksWithReviews.forEach((book) => {
    book.reviews?.forEach((review) => {
      if (review.date_read) {
        const year = review.date_read.substring(0, 4);
        booksByYearRead[year] = (booksByYearRead[year] || 0) + 1;
      }
    });
  });

  // Sort years
  const sortedYears = Object.keys(booksByYearRead).sort();

  // Display in the style of the original Libro
  console.log("\n                         Books Read by Year\n");
  console.log("  Year   Count   Bar");
  console.log(
    " ───────────────────────────────────────────────────────────────────",
  );

  // Find max for scaling
  const maxBooks = Math.max(...Object.values(booksByYearRead));
  const barScale = 50 / maxBooks; // Scale to max 50 characters

  sortedYears.forEach((year) => {
    const count = booksByYearRead[year];
    const barLength = Math.max(1, Math.round(count * barScale));
    const bar = "▄".repeat(barLength);

    // Format to match original Libro
    console.log(`  ${year}   ${String(count).padEnd(6)}${bar}`);
  });

  console.log("");
}
