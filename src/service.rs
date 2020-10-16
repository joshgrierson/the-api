use std::error;
use serde::Serialize;
use uuid::Uuid;

#[macro_export]
macro_rules! ser_data {
    ($data:expr) => {
        serde_json::to_string(&$data).unwrap()
    };
}

#[derive(Serialize, Debug)]
pub struct ListObject {
    name: String,
    id: String,
    item_count: u32
}

pub fn return_lists() -> Result<Vec<ListObject>, Box<dyn error::Error>> {
    let uuid: String = Uuid::new_v4().to_string();
    let lists_arr = vec![
        ListObject {
            name: String::from("Test List"),
            id: uuid,
            item_count: 4
        }
    ];

    Ok(lists_arr)
}
