// option for creating Json
use serde::Serialize;
// use serde_json::json; // uncomment when use json!
// use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct Account {
    id: uuid::Uuid,
    user_name: String,
    login_date: Option<String>,
}

impl Account {
    fn to_json(&self) -> String {
        //r# specifies the raw format no need to scape ""
        //manually
        // format!(
        //     r#"{{"id": "{:?}", "user_name":{:?} }}"#,
        //     self.id, self.user_name
        // )

        //json! serde
        // json!({
        //     "id": self.id.to_string(), // need to convert to string if not error
        //     "user_name": self.user_name
        // })
        // .to_string() //this returns a type Value

        // serde::Serialize (include derive feature on cargo for this)
        // for uuid include de "serde" on features
        serde_json::to_string(self).unwrap()
    }
}

fn main() {
    let acct = Account {
        id: uuid::Uuid::new_v4(),
        user_name: "Vale".to_string(),
        login_date: Some("2023-08-15".to_string()),
    };
    println!("{}", acct.to_json())
}
