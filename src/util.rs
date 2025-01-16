
use rusqlite::Connection;
use scraper::ElementRef;

use crate::area::City;
use crate::area::Prefecture;

pub fn create_prefecture(id: u32, cells: &Vec<ElementRef<'_>>) -> Prefecture {
    let name = cells.get(2).unwrap().text().collect::<String>();
    let read = cells.get(4).unwrap().text().collect::<String>();
    Prefecture { name, read, id }
}

pub fn create_city(id: u32, parent_id: u32, cells: &Vec<ElementRef<'_>>) -> City {
    let name = cells.get(3).unwrap().text().collect::<String>();
    let read = cells.get(4).unwrap().text().collect::<String>();
    City {
        id,
        parent_id,
        name,
        read,
    }
}

pub fn insert_prefecture(
    conn: &Connection,
    table_name: &str,
    prefecture: &Prefecture,
) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO {} (id, name, read) VALUES (?1, ?2, ?3)",
            table_name
        ),
        (&prefecture.id, &prefecture.name, &prefecture.read),
    )
    .expect("insert_prefecture error.");
    Ok(())
}

pub fn insert_city(conn: &Connection, table_name: &str, city: &City) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO {} (id, parent_id, name, read) VALUES (?1, ?2, ?3, ?4)",
            table_name
        ),
        (&city.id, &city.parent_id, &city.name, &city.read),
    )
    .expect("insert_city error.");
    Ok(())
}
