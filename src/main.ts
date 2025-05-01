#!/usr/bin/env -S deno run --allow-read --allow-write --allow-ffi

import { Command } from "cliffy/command/mod.ts";
import { DB } from "sqlite";

const db = new DB("libro.db");
// 테이블 생성 (최초 실행 시)
db.query(`
  CREATE TABLE IF NOT EXISTS books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    pages INTEGER,
    pub_year INTEGER,
    genre TEXT
  );
`);
db.query(`
  CREATE TABLE IF NOT EXISTS reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    book_id INTEGER,
    date_read DATE,
    rating INTEGER,
    review TEXT,
    FOREIGN KEY(book_id) REFERENCES books(id)
  );
`);

await new Command()
  .name("libro")
  .version("0.1.0")
  .description("A simple reading tracker CLI built with Deno")
  .command("add", "Add a new book")
    .action(async () => {
      // 사용자 입력 로직…
    })
  .command("show", "Show book(s)")
    .option("--year <year:number>", "Show by year")
    .action((options) => {
      // 조회 로직…
    })
  .command("report", "Generate reports")
    .option("--author", "Report by author")
    .action((options) => {
      // 리포트 로직…
    })
  .parse(Deno.args);
