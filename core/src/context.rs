use std::sync::Mutex;

pub trait Model {
    fn register_context_handler(&'static self, context_handler: &'static ContextHandler);
}

pub fn get_static_context_handler() -> &'static ContextHandler {
    Box::leak(Box::new(ContextHandler::new()))
}

pub struct ContextHandler {
    on_change_callbacks: std::sync::RwLock<Vec<Box<dyn Fn() + Send + Sync>>>,
}

impl ContextHandler {
    pub fn new() -> Self {
        Self {
            on_change_callbacks: vec![].into(),
        }
    }

    pub fn register_on_change(&'static self, callback: Box<dyn Fn() + Send + Sync>) {
        self.on_change_callbacks.write().unwrap().push(callback);
    }

    pub fn register_context<T: Send + Sync>(&'static self, context: &'static Context<T>) {
        context.register_on_change(Box::new(move || {
            for callback in self.on_change_callbacks.read().unwrap().iter() {
                callback();
            }
        }));
    }

    pub fn register_contexts<T: Send + Sync>(&'static self, contexts: Vec<&'static Context<T>>) {
        for context in contexts.iter() {
            self.register_context(context);
        }
    }
}

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

    pub fn register_on_change(&self, callback: Box<dyn Fn() + Send + Sync>) {
        self.callbacks.lock().unwrap().push(callback);
    }
}
