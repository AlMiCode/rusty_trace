use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Id<T, IdT = u32> {
    id: IdT,
    phantom: std::marker::PhantomData<fn(T)>,
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Id<T> {
    pub fn new(n: impl Into<u32>) -> Self {
        Self {
            id: n.into(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VecRepo<T> {
    data: Vec<Option<T>>,
}

impl<T: Default> Default for VecRepo<T> {
    fn default() -> Self {
        Self {
            data: vec![Some(Default::default())],
        }
    }
}

impl<T> VecRepo<T> {
    pub fn new(default: T) -> Self {
        Self {
            data: vec![Some(default)],
        }
    }

    pub fn insert(&mut self, value: impl Into<T>) -> Id<T> {
        let id = self.data.len() as u32;
        self.data.push(Some(value.into()));
        Id::new(id)
    }

    pub fn get_default(&self) -> &T {
        unsafe { self.data.get_unchecked(0).as_ref().unwrap_unchecked() }
    }

    pub fn get(&self, key: Id<T>) -> &T {
        let x = self.data.get(key.id as usize);
        match x {
            None => self.get_default(),
            Some(x) => x.as_ref().unwrap_or(self.get_default()),
        }
    }

    pub fn get_mut(&mut self, key: Id<T>) -> Option<&mut T> {
        let x = self.data.get_mut(key.id as usize);
        match x {
            None => None,
            Some(x) => x.as_mut(),
        }
    }

    pub fn remove(&mut self, key: Id<T>) -> Option<T> {
        if self.data.len() == 1 {
            return None;
        }
        self.data.remove(key.id as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|x| x.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter_map(|x| x.as_mut())
    }
}
