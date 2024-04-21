use std::any::Any;

pub struct Cache {}

impl Cache {
    pub fn get(&self, key: &str) -> Box<dyn Any> {
        return Box::new("x");
    }

    pub fn get_fn(&self, key: &str) -> Box<dyn Any> {
        return Box::new("x");
    }

    pub fn has(&self, key: &str) -> bool {
        return true;
    }

    pub fn add<T>(&self, key: &str, value: T) {}

    pub fn pull<T>(&self, key: &str, value: T) {}

    pub fn put<T>(&self, key: &str, value: T) {}

    pub fn forever<T>(&self, key: &str, value: T) {}

    pub fn forget<T>(&self, key: &str, value: T) {}

    pub fn flush<T>(&self, key: &str, value: T) {}

    pub fn lock<T>(&self, key: &str, value: T) {}
}

pub struct Lock {}

impl Lock {
    pub fn get(&self) -> bool {
        return true;
    }
    pub fn block(&self, time: i32) -> bool {
        return true;
    }
    pub fn release(&self) -> bool {
        return true;
    }
}
