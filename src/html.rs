use crate::prelude::*; 

const WEBSITES: &str = "./websites";

pub async fn website(
    path: web::Path<WebsitePathParams>
) -> Result<impl Responder, std::io::Error> {
    let page = path.page.clone().unwrap_or_else(|| "index".to_string());
    let website = path.website_name.clone().unwrap_or_else(|| "main_page".to_string());
    let file_path = PathBuf::from(format!("{WEBSITES}/{}/{}.html", website, page));
    
    match std::fs::read_to_string(&file_path) {
        Ok(html) => Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)),
        Err(_) => Ok(HttpResponse::NotFound().body("404 - Page not found")),
    }
}