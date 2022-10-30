use std::sync::Arc;
use std::collections::BTreeMap;

use surrealdb::sql::{Object, Value, Array, thing};
use surrealdb::{Datastore, Session};

use crate::schemas::Creatable;


use crate::prelude::*;
pub struct SurrealDB {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

impl SurrealDB {
    pub async fn init() -> Self {
        let ds = Arc::new(Datastore::new("file://surreal.db").await.expect("Error connecting to database"));
        let ses = Session::for_kv().with_ns("app").with_db("app");

        SurrealDB { ds, ses}
    }
}

impl SurrealDB {
    pub async fn create<T: Creatable>(&self, tb: &str, data: T) -> Result<Object> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

        let data: Object = W(data.into()).try_into()?;

		let vars: BTreeMap<String, Value> = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

		let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
		
        let first_val = ress.into_iter().next().map(|r| r.result).expect("id not returned")?;
        
        W(first_val.first()).try_into()
    }

    pub async fn get(&self, tb: &str, tid: &str) -> Result<Object> {
        let sql = "SELECT * FROM $th";
            
        let tid = format!("{}:{}", tb, tid);

        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];
    
        let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;
    
        let first_res = ress.into_iter().next().expect("Did not get a response");
    
        W(first_res.result?.first()).try_into()
           
    }

    pub async fn get_all(&self, tb: &str) -> Result<Vec<Object>> {
        let sql = String::from("SELECT * FROM type::table($tb)");

		let vars: BTreeMap<String, Value> = BTreeMap::from([("tb".into(), tb.into())]);

        let res = self.ds.execute(&sql, &self.ses, Some(vars), true).await?;
        
        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }
}