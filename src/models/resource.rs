use std::collections::HashMap;

use axum::Json;

pub type ResourceArray = HashMap<&'static str, String>;

pub type JsonResource = Json<ResourceArray>;

#[macro_export]
macro_rules! json {
    ($($key:expr => $value:expr),*) => {{
        use std::collections::HashMap;

        let mut map: ResourceArray = HashMap::new();
        $(map.insert($key, $value.to_string().to_owned());)*

        map
    }};
}

pub trait Resource<T>: Default {
    fn to_array(&self, data: T) -> ResourceArray;
    fn make(data: T) -> Json<ResourceArray> {
        Json(Self::default().to_array(data))
    }
    fn make_collection(data: Vec<T>) -> Json<Vec<ResourceArray>> {
        Json(
            data.into_iter()
                .map(|item| Self::default().to_array(item))
                .collect(),
        )
    }
}
