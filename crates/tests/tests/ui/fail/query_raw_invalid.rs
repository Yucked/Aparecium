fn main() {
    // `SELEC` is not a SurrealQL statement; the error must point at this literal.
    let _query = umbriel::query_raw!("SELEC name FROM user");
}
