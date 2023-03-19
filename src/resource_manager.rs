use std::{sync::atomic::{AtomicU32, Ordering}, marker::PhantomData, collections::HashMap};

#[repr(transparent)]
pub struct Id<T: ?Sized> {
    id: u32, 
    phantom: PhantomData<T>
}

impl<T> Copy for Id<T> where T: ?Sized {}
impl<T> Clone for Id<T> where T: ?Sized {
    fn clone(&self) -> Self {
        Id { id: self.id, phantom: PhantomData }
    }
}

impl<T> Id<T> where T: ?Sized {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        assert_ne!(id, 0, "Id overflow detected");
        Self {id, phantom: PhantomData }
    }

    fn get(self) -> u32 { self.id }
}

pub struct ResourceManager<T: ?Sized> {
    resources: HashMap<u32, Box<T>>,
    default_value: Box<T>
}

impl<T> ResourceManager<T> where T: ?Sized {
    pub fn new(default_value: Box<T>) -> Self {
        Self { resources: HashMap::new(), default_value }
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.resources.get(&id.get()).unwrap_or(&self.default_value)
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.resources.get_mut(&id.get()).unwrap_or(&mut self.default_value)
    }

    pub fn insert(&mut self, id: Id<T>, value: Box<T>) {
        self.resources.insert(id.get(), value);
    }
}