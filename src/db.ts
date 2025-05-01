import { DB } from "sqlite";

/**
 * Runs migrations and exposes high-level CRUD helpers for books and reviews.
 */
export class Database {
  #db: DB;

  constructor(path = "libro.db") {
    this.#db = new DB(path);
    this.migrate();
  }

  /** Apply schema updates if version changed. */
  migrate() {
    // Ensure foreign key constraints are enabled.
    this.#db.execute("PRAGMA foreign_keys = ON;");
    // Get the current schema version.
    const [[oldVersion]] = this.#db.query<[number]>("PRAGMA user_version");
    // Initial migration to version 1.
    if (oldVersion < 1) {
      this.#db.execute(`
        CREATE TABLE IF NOT EXISTS books (
          id        INTEGER PRIMARY KEY AUTOINCREMENT,
          title     TEXT    NOT NULL,
          author    TEXT    NOT NULL,
          pages     INTEGER,
          pub_year  INTEGER,
          genre     TEXT
        );

        CREATE TABLE IF NOT EXISTS reviews (
          id         INTEGER PRIMARY KEY AUTOINCREMENT,
          book_id    INTEGER NOT NULL,
          date_read  DATE,
          rating     INTEGER,
          review     TEXT,
          FOREIGN KEY(book_id) REFERENCES books(id)
        );
      `);
      this.#db.execute("PRAGMA user_version = 1;");
    }
  }

  addBook(book: {
    title: string;
    author: string;
    pages?: number;
    pub_year?: number;
    genre?: string;
  }): number {
    const { title, author, pages, pub_year, genre } = book;
    this.#db.query(
      `INSERT INTO books (title, author, pages, pub_year, genre) VALUES (?, ?, ?, ?, ?);`,
      [title, author, pages ?? null, pub_year ?? null, genre ?? null],
    );
    return this.#db.lastInsertRowId;
  }

  addReview(review: {
    book_id: number;
    date_read?: string;
    rating: number;
    review: string;
  }): number {
    const date = review.date_read ?? new Date().toISOString().split("T")[0];
    this.#db.query(
      `INSERT INTO reviews (book_id, date_read, rating, review) VALUES (?, ?, ?, ?);`,
      [review.book_id, date, review.rating, review.review],
    );
    return this.#db.lastInsertRowId;
  }

  /**
   * Fetches books. With filter.id returns a single book with its reviews.
   * With filter.year returns all books from that year.
   * Without filters returns all books.
   */
  getBooks(filter?: { id?: number; year?: number }): Array<any> {
    if (filter?.id) {
      const row = this.#db.query<[number, string, string, number | null, number | null, string | null]>(
        `SELECT id, title, author, pages, pub_year, genre FROM books WHERE id = ?;`,
        [filter.id],
      )[0];
      if (!row) return [];
      const [id, title, author, pages, pub_year, genre] = row;
      const book = { id, title, author, pages, pub_year, genre };
      const reviews = this.#db.query<[number, number, string, number, string]>(
        `SELECT id, book_id, date_read, rating, review FROM reviews WHERE book_id = ? ORDER BY date_read;`,
        [id],
      ).map(([rid, bid, date_read, rating, text]) => ({
        id: rid,
        book_id: bid,
        date_read,
        rating,
        review: text,
      }));
      return [{ ...book, reviews }];
    }

    const query = filter?.year
      ? `SELECT id, title, author, pages, pub_year, genre FROM books WHERE pub_year = ? ORDER BY title;`
      : `SELECT id, title, author, pages, pub_year, genre FROM books ORDER BY title;`;
    const params = filter?.year ? [filter.year] : [];
    const rows = this.#db.query<[number, string, string, number | null, number | null, string | null]>(query, params);
    return rows.map(([id, title, author, pages, pub_year, genre]) => ({
      id,
      title,
      author,
      pages,
      pub_year,
      genre,
    }));
  }
}
