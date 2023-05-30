/// `quote_ident` quotes the given value as an identifier (table, schema)
/// safely for use in a dynamically generated sql query.
/// Implementation matches that of `quote_identifier` in ruleutils.c of the
/// `PostgreSQL` code, with `quote_all_identifiers` = true.
pub fn quote_ident(value: &str) -> String {
    let mut result = String::with_capacity(value.len() + 4);
    result.push('"');
    for c in value.chars() {
        if c == '"' {
            result.push(c);
        }
        result.push(c);
    }
    result.push('"');
    result
}
