use actix_web::HttpResponse;
use crate::views::app::content_loader::read_file;

pub async fn items() -> HttpResponse {
    let mut html_data = read_file("c:/git/rust/web_app/src/templates/main.html");
    let javascript_data = read_file("c:/git/rust/web_app/src/javascript/main.js");
    let css_data: String = read_file("c:/git/rust/web_app/src/css/main.css");
    let base_css_data: String = read_file("c:/git/rust/web_app/src/css/base.css");
    html_data = html_data.replace("{{JAVASCRIPT}}", &javascript_data);
    html_data = html_data.replace("{{BASE_CSS}}", &base_css_data);
    html_data = html_data.replace("{{CSS}}", &css_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}