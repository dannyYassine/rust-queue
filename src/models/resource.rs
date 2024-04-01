use std::collections::HashMap;

pub type ResourceArray = HashMap<&'static str, String>;

#[macro_export]
macro_rules! json {
    ($($key:expr => $value:expr),*) => {{
        use std::collections::HashMap;

        let mut map = HashMap::new();
        $(map.insert($key, $value);)*

        map
    }};
}

pub trait JsonResource<T>: Default {
    fn to_array(&self, data: T) -> ResourceArray;
    fn make(data: T) -> ResourceArray {
        Self::default().to_array(data)
    }
    fn make_collection(data: Vec<T>) -> Vec<ResourceArray> {
        data.into_iter().map(|item| Self::make(item)).collect()
    }
}
