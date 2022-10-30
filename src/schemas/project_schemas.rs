use std::{collections::BTreeMap};
use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Value, Object};

use super::Creatable;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl From<Object> for Owner {
    fn from(val: Object) -> Self {

        Owner {
            _id: Some(val.get("id").unwrap().clone().to_string()),
            name: val.get("name").unwrap().clone().to_string(),
            email: val.get("email").unwrap().clone().to_string(),
            phone: val.get("phone").unwrap().clone().to_string(),
        }
    }
}

#[derive(InputObject)]
pub struct CreateOwner {
    pub name: String,
    pub email: String,
    pub phone: String,
}
impl Creatable for CreateOwner {}

impl From<CreateOwner> for Value {
    fn from(val: CreateOwner) -> Self {

        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        

        value.insert("name".into(), val.name.into());
        
        value.insert("email".into(), val.email.into());

        value.insert("phone".into(), val.phone.into());

        Value::from(value)
    }
}
#[derive(InputObject)]
pub struct FetchOwner {
    pub _id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
}

impl Creatable for Project {}

impl From<Project> for Value {
    fn from(val: Project) -> Self {

        let mut value: BTreeMap<String, Value> = BTreeMap::new();
        
        if let Some(id) = val._id {
            value.insert("id".into(), id.into());
        }

        value.insert("name".into(), val.name.into());

        value.insert("status".into(), val.status.to_string().into());

        value.insert("owner_id".into(), val.owner_id.into());

        Value::from(value)
    }
}
impl From<Object> for Project {
    fn from(val: Object) -> Self {


        Project {
            _id: Some(val.get("id").unwrap().clone().to_string()),
            name: val.get("name").unwrap().clone().to_string(),
            description: val.get("description").unwrap().clone().to_string(),
            status: val.get("status").unwrap().clone().to_string(),
            owner_id: val.get("owner_id").unwrap().clone().to_string()
        }
    }
}

#[derive(InputObject)]
pub struct CreateProject {
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: String,
}
impl Creatable for CreateProject {}

impl From<CreateProject> for Value {
    fn from(val: CreateProject) -> Self {

        let mut value: BTreeMap<String, Value> = BTreeMap::new();

        value.insert("name".into(), val.name.into());
        
        value.insert("description".into(), val.description.into());

        value.insert("status".into(), val.status.to_string().into());

        value.insert("owner_id".into(), val.owner_id.into());

        Value::from(value)
    }
}
#[derive(InputObject)]
pub struct FetchProject {
    pub _id: String
}