use std::time::SystemTime;

use sqlx::prelude::*;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::model::{Blog, User};
use crate::result::{Error, Result};

type SqliteConnPool = sqlx::Pool<sqlx::sqlite::SqliteConnection>;

#[derive(Clone)]
pub struct DataSource {
    user: sled::Db,
    blog: sled::Db,
    sqlite: SqliteConnPool,
}

#[derive(sqlx::FromRow)]
struct Id {
    id: i64,
}

pub async fn get_datasource() -> Result<DataSource> {
    let p = SqlitePool::builder()
        .min_size(5)
        .max_size(50)
        .build("sqlite://./data/all.db")
        .await?;
    Ok(DataSource {
        user: sled::open("data/user").expect("open"),
        blog: sled::open("data/blog").expect("open"),
        sqlite: p,
    })
}

async fn sled_gen_id(db: &sled::Db) -> Result<u64> {
    db.generate_id().map_err(|e| {
        eprintln!("{}", e);
        Error::SledGenIdFailed
    })
}

#[inline]
async fn sled_save(db: &sled::Db, key: impl AsRef<[u8]>, value: &str) -> Result<usize> {
    db.insert(key, value)?;
    db.flush_async().await.map_err(|e| {
        eprintln!("{}", e);
        Error::SledDbError
    })
}

async fn sled_get<T>(db: &sled::Db, key: impl AsRef<[u8]>) -> Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    if let Some(data) = db.get(key)? {
        let b: T = serde_json::from_slice(data.as_ref())?;
        return Ok(Some(b));
    }
    Ok(None)
}

async fn sled_get_list<T>(db: &sled::Db, id_array: &Vec<i64>) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut list: Vec<T> = Vec::with_capacity(id_array.len());
    for id in id_array {
        if let Some(data) = db.get(id.to_le_bytes())? {
            let b: T = serde_json::from_slice(data.as_ref())?;
            list.push(b);
        }
    }
    Ok(list)
}

async fn sqlite_get_blog_id_array(pool: &SqliteConnPool, offset: i32) -> Result<Vec<i64>> {
    // let rows: Vec<Id> = sqlx::query_as!(Id, "SELECT id FROM blog ORDER BY id").fetch_all(&d.sqlite).await?;
    // let mut conn = d.sqlite.acquire().await?;
    let rows = sqlx::query_as::<_, Id>("SELECT id FROM blog LIMIT ?,? ORDER BY id")
        .bind(offset)
        .bind(crate::vars::BLOG_PAGE_SIZE)
        .fetch_all(pool)
        .await?;
    let mut d: Vec<i64> = Vec::with_capacity(rows.len());
    for row in rows {
        println!("{}", row.id,);
        d.push(row.id);
    }
    Result::Ok(d)
}

impl DataSource {
    pub async fn user_login(&self, username: &str, password: &str) -> Result<User> {
        let r: Option<User> = sled_get(&self.user, username).await?;
        if r.is_none() {
            return Err(Error::LoginFailed);
        }
        let mut u = r.unwrap();
        if crate::crypt::verify_password(password, &u.password) {
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, username.as_bytes());
            u.password.clear();
            u.access_token = uuid.to_hyphenated().to_string();
            Ok(u)
        } else {
            Err(Error::LoginFailed)
        }
    }

    pub async fn blog_list(&self, offset: i32) -> Result<Vec<Blog>> {
        let id_array = sqlite_get_blog_id_array(&self.sqlite, offset).await?;
        sled_get_list(&self.blog, &id_array).await
    }

    pub async fn blog_save(&self, blog: &mut Blog) -> Result<usize> {
        blog.id = sled_gen_id(&self.blog).await?;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        blog.created_at = now.unwrap().as_secs() as i64;

        // save to sqlite
        let _effected_row = sqlx::query("INSERT INTO blog(title,content,created_at)VALUES(?,?,?)")
            .bind(&blog.title)
            .bind(&blog.content)
            .bind(&blog.created_at)
            .execute(&self.sqlite)
            .await?;

        // save to sled
        // let b: Option<Blog> = sled_get(d, 100).await?;
        let serialized = serde_json::to_string(blog).unwrap();
        sled_save(&self.blog, blog.id.to_le_bytes(), &serialized).await
    }

    pub async fn blog_show(&self, id: u64) -> Result<Blog> {
        let r: Option<Blog> = sled_get(&self.blog, id.to_le_bytes()).await?;
        if r.is_none() {
            Err(Error::CannotFoundBlog)
        } else {
            Ok(r.unwrap())
        }
    }
}
