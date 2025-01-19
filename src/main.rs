pub(crate) mod area;
pub(crate) mod util;

use std::fs::{remove_file, File};

use reqwest::{self};
use rusqlite::Connection;
use scraper::{Html, Selector};
use util::{insert_city, insert_prefecture};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_file_path = "location.db";
    if File::open(db_file_path).is_ok() {
        remove_file(db_file_path).unwrap_or_else(|_| panic!("Can not remove file: {}.", db_file_path));
    };
    // create database
    let conn = Connection::open("location.db")?;
    conn.execute(
        "CREATE TABLE prefecture (
            id      INTEGER NOT NULL PRIMARY KEY,
            name    TEXT NOT NULL,
            read    TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE city (
        id  INTEGER  NOT NULL PRIMARY KEY,
        prefecture_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        read TEXT NOT NULL,
        FOREIGN KEY(prefecture_id) REFERENCES prefecture(id)
    )
    ",
        (),
    )?;

    let html = reqwest::get("https://www.gaoshukai.com/20/15/0031/")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);

    let row_selector = Selector::parse("table tr").expect("Failed to parse selector");

    let mut prefecture_counts = 0;
    for row in document.select(&row_selector) {
        let cell_selector = Selector::parse("td").unwrap();
        let cells = row.select(&cell_selector).collect::<Vec<_>>();
        if !cells.is_empty() {
            if !cells
                .get(3)
                .unwrap()
                .text()
                .collect::<String>()
                .trim().is_empty()
            {
                // it is a city
                let id_str = cells.first().unwrap().text().collect::<String>();
                let id = id_str.parse::<u32>().expect("Can not cast id_str to u32");
                let parent_id = prefecture_counts;
                let new_city = util::create_city(id, parent_id, &cells);
                insert_city(&conn, "city", &new_city).unwrap_or_else(|_| panic!("Can not insert into city table. city: {:#?}", new_city));
            } else {
                // it is a prefecture
                let new_prefecture = util::create_prefecture(prefecture_counts + 1, &cells);
                prefecture_counts += 1;
                insert_prefecture(&conn, "prefecture", &new_prefecture).unwrap_or_else(|_| panic!("Can not insert into prefecture table. prefecture: {:#?}", new_prefecture));
            }
        }
    }

    conn.close().expect("Database connection close failed.");
    Ok(())
}
