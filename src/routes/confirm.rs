#[derive(askama::Template)]
#[template(path = "confirm.html")]
pub struct ConfirmTemplate {
    pub name: Option<String>,
    pub escorts: Vec<String>,
}
