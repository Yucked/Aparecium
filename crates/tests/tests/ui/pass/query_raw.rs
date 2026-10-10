fn main() {
    let _query = aparecium::query_raw!("SELECT name, email FROM user WHERE age > 18");
}
