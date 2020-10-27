#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub location: String,
}
