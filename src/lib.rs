use serde::Serialize;
use sqlx::query::query;
use sqlx::{Executor, MySql};

pub async fn bulk_insert<T, E>(
    table: impl Into<String>,
    columns: Vec<String>,
    values: Vec<String>,
    params: Vec<T>,
    executor: E,
) -> Result<(), ()>
where
    T: Serialize,
    E: Executor<Database = MySql>,
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
                q = q.bind::<u64>(casted);
            } else if let Some(casted) = v.as_str() {
                q = q.bind::<&str>(casted);
            } else {
                panic!("invalid type");
            };
        }
    }

    q.execute(executor).await.map_err(|_e| ())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use sqlx::MySqlPool;
    use std::sync::Arc;

    #[derive(Serialize)]
    struct Todo {
        pub id: u64,
        pub name: String,
    }

    async fn setup() -> Arc<MySqlPool> {
        let pool = Arc::new(
            MySqlPool::new("mysql://root:secret@localhost:3307/foo")
                .await
                .expect("connection error"),
        );
        query("DROP TABLE todos")
            .execute(pool.clone().as_ref())
            .await
            .expect("failed drop todos");
        query(
            "
CREATE TABLE todos(
    id BIGINT UNSIGNED PRIMARY KEY,
    name VARCHAR(255)
)
",
        )
        .execute(pool.clone().as_ref())
        .await
        .expect("failed create todos");

        pool.clone()
    }

    async fn teardown(pool: &MySqlPool) {
        pool.close().await;
    }

    #[tokio::test]
    async fn test_bulk_insert() {
        let pool = setup().await;

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
        bulk_insert(
            "todos",
            vec!["id".to_string(), "name".to_string()],
            vec!["?".to_string(), "?".to_string()],
            params,
            pool.clone().as_ref(),
        )
        .await
        .unwrap();

        teardown(pool.clone().as_ref()).await;
    }
}
