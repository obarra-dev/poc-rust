use serde::{Deserialize, Serialize};
use serde_json::from_str;

const JSON_ONE_TODO: &str = r#"
{
    "userId": 1,
    "id": 4,
    "title": "omar rules",
    "complete": false
}"#;

const JSON_LIST_TODO: &str = r#"
[
{
    "userId": 1,
    "id": 4,
    "title": "omar rules",
    "complete": false
},
{
    "userId": 1,
    "id": 10,
    "title": "omar rules10",
    "complete": true
}
]"#;

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: usize,
    id: usize,
    title: String,
    complete: bool,
}

fn main() {
    let res = from_str::<Todo>(JSON_ONE_TODO);
    // TODO why this does not panic when user_id is not userId?
    print!("{:?}", res);

    let res = from_str::<Vec<Todo>>(JSON_LIST_TODO);
    print!("{:?}", res);
    println!("Hello, world!");
}
