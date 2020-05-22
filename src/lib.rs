/// Generate a sql for bulk insert.
///
/// # Examples
///
/// ```
/// use bulk_sql_rs::bulk_insert_sql;
///
/// let res = bulk_insert_sql("todos", vec!["id", "name"], vec!["?", "?"], 3);
/// assert_eq!(res, "INSERT INTO todos (id, name) VALUES (?, ?), (?, ?), (?, ?)");
/// ```
pub fn bulk_insert_sql(
    table: impl Into<String>,
    columns: Vec<impl Into<String>>,
    values: Vec<impl Into<String>>,
    item_count: usize,
) -> String {
    let table = table.into();
    let columns = columns
        .into_iter()
        .map(|a| a.into())
        .collect::<Vec<String>>();
    let values = values
        .into_iter()
        .map(|s| s.into())
        .collect::<Vec<String>>();
    let column_sql: String = columns.join(", ");

    let mut sql = format!("INSERT INTO {} ({}) VALUES ", table, column_sql);
    for i in 0..item_count {
        let value_sql: String = values.clone().join(", ");
        sql.push_str(format!("({})", value_sql).as_str());
        if i < item_count - 1 {
            sql.push_str(", ");
        }
    }
    sql
}
