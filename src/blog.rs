use rocket::http::Status;
use rocket::response::Redirect;
use rocket_contrib::databases::postgres;
use rocket_contrib::templates::Template;
use chrono::{DateTime, Utc};

#[database("content_db")]
pub struct ContentDb(postgres::Connection);

#[derive(Debug)]
struct BlogPost {
    title: String,
    date: DateTime<Utc>,
    body: String,
    prev_path:  Option<String>,
    prev_title: Option<String>,
    next_path:  Option<String>,
    next_title: Option<String>,
}

impl ContentDb {
    fn get_blog_post(&self, path: &str) -> Result<BlogPost, Status> {
        let query_result = self.query(
            "SELECT * FROM 
                (SELECT *,
                    LAG  (path,  1) OVER (ORDER BY ordinal) as prev_path,
                    LAG  (title, 1) OVER (ORDER BY ordinal) as prev_title,
                    LEAD (path,  1) OVER (ORDER BY ordinal) as next_path,
                    LEAD (title, 1) OVER (ORDER BY ordinal) as next_title
                FROM blog_posts)V
            WHERE path = $1",
            &[&path],
        );

        let blog_rows = query_result.or(Err(Status::InternalServerError))?;
        let blog_row = blog_rows.iter().next().ok_or(Status::NotFound)?;

        Ok(BlogPost {
            title: blog_row.get_opt("title").map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
            date:  blog_row.get_opt("date") .map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
            body:  blog_row.get_opt("body") .map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
            prev_path:  blog_row.get_opt("prev_path").map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
            prev_title: blog_row.get_opt("prev_title").map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
            next_path:  blog_row.get_opt("next_path").map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
            next_title: blog_row.get_opt("next_title").map(Result::ok).flatten().ok_or(Status::InternalServerError)?,
        })
    }
}

#[get("/blog")]
pub fn latest_blog(content: ContentDb) -> Result<Redirect, Status> {
    let query_result = content.query(
        "SELECT path FROM blog_posts
        ORDER BY ordinal DESC
        FETCH FIRST ROW ONLY",
        &[],
    );
    let path_rows = query_result.or(Err(Status::InternalServerError))?;
    let path_row = path_rows.iter().next().ok_or(Status::InternalServerError)?;
    let path: String = path_row.get_opt("path").map(Result::ok).flatten().ok_or(Status::InternalServerError)?;

    Ok(Redirect::to(format!("/blog/{}", path)))
}


#[get("/blog/<path>")]
pub fn blog(content: ContentDb, path: String) -> Result<Template, Status> {
    let blog_post = content.get_blog_post(&path)?;
    
    let mut map = std::collections::HashMap::<&str, String>::new();
    map.insert("title", blog_post.title);
    map.insert("date", blog_post.date.to_rfc2822());
    map.insert("body",  blog_post.body);
    map.insert("prev_path",  blog_post.prev_path.unwrap_or("".to_string()));
    map.insert("prev_title", blog_post.prev_title.unwrap_or("".to_string()));
    map.insert("next_path",  blog_post.next_path.unwrap_or("".to_string()));
    map.insert("next_title", blog_post.next_title.unwrap_or("".to_string()));
    Ok(Template::render("blog", &map))
}