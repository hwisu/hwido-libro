use rusqlite::{Connection, Result};

/// DB 초기화: 파일 경로로 연결 후, 기본 테이블 생성
pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        BEGIN;
        CREATE TABLE IF NOT EXISTS books (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            title     TEXT    NOT NULL,
            pages     INTEGER,
            pub_year  INTEGER,
            genre     TEXT    NOT NULL
        );
        CREATE TABLE IF NOT EXISTS reviews (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            book_id    INTEGER NOT NULL,
            date_read  TEXT,
            rating     INTEGER,
            review     TEXT,
            FOREIGN KEY(book_id) REFERENCES books(id)
        );
        CREATE TABLE IF NOT EXISTS writers (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            type TEXT NOT NULL CHECK (type IN ('author', 'translator'))
        );
        CREATE TABLE IF NOT EXISTS book_writers (
            book_id   INTEGER NOT NULL,
            writer_id INTEGER NOT NULL,
            type      TEXT    NOT NULL CHECK (type IN ('author', 'translator')),
            PRIMARY KEY (book_id, writer_id, type),
            FOREIGN KEY(book_id) REFERENCES books(id) ON DELETE CASCADE,
            FOREIGN KEY(writer_id) REFERENCES writers(id) ON DELETE CASCADE
        );
        COMMIT;
        ",
    )?;
    Ok(conn)
}
