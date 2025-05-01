import { Database } from "../db.ts";
import { colors, Table, barChart } from "../utils/index.ts";

export interface ReportOptions {
  author?: string | boolean;
  year?: number | boolean;
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
export function handleReportCommand(db: Database, options: ReportOptions): void {
  const { author, year } = options;

  // Get all books
  const books = db.getBooks() as Book[];

  if (books.length === 0) {
    console.log("No books in the database to generate reports.");
    return;
  }

  console.log(colors.bold(colors.green(`Library Statistics (${books.length} books)`)));

  if (author) {
    authorReport(books, typeof author === 'string' ? author : undefined);
  } else if (year) {
    yearReport(books, typeof year === 'number' ? year : undefined);
  } else {
    // Default to a summary report with tables
    console.log(`\nTotal books: ${books.length}`);

    // Count books with reviews
    const booksWithReviews = books.filter(book => book.reviews && book.reviews.length > 0);
    console.log(`Books with reviews: ${booksWithReviews.length}`);

    // Average rating
    const allReviews = books.flatMap(book => book.reviews || []);
    const totalRating = allReviews.reduce((sum, review) => sum + review.rating, 0);
    const avgRating = allReviews.length ? (totalRating / allReviews.length).toFixed(1) : "N/A";
    console.log(`Average rating: ${avgRating}`);

    // Authors count
    const uniqueAuthors = new Set(books.map(book => book.author));
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
        sort: "desc"
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
    ? books.filter(book => book.author.toLowerCase().includes(filterAuthor.toLowerCase()))
    : books;

  if (filterAuthor && filteredBooks.length === 0) {
    console.log(`No books found by author matching "${filterAuthor}".`);
    return;
  }

  if (filterAuthor) {
    console.log(`\nBooks by author matching "${filterAuthor}":`);

    const table = new Table({ zebra: true });
    table.header(["Title", "Year", "Genre"]);

    filteredBooks.forEach(book => {
      table.row([book.title, book.pub_year?.toString() || "Unknown", book.genre || "Unknown"]);
    });

    table.render();
    return;
  }

  // Group books by author
  const authorBooks = filteredBooks.reduce((acc, book) => {
    acc[book.author] = acc[book.author] || [];
    acc[book.author].push(book);
    return acc;
  }, {} as Record<string, Book[]>);

  // Create a count map for barChart
  const authorCounts: Record<string, number> = {};
  Object.entries(authorBooks).forEach(([author, books]) => {
    authorCounts[author] = books.length;
  });

  console.log("\nBooks by Author:");
  console.log(barChart(authorCounts, {
    maxBarWidth: 25,
    sort: "desc",
    colorize: true
  }));

  // Add a detail table with the top 5 authors
  const top5Authors = Object.entries(authorBooks)
    .sort((a, b) => b[1].length - a[1].length)
    .slice(0, 5);

  if (top5Authors.length > 0) {
    console.log("\nTop authors and their books:");

    const table = new Table({ zebra: true });
    table.header(["Author", "Books"]);

    top5Authors.forEach(([author, books]) => {
      table.row([author, books.map(b => b.title).join(", ")]);
    });

    table.render();
  }
}

/**
 * Generate a report of books grouped by publication year
 */
function yearReport(books: Book[], filterYear?: number): void {
  // Filter out books without publication year
  const booksWithYear = books.filter(book => book.pub_year);

  if (booksWithYear.length === 0) {
    console.log("No books with publication year data.");
    return;
  }

  // If filterYear is provided, only show books from that year
  const filteredBooks = filterYear
    ? booksWithYear.filter(book => book.pub_year === filterYear)
    : booksWithYear;

  if (filterYear && filteredBooks.length === 0) {
    console.log(`No books found from year ${filterYear}.`);
    return;
  }

  if (filterYear) {
    console.log(`\nBooks from ${filterYear}:`);

    const table = new Table({ zebra: true });
    table.header(["Title", "Author"]);

    filteredBooks.forEach(book => {
      table.row([book.title, book.author]);
    });

    table.render();
    return;
  }

  // Group books by year
  const yearBooks = filteredBooks.reduce((acc, book) => {
    const year = book.pub_year as number;
    acc[year] = acc[year] || [];
    acc[year].push(book);
    return acc;
  }, {} as Record<number, Book[]>);

  // Create a count map for barChart
  const yearCounts: Record<string, number> = {};
  Object.entries(yearBooks).forEach(([year, books]) => {
    yearCounts[year] = books.length;
  });

  console.log("\nBooks by Year:");
  console.log(barChart(yearCounts, {
    sort: "desc",
    maxBarWidth: 30
  }));
}
