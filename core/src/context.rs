use std::sync::Mutex;

pub struct Context<T: Send + Sync> {
    value: std::sync::RwLock<T>,
    callbacks: Mutex<Vec<Box<dyn Fn() + Send + Sync>>>,
}

impl<T: Send + Sync> Context<T> {
    pub fn new(value: T) -> Self {
        Context {
            value: std::sync::RwLock::new(value),
            callbacks: vec![].into(),
        }
    }

    pub fn get(&self) -> std::sync::RwLockReadGuard<'_, T> {
        loop {
            if let Ok(value) = self.value.try_read() {
                return value;
            }
        }
    }

    pub fn set(&self, value: T) {
        loop {
            if let Ok(mut writer) = self.value.try_write() {
                *writer = value;
                break;
            }
        }
        for callback in self.callbacks.lock().unwrap().iter() {
            callback();
        }
    }

    pub fn register_on_changed(&self, callback: Box<dyn Fn() + Send + Sync>) {
        self.callbacks.lock().unwrap().push(callback);
    }
}
