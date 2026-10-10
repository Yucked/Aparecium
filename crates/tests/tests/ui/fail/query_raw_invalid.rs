fn main() {
    // `SELEC` is not a SurrealQL statement; the error must point at this literal.
    let _query = aparecium::query_raw!("SELEC name FROM user");
}
