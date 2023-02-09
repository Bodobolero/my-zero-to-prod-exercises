use crate::authentication::UserId;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn submit_newsletter_form(
    flash_messages: IncomingFlashMessages,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = user_id.into_inner();
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit a new newsletter</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Title:<br>
            <input
                type="text"
                placeholder="Enter the issue title"
                name="title"
            >
        </label>
        <br>
        <label>Plain text content:<br>
            <textarea
                placeholder="Enter the content in plain text"
                name="text"
                rows="20"
                cols="50"
            ></textarea>
        </label>
        <br>
        <label>HTML content:<br>
            <textarea
                placeholder="Enter the content in HTML format"
                name="html"
                rows="20"
                cols="50"
            ></textarea>
        </label>
        <br>
        <button type="submit">Publish</button>
    </form>
    <br>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
