use rocket::http::Status;
use rocket_contrib::databases::postgres;
use rocket_contrib::templates::Template;

#[database("content_db")]
pub struct ContentDb(postgres::Connection);

#[get("/blog/<path>")]
pub fn blog(content: ContentDb, path: String) -> Result<Template, Status> {
    let query_result = content.query(
        "SELECT * FROM blog_posts WHERE path = $1",
        &[&path],
    );
    
    let blog_rows = query_result.or(Err(Status::InternalServerError))?;
    let blog_row = blog_rows.iter().next().ok_or(Status::NotFound)?;

    println!("{:?}", blog_row.columns());
    
    let mut map = std::collections::HashMap::<&str, String>::new();
    map.insert("title", blog_row.get("title"));
    map.insert("body",  blog_row.get("body"));
    Ok(Template::render("blog", &map))
}