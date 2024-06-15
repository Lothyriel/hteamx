mod confirm;

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> IndexTemplate {
    IndexTemplate
}
