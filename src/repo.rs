use indexmap::map::{Iter, IterMut};
use std::{
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    sync::{atomic::{AtomicU32, Ordering}, Arc}, ops::{Deref, DerefMut},
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


pub type Repo<T> = Repository<T, Box<T>>;
pub type ARepo<T> = Repository<T, Arc<T>>;

pub struct Repository<Type: ?Sized, ContainedType: Deref<Target = Type>> {
    resources: IndexMap<Id<Type>, ContainedType>,
    default_value: ContainedType,
}

impl<T, C> Repository<T, C>
where
    T: ?Sized,
    C: Deref<Target = T>
{
    pub fn new(default_value: C) -> Self {
        Self {
            resources: IndexMap::new(),
            default_value,
        }
    }

    pub fn get_default(&self) -> &T {
        &self.default_value
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.resources.get(&id).unwrap_or(&self.default_value)
    }

    pub fn insert(&mut self, id: Id<T>, value: C) {
        self.resources.insert(id, value);
    }

    pub fn iter(&self) -> Iter<'_, Id<T>, C> {
        self.resources.iter()
    }
}

impl<T, C> Repository<T, C>
where
    T: ?Sized,
    C: Deref<Target = T> + DerefMut<Target = T>
{
    pub fn get_defaul_mut(&mut self) -> &mut T {
        &mut self.default_value
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.resources
            .get_mut(&id)
            .unwrap_or(&mut self.default_value)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Id<T>, C> {
        self.resources.iter_mut()
    }
}

impl<T, C> Clone for Repository<T, C>
where
    T: ?Sized,
    C: Deref<Target = T> + Clone
{
    fn clone(&self) -> Self {
        Repository { resources: self.resources.clone(), default_value: self.default_value.clone() }
    }
}