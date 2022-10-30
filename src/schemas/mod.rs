use surrealdb::sql::Value;
pub trait Creatable: Into<Value> {}

pub mod project_schemas;
