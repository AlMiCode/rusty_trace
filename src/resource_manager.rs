use std::{sync::atomic::{AtomicU32, Ordering}, marker::PhantomData, collections::HashMap};

#[repr(transparent)]
pub struct Id<T> {
    id: u32, 
    phantom: PhantomData<T>
}

impl<T> Copy for Id<T> {}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id { id: self.id, phantom: PhantomData }
    }
}

impl<T> Id<T> {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        assert_ne!(id, 0, "Id overflow detected");
        Self {id, phantom: PhantomData }
    }

    fn get(self) -> u32 { self.id }
}

pub struct ResourceManager<T> {
    resources: HashMap<u32, T>,
    default_value: T
}

impl<T> ResourceManager<T> {
    pub fn new(default_value: T) -> Self {
        Self { resources: HashMap::new(), default_value }
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.resources.get(&id.get()).unwrap_or(&self.default_value)
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.resources.get_mut(&id.get()).unwrap_or(&mut self.default_value)
    }

    pub fn insert(&mut self, id: Id<T>, value: T) {
        self.resources.insert(id.get(), value);
    }
}