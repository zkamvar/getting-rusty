#[derive(Debug)]
pub struct Vegetable {
    pub name: String, // only this part of the struct is visible
    id: i32,
}
impl Vegetable {
    pub fn new(name: &str) -> Vegetable {
        Vegetable {
            name: String::from(name),
            id: 1,
        }
    }
}
