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

  /** Close the database connection */
  close(): void {
    this.#db.close();
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

    // Migration to version 2: Add translator column to books table.
    // This migration is now superseded by version 3 and 4, but keeping the version check.
    if (oldVersion < 2) {
      // The actual migration logic for version 2 is now part of version 3/4 data migration
      // This block primarily serves to increment the user_version if it's stuck at 1.
      // We should not re-add the translator TEXT column here.
      // If the user_version is 1, just set it to 2 so version 3 can run.
      // Check if translator column exists before trying to add it (robustness)
      try {
        this.#db.query("SELECT translator FROM books LIMIT 1");
        console.log(
          "Translator column already exists, skipping add in v2 check.",
        );
      } catch (e) {
        console.log(e);
        // Column does not exist, safely try to add it if oldVersion was 1
        if (oldVersion < 2) {
          try {
            this.#db.execute(`ALTER TABLE books ADD COLUMN translator TEXT;`);
            console.log("Added translator column in v2 check.");
          } catch (addError) {
            console.warn(
              "Could not add translator column in v2 check:",
              addError,
            ); /* ignore */
          }
        }
      }
      this.#db.execute("PRAGMA user_version = 2;");
    }

    // Migration to version 3: Introduce writers table and add author_id/translator_id to books (transitional)
    if (oldVersion < 3) {
      console.log("Running migration to version 3...");
      // Create writers table if it doesn't exist
      this.#db.execute(`
        CREATE TABLE IF NOT EXISTS writers (
          id   INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL UNIQUE,
          type TEXT NOT NULL CHECK (type IN ('author', 'translator'))
        );
      `);

      // Check if columns exist before adding them using PRAGMA table_info
      const columns = this.#db.query<
        [number, string, string, number, number, number, number]
      >("PRAGMA table_info(books);");
      const columnNames = columns.map((col) => col[1]); // Column name is at index 1

      if (!columnNames.includes("author_id")) {
        try {
          this.#db.execute(`ALTER TABLE books ADD COLUMN author_id INTEGER;`);
          console.log("Added author_id column in v3.");
        } catch (e) {
          console.warn("Could not add author_id column in v3:", e); /* ignore */
        }
      } else {
        console.log("author_id column already exists, skipping add in v3.");
      }

      if (!columnNames.includes("translator_id")) {
        try {
          this.#db.execute(
            `ALTER TABLE books ADD COLUMN translator_id INTEGER;`,
          );
          console.log("Added translator_id column in v3.");
        } catch (e) {
          console.warn(
            "Could not add translator_id column in v3:",
            e,
          ); /* ignore */
        }
      } else {
        console.log("translator_id column already exists, skipping add in v3.");
      }

      // Note: Data migration from old author/translator TEXT columns to new ID columns
      // and dropping old columns are handled in the version 4 migration,
      // as version 3 is now primarily about creating the necessary ID columns
      // before the full many-to-many migration in version 4.

      this.#db.execute("PRAGMA user_version = 3;");
      console.log("Migration to version 3 complete.");
    }

    // Migration to version 4: Implement many-to-many relationship for writers.
    if (oldVersion < 4) {
      console.log("Running migration to version 4...");
      // Create book_writers linking table
      this.#db.execute(`
        CREATE TABLE IF NOT EXISTS book_writers (
          book_id INTEGER NOT NULL,
          writer_id INTEGER NOT NULL,
          type TEXT NOT NULL CHECK (type IN ('author', 'translator')),
          PRIMARY KEY (book_id, writer_id, type), -- Composite primary key
          FOREIGN KEY(book_id) REFERENCES books(id) ON DELETE CASCADE,
          FOREIGN KEY(writer_id) REFERENCES writers(id) ON DELETE CASCADE
        );
      `);

      // Migrate data from books (author_id) to book_writers
      // Ensure this only runs if author_id column exists (implies version 3 ran)
      try {
        // Check if column exists by trying a query
        this.#db.query("SELECT author_id FROM books LIMIT 1");
        console.log(
          "author_id column exists, proceeding with data migration to book_writers.",
        );
        this.#db.execute(`
          INSERT INTO book_writers (book_id, writer_id, type)
          SELECT id, author_id, 'author' FROM books WHERE author_id IS NOT NULL;
        `);
        console.log("Migrated author_id data to book_writers.");
      } catch (e) {
        console.log(
          "author_id column not found or error during migration to book_writers:",
          e,
        );
        /* author_id might not exist if migration 3 failed previously, ignore data migration */
      }

      // Migrate data from books (translator_id) to book_writers
      // Ensure this only runs if translator_id column exists
      try {
        // Check if column exists by trying a query
        this.#db.query("SELECT translator_id FROM books LIMIT 1");
        console.log(
          "translator_id column exists, proceeding with data migration to book_writers.",
        );
        this.#db.execute(`
          INSERT INTO book_writers (book_id, writer_id, type)
          SELECT id, translator_id, 'translator' FROM books WHERE translator_id IS NOT NULL AND TRIM(translator) != '';
        `);
        console.log("Migrated translator_id data to book_writers.");
      } catch (e) {
        console.log(
          "translator_id column not found or error during migration to book_writers:",
          e,
        );
        /* translator_id might not exist, ignore data migration */
      }

      // Recreate books table without author_id and translator_id
      // Temporarily disable foreign key constraints for recreation
      this.#db.execute("PRAGMA foreign_keys = OFF;");

      this.#db.execute(`
        CREATE TABLE books_new (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          title TEXT NOT NULL,
          pages INTEGER,
          pub_year INTEGER,
          genre TEXT
        );
      `);

      // Copy data to the new books table
      this.#db.execute(`
        INSERT INTO books_new (id, title, pages, pub_year, genre)
        SELECT id, title, pages, pub_year, genre FROM books;
      `);

      // Drop the old books table
      this.#db.execute(`DROP TABLE books;`);

      // Rename the new table
      this.#db.execute(`ALTER TABLE books_new RENAME TO books;`);

      // Re-enable foreign key constraints
      this.#db.execute("PRAGMA foreign_keys = ON;");

      // Update user_version
      this.#db.execute("PRAGMA user_version = 4;");
      console.log("Migration to version 4 complete.");
    }
  }

  /**
   * Gets the ID for a writer (author or translator) by name, adding them if they don't exist.
   */
  getOrAddWriter(name: string, type: "author" | "translator"): number {
    // Check if writer already exists
    const existingWriter = this.#db.query<[number]>(
      `SELECT id FROM writers WHERE name = ? AND type = ?;`,
      [name, type],
    )[0];

    if (existingWriter) {
      return existingWriter[0];
    } else {
      // Add new writer
      this.#db.query(
        `INSERT INTO writers (name, type) VALUES (?, ?);`,
        [name, type],
      );
      return this.#db.lastInsertRowId;
    }
  }

  /**
   * Adds a link between a book and a writer (author or translator) in the book_writers table.
   */
  addBookWriterLink(
    bookId: number,
    writerId: number,
    type: "author" | "translator",
  ): void {
    this.#db.query(
      `INSERT INTO book_writers (book_id, writer_id, type) VALUES (?, ?, ?);`,
      [bookId, writerId, type],
    );
  }

  addBook(book: {
    title: string;
    pages?: number;
    pub_year?: number;
    genre?: string;
  }): number {
    const { title, pages, pub_year, genre } = book;
    this.#db.query(
      `INSERT INTO books (title, pages, pub_year, genre) VALUES (?, ?, ?, ?);`,
      [title, pages ?? null, pub_year ?? null, genre ?? null],
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
    const query = `
      SELECT DISTINCT
        b.id, b.title, b.pages, b.pub_year, b.genre,
        w.id AS writer_id, w.name AS writer_name, w.type AS writer_type
      FROM books b
      LEFT JOIN book_writers bw ON b.id = bw.book_id
      LEFT JOIN writers w ON bw.writer_id = w.id
      ${
      filter?.id ? "WHERE b.id = ?" : filter?.year ? "WHERE b.pub_year = ?" : ""
    }
      ORDER BY b.id, w.type, w.name
    `;

    const params = filter?.id ? [filter.id] : filter?.year ? [filter.year] : [];
    const rows = this.#db.query(query, params);
    const booksMap = new Map<number, any>();

    for (const row of rows) {
      const [
        bookId,
        title,
        pages,
        pub_year,
        genre,
        writer_id,
        writer_name,
        writer_type,
      ] = row;

      if (!booksMap.has(bookId as number)) {
        const reviews = this.#db.query(
          `
          SELECT review, rating, date_read
          FROM reviews
          WHERE book_id = ?
          ORDER BY date_read DESC
        `,
          [bookId],
        ).map(([review, rating, date_read]) => ({
          review,
          rating,
          date_read,
        }));

        booksMap.set(bookId as number, {
          id: bookId,
          title,
          pages,
          pub_year,
          genre,
          authors: [],
          translators: [],
          reviews: reviews,
        });
      }

      if (writer_id !== null) {
        const writer = { id: writer_id, name: writer_name };
        const book = booksMap.get(bookId as number);
        if (writer_type === "author") {
          book.authors.push(writer);
        } else if (writer_type === "translator") {
          book.translators.push(writer);
        }
      }
    }

    return Array.from(booksMap.values());
  }

  /**
   * Update an existing book in the database
   */
  updateBook(bookId: number, book: {
    title?: string;
    pages?: number;
    pub_year?: number;
    genre?: string;
    // Note: Updating authors/translators via updateBook is not supported in this scheme.
    // Separate methods would be needed to add/remove book_writers links.
    // For now, updateBook only handles core book fields.
  }): void {
    const updateParts = [];
    const params = [];

    // 업데이트할 필드만 쿼리에 포함
    if (book.title !== undefined) {
      updateParts.push("title = ?");
      params.push(book.title);
    }
    if (book.pages !== undefined) {
      updateParts.push("pages = ?");
      params.push(book.pages);
    }
    if (book.pub_year !== undefined) {
      updateParts.push("pub_year = ?");
      params.push(book.pub_year);
    }
    if (book.genre !== undefined) {
      updateParts.push("genre = ?");
      params.push(book.genre);
    }

    // 업데이트할 항목이 없으면 종료
    if (updateParts.length === 0) {
      return;
    }

    // 마지막 매개변수로 bookId 추가
    params.push(bookId);

    // 쿼리 실행
    this.#db.query(
      `UPDATE books SET ${updateParts.join(", ")} WHERE id = ?;`,
      params,
    );
  }

  /**
   * Get all reviews for a book
   */
  getReviews(bookId: number): Array<{
    id: number;
    book_id: number;
    date_read: string;
    rating: number;
    review: string;
  }> {
    return this.#db.query<[number, number, string, number, string]>(
      `SELECT id, book_id, date_read, rating, review FROM reviews WHERE book_id = ? ORDER BY date_read;`,
      [bookId],
    ).map(([id, book_id, date_read, rating, review]) => ({
      id,
      book_id,
      date_read,
      rating,
      review,
    }));
  }

  /**
   * Update an existing review in the database
   */
  updateReview(reviewId: number, review: {
    book_id?: number;
    date_read?: string;
    rating?: number;
    review?: string;
  }): void {
    const updateParts = [];
    const params = [];

    // 업데이트할 필드만 쿼리에 포함
    if (review.book_id !== undefined) {
      updateParts.push("book_id = ?");
      params.push(review.book_id);
    }
    if (review.date_read !== undefined) {
      updateParts.push("date_read = ?");
      params.push(review.date_read);
    }
    if (review.rating !== undefined) {
      updateParts.push("rating = ?");
      params.push(review.rating);
    }
    if (review.review !== undefined) {
      updateParts.push("review = ?");
      params.push(review.review);
    }

    // 업데이트할 항목이 없으면 종료
    if (updateParts.length === 0) {
      return;
    }

    // 마지막 매개변수로 reviewId 추가
    params.push(reviewId);

    // 쿼리 실행
    this.#db.query(
      `UPDATE reviews SET ${updateParts.join(", ")} WHERE id = ?;`,
      params,
    );
  }

  /**
   * Gets all writers (authors and translators) for a specific book
   */
  getBookWriters(
    bookId: number,
  ): Array<{ name: string; type: "author" | "translator" }> {
    return this.#db.query<[string, "author" | "translator"]>(
      `
      SELECT w.name, bw.type
      FROM writers w
      JOIN book_writers bw ON w.id = bw.writer_id
      WHERE bw.book_id = ?
      ORDER BY bw.type, w.name
    `,
      [bookId],
    ).map(([name, type]) => ({ name, type }));
  }
}
