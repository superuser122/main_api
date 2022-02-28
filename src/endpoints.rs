#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}