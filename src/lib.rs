use sqlx::query::query;
use sqlx::{Pool, MySqlPool, Connection, query_as};

// pub fn bulk_insert(insert_sql: impl Into<String>, value_sql: impl Into<String>, values: Vec<String>, pool: Pool<impl Connection>) {
//     let insert_sql = insert_sql.into();
//     let value_sql = value_sql.into();
//
//     let mut sql = insert_sql;
//     for (i, value) in values.iter().enumerate() {
//         sql.push_str(value_sql.clone().as_str());
//     }
//
//     let q = query(sql.as_str());
//     for (i, value) in values.iter().enumerate() {
//
//     }
// }

#[macro_export]
macro_rules! bulk_insert {
    ($t:ty) => {
        println!("ty: {}", stringify!($t));
    }
}

struct Todo {
    id: u64,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Todo {
        id: u64,
        name: String,
    }

    #[test]
    fn it_works() {
        bulk_insert!(Todo);
    }
}
