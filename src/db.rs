use std::env;

use sqlx::SqlitePool;
use sqlx_core::sqlite::SqliteQueryAs;

use crate::result::{Result, Err};

type SqliteConnPool = sqlx::Pool<sqlx::sqlite::SqliteConnection>;

pub struct DataSource {
    sled: sled::Db,
    sqlite: SqliteConnPool,
}

#[derive(sqlx::FromRow)]
struct Id {
    id: i64,
}

pub async fn get_datasource() -> Result<DataSource> {
    let p = SqlitePool::builder().min_size(5).max_size(50).build(&env::var("DATABASE_URL")?).await?;
    Ok(DataSource {
        sled: sled::open("db/data.db").expect("open"),
        sqlite: p,
    })
}

pub async fn sled_id(d: &DataSource) -> Result<u64> {
    match d.sled.generate_id() {
        sled::Result::Ok(id) => Result::Ok(id),
        sled::Result::Err(e) => {
            eprintln!("{:?}", e);
            Result::Err(Err::new("failed to create id"))
        },
    }
}

async fn sled_save_data(d: &DataSource, key: impl AsRef<[u8]>, value: &str) -> Result<usize> {
    d.sled.insert(key, value)?;
    let r = d.sled.flush_async().await;
    match r {
        Ok(u) => Result::Ok(u),
        Err(e) => {
            eprintln!("{:?}", e);
            Result::Err(Err::new("save data failed"))
        },
    }
}

pub async fn sled_save(d: &DataSource, key: u64, value: &str) -> Result<usize> {
    sled_save_data(d, key.to_le_bytes(), value).await
}

pub async fn sled_get<T>(d: &DataSource, key: u64) -> Result<Option<T>> where T:serde::de::DeserializeOwned {
    if let Some(d) = d.sled.get(key.to_le_bytes())? {
        let b: T = serde_json::from_slice(d.as_ref())?;
        return Ok(Some(b))
    }
    Ok(None)
}

async fn sqlite_get_data(d: &DataSource) -> Result<Vec<i64>> {
    // let rows: Vec<Id> = sqlx::query_as!(Id, "SELECT id FROM blog ORDER BY id").fetch_all(&d.sqlite).await?;
    let mut conn = d.sqlite.acquire().await?;
    let rows = sqlx::query_as::<_, Id>("SELECT id FROM blog ORDER BY id").fetch_all(&mut conn).await?;
    let mut d: Vec<i64> = Vec::with_capacity(rows.len());
    for row in rows {
        println!(
            "{}",
            row.id,
        );
        d.push(row.id);
    }
    Result::Ok(d)
}

async fn sqlite_save_data() -> Result<()> {
    use sqlx::Connect;
    let mut conn = sqlx::SqliteConnection::connect("sqlite://./db/test.db").await?;
    let _effected_row = sqlx::query("SELECT * FROM tbl").execute(&mut conn).await?;
    Result::Ok(())
}