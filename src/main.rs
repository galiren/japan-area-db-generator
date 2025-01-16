pub(crate) mod area;
pub(crate) mod util;

use std::fs::{remove_file, File};

use reqwest::{self};
use rusqlite::Connection;
use scraper::{Html, Selector};
use util::{insert_city, insert_prefecture};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greeting_file = File::open("location.db").map(|file| {
        remove_file("location.db").expect("Delete existing db file failed.");
    });
    // create database
    let conn = Connection::open("location.db")?;
    conn.execute(
        "CREATE TABLE prefecture (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            read    TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE city (
        id  INTEGER PRIMARY KEY,
        parent_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        read TEXT NOT NULL,
        FOREIGN KEY(parent_id) REFERENCES prefecture(id)
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

    let count = 0;
    let mut prefecture_counts = 0;
    for row in document.select(&row_selector) {
        let cell_selector = Selector::parse("td").unwrap();
        let cells = row.select(&cell_selector).collect::<Vec<_>>();
        if !cells.is_empty() {
            if cells
                .get(3)
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .len()
                != 0
            {
                // it is a city
                let id_str = cells.get(0).unwrap().text().collect::<String>();
                let id = id_str.parse::<u32>().expect("Can not cast id_str to u32");
                let parent_id = prefecture_counts;
                let new_city = util::create_city(id, parent_id, &cells);
                insert_city(&conn, "city", &new_city).unwrap();
            } else {
                // it is a prefecture
                let new_prefecture = util::create_prefecture(prefecture_counts + 1, &cells);
                prefecture_counts += 1;
                insert_prefecture(&conn, "prefecture", &new_prefecture).unwrap();
            }
        }
    }

    conn.close().unwrap();
    Ok(())
}
