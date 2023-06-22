use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    num: u16
}

#[derive(Template)]
#[template(path = "new.html")]
pub struct NewTemplate {}

#[derive(Template)]
#[template(path = "enter.html")]
pub struct EnterTemplate {
    name: String,
    part_max: Option<u8>,
    part_now: u8
}

#[derive(Template)]
#[template(path = "rand.html")]
pub struct RandTemplate {
    part: u8,
    name: String
}