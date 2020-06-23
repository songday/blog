use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::result::{Error, Result};
use crate::model::{Blog, User};
use crate::db::DataSource;

type OnlineUsers = HashMap<String, User>;

lazy_static! {
    static ref ONLINE_USER: Arc<RwLock<OnlineUsers>> = Arc::new(RwLock::new(HashMap::new()));
}

pub fn check_auth(token: &str) -> Result<User> {
    if token.len() != 32 {
        return Err(Error::NotAuthed);
    }
    let r = ONLINE_USER.read();
    if let Some(u) = r.get(token) {
        return Ok((*u).clone());
    }
    Err(Error::NotAuthed)
}

pub async fn user_login(datasource: &DataSource, username: &str, password: &str) -> Result<User> {
    if username.len() < 3 || password.len() < 5 {
        return Err(Error::LoginFailed);
    }
    datasource.user_login(username, password).await
}

pub async fn blog_list(datasource: &DataSource, mut page_num: i32) -> Result<Vec<Blog>> {
    if page_num < 1 {
        page_num = 1;
    }
    datasource.blog_list(page_num).await
}

pub async fn blog_save(datasource: &DataSource, mut blog: Blog) -> Result<Blog> {
    if blog.title.len() < 3 || blog.title.len() > 60 {
        return Err(Error::SaveBlogFailed);
    }
    if blog.content.len() < 5 || blog.content.len() > 65535 {
        return Err(Error::SaveBlogFailed);
    }
    let num = datasource.blog_save(&mut blog).await?;
    dbg!("num = {}", num);
    if num == 0 {
        return Err(Error::SaveBlogFailed);
    }
    Ok(blog)
}