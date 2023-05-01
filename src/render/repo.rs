use std::{
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Hash)]
pub struct Id<T: ?Sized> {
    pub id: u32,
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Copy for Id<T> {}

impl<T: ?Sized> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}
impl<T: ?Sized> Eq for Id<T> {}

impl<T: ?Sized> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T: ?Sized> Default for Id<T> {
    fn default() -> Self {
        Id::from(0)
    }
}

impl<T: ?Sized> Id<T> {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        assert_ne!(id, 0, "Id overflow detected");
        Self::from(id)
    }
}

impl<T: ?Sized> From<u32> for Id<T> {
    fn from(id: u32) -> Self {
        Self {
            id,
            phantom: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct VecRepo<T>(Vec<T>);
impl<T: Default> Default for VecRepo<T> {
    fn default() -> Self {
        Self(vec![T::default()])
    }
}
impl<T> From<Vec<T>> for VecRepo<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
impl<T> Into<Vec<T>> for VecRepo<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
impl<T> VecRepo<T> {
    pub fn insert(&mut self, value: impl Into<T>) -> Id<T> {
        let id = self.0.len();
        self.0.push(value.into());
        Id::from(id as u32)
    }

    pub fn get_default(&self) -> &T {
        &self.0[0]
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.0.get(id.id as usize).unwrap_or(self.get_default())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id<T>, &T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(id, val)| (Id::from(id as u32), val))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id<T>, &mut T)> {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(id, val)| (Id::from(id as u32), val))
    }
}
