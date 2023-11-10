use std::rc::Rc;
use std::slice::Chunks;
use std::sync::Arc;
use string_builder::Builder;

pub trait GrowingWindow<T> {
    fn growing_window(&self) -> Vec<Rc<&[T]>>;
}
impl <T> GrowingWindow<T> for Vec<T> {
    fn growing_window(&self) -> Vec<Rc<&[T]>> {
        let mut windows = vec![];
        for i in 1..self.len() {
            windows.push(Rc::new(&self[0..i]));
        }
        return windows
    }
}

pub trait Predicates<T> {
    fn first<F>(&self, predicate: F) -> (Option<&T>, Option<usize>) where F: Fn(&T) -> bool;
    fn last<F>(&self, predicate: F) -> (Option<&T>, Option<usize>) where F: Fn(&T) -> bool;
}
impl <T> Predicates<T> for Vec<T> {
    fn first<F>(&self, predicate: F) -> (Option<&T>, Option<usize>) where F: Fn(&T) -> bool {
        for i in 0..self.len() {
            if predicate(&self[i]) {
                return (Some(&self[i]), Some(i))
            }
        }
        (None, None)
    }

    fn last<F>(&self, predicate: F) -> (Option<&T>, Option<usize>) where F: Fn(&T) -> bool {
        for i in self.len()..0 {
            if predicate(&self[i]) {
                return (Some(&self[i]), Some(i))
            }
        }
        (None, None)
    }
}

pub trait CharsToString {
    fn to_string(&self) -> String;
}
impl CharsToString for [char] {
    fn to_string(&self) -> String {
        let mut builder = Builder::new(self.len());
        for c in self { builder.append(c.clone()); }
        builder.string().unwrap()
    }
}

pub trait ByteArray {
}

pub struct BitArray {
    vec: Vec<u8>
}

impl BitArray {
    pub fn with_capacity(size: usize) -> Self {
        let mut vec = Vec::with_capacity((size as f32 / 8.0).ceil() as usize);
        for i in 0..(size as f32 / 8.0).ceil() as usize { vec.push(0) }
        BitArray {
            vec
        }
    }

    pub fn set(&mut self, index: usize, values: &[u8]) {
        for i in 0..values.len() {
            self.vec[index + i] = values[i];
        }
    }

    pub fn chunked(&self, size: usize) -> Chunks<'_, u8> {
        self.vec.chunks(size)
    }
}