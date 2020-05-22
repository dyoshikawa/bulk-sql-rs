use serde::Serialize;
use sqlx::query::query;

pub fn bulk_insert_query<T>(
    table: impl Into<String>,
    columns: Vec<String>,
    values: Vec<String>,
    params: Vec<T>,
) where
    T: Serialize,
{
    let table = table.into();

    let column_sql: String = columns.join(", ");
    let mut sql = format!("INSERT INTO {} ({}) VALUES ", table, column_sql);
    for i in 0..params.len() {
        let value_sql: String = values.clone().join(", ");
        sql.push_str(format!("({})", value_sql).as_str());
        if i < params.len() - 1 {
            sql.push_str(", ");
        }
    }

    let mut q = query::<MySql>(sql.as_str());
    for p in params.iter() {
        let val = serde_json::to_value(p).unwrap();
        for col in columns.iter() {
            let v = val.get(col).unwrap();
            if let Some(casted) = v.as_u64() {
                q = q.bind(casted);
            } else if let Some(casted) = v.as_str() {
                q = q.bind(casted);
            } else {
                panic!("aaa");
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Todo {
        pub id: u64,
        pub name: String,
    }

    #[test]
    fn test_bulk_insert_query() {
        let params = vec![
            Todo {
                id: 1,
                name: "test".to_string(),
            },
            Todo {
                id: 2,
                name: "test".to_string(),
            },
        ];
        bulk_insert_query(
            "todos",
            vec!["id".to_string(), "name".to_string()],
            vec!["?".to_string(), "?".to_string()],
            params,
        );
    }
}
