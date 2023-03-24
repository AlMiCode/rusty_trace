use indexmap::map::{Iter, IterMut};
use std::{
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

use indexmap::IndexMap;

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

impl<T> Default for Id<T>
where
    T: ?Sized,
{
    fn default() -> Self {
        Id::from(0)
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



pub struct Repo<T: ?Sized> {
    resources: IndexMap<Id<T>, Box<T>>,
    default_value: Box<T>,
}

impl<T> Repo<T>
where
    T: ?Sized,
{
    pub fn new(default_value: Box<T>) -> Self {
        Self {
            resources: IndexMap::new(),
            default_value,
        }
    }

    pub fn get_default(&self) -> &T {
        &self.default_value
    }

    pub fn get_defaul_mut(&mut self) -> &mut T {
        &mut self.default_value
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

    pub fn iter_mut(&mut self) -> IterMut<'_, Id<T>, Box<T>> {
        self.resources.iter_mut()
    }
}

impl<T> Clone for Repo<T>
where
    T: ?Sized,
    Box<T>: Clone
{
    fn clone(&self) -> Self {
        Self { resources: self.resources.clone(), default_value: self.default_value.clone() }
    }
}
