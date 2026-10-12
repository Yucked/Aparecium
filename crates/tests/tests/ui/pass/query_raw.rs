fn main() {
    let _query = umbriel::query_raw!("SELECT name, email FROM user WHERE age > 18");
}
