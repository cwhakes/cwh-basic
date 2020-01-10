use rocket::http::Status;
use rocket_contrib::databases::postgres;
use rocket_contrib::templates::Template;
use chrono::{DateTime, Utc};

#[database("content_db")]
pub struct ContentDb(postgres::Connection);

#[get("/blog/<path>")]
pub fn blog(content: ContentDb, path: String) -> Result<Template, Status> {
    let query_result = content.query(
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

    println!("{:?}", blog_row.columns());
    
    let mut map = std::collections::HashMap::<&str, String>::new();
    map.insert("title", blog_row.get("title"));
    map.insert("date", blog_row.get::<&str, DateTime<Utc>>("date").to_rfc2822());
    map.insert("body",  blog_row.get("body"));
    map.insert("prev_path", blog_row.get_opt("prev_path").unwrap_or(Ok("".to_string())).or(Ok("".to_string()))?);
    map.insert("prev_title", blog_row.get_opt("prev_title").unwrap_or(Ok("".to_string())).or(Ok("".to_string()))?);
    map.insert("next_path", blog_row.get_opt("next_path").unwrap_or(Ok("".to_string())).or(Ok("".to_string()))?);
    map.insert("next_title", blog_row.get_opt("next_title").unwrap_or(Ok("".to_string())).or(Ok("".to_string()))?);
    Ok(Template::render("blog", &map))
}