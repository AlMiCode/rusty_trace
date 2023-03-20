use std::collections::hash_map::Iter;
use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

#[repr(transparent)]
pub struct Id<T: ?Sized> {
    id: u32,
    phantom: PhantomData<T>,
}

impl<T> Copy for Id<T> where T: ?Sized {}
impl<T> Clone for Id<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Id {
            id: self.id,
            phantom: PhantomData,
        }
    }
}
impl<T> Hash for Id<T>
where
    T: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
impl<T> PartialEq for Id<T>
where
    T: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}
impl<T> Eq for Id<T> where T: ?Sized {}

impl<T> Display for Id<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T> Id<T>
where
    T: ?Sized,
{
    pub fn new() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        assert_ne!(id, 0, "Id overflow detected");
        Self::from(id)
    }
    fn from(id: u32) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

pub struct ResourceManager<T: ?Sized> {
    resources: HashMap<Id<T>, Box<T>>,
    default_value: Box<T>,
}

impl<T> ResourceManager<T>
where
    T: ?Sized,
{
    pub fn new(default_value: Box<T>) -> Self {
        Self {
            resources: HashMap::new(),
            default_value,
        }
    }

    pub fn get_default(&self) -> &T {
        &self.default_value
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.resources.get(&id).unwrap_or(&self.default_value)
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.resources
            .get_mut(&id)
            .unwrap_or(&mut self.default_value)
    }

    pub fn insert(&mut self, id: Id<T>, value: Box<T>) {
        self.resources.insert(id, value);
    }

    pub fn remove(&mut self, id: Id<T>) -> Option<Box<T>> {
        self.resources.remove(&id)
    }

    pub fn iter(&self) -> Iter<'_, Id<T>, Box<T>> {
        self.resources.iter()
    }
}
