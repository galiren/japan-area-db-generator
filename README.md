# `japan-area-db-generator`

It generates location database file of Japan's prefectures and corresponding cities base on data fetched from [日本都道府県・市区町村地名読み方日中辞書](https://www.gaoshukai.com/20/15/0031/).

## SQLite Table Definition

```
TABLE prefecture (
    id      INTEGER PRIMARY KEY,
    name    TEXT NOT NULL,
    read    TEXT NOT NULL
)

TABLE city (
    id  INTEGER PRIMARY KEY,
    parent_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    read TEXT NOT NULL,
    FOREIGN KEY(parent_id) REFERENCES prefecture(id)
)
```

## Dependencies

See [`Cargo.toml`](./Cargo.toml).

## Usage

Run `cargo run --release`, a `location.db` file will be generated in current folder which contains two tables as described above.

## Special Thanks

[gaoshukai](https://www.gaoshukai.com)

## Plans

Ths basic functionality seems has been implemented correctly. As it is my very first project implemented in Rust, the following works will aim at improve code style and add new things I want to do.

## License

[MIT](./LICENSE)
