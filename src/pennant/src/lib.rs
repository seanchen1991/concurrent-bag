#![allow(dead_code)]
#![feature(box_into_raw_non_null)]

use std::ptr::NonNull;

/// A Pennant is comprised of a unary root whose child
/// is a complete binary tree. Each Pennant nodes stores
/// a value. Bags store multiple Pennants in order to 
/// store any arbitrary number of elements in the Bag.
pub struct Pennant<T> {
    pub k: u32,
    element: T,
    count: usize,
    pub left: Option<NonNull<Pennant<T>>>,
    pub middle: Option<NonNull<Pennant<T>>>,
    pub right: Option<NonNull<Pennant<T>>>,
}

impl<T> Pennant<T> {
    pub fn new(element: T) -> Self {
        Pennant { 
            element,
            k: 0,
            count: 1,
            left: None,
            right: None,
            middle: None, 
        }
    }

    pub fn fetch_element(&self) -> &T {
        &self.element
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn degree(&self) -> u32 {
        self.k
    }

    /// Combines two Pennants into a single Pennant whose
    /// total number of elements is the sum of the number 
    /// of elements of the combined Pennants.
    /// Note that the Bag will maintain the invariant 
    /// that only Pennants of equal k will be combined.
    /// Thus combining two Pennants should result in a new
    /// Pennant whose degree is k + 1 for the old degree
    /// of the combined Pennants.
    pub fn combine(&mut self, mut pennant: Box<Pennant<T>>) {
        assert!(self.degree() == pennant.degree());

        match self.middle {
            None => {
                self.middle = Some(Box::into_raw_non_null(pennant));
                self.count += 1;
                self.k = 1;
            },
            Some(middle) => {
                pennant.left = Some(middle);
                pennant.right = pennant.middle;
                pennant.middle = None;
                self.count += pennant.len();
                self.k = f32::log2(self.count as f32) as u32;
                self.middle = Some(Box::into_raw_non_null(pennant));
            }
        }
    }

    /// Performs the inverse of the `combine` method. Splits
    /// a Pennant into two Pennants of equal size, updating
    /// each new Pennant's k value accordingly.
    /// Mutates the original Pennant and returns the 
    /// split-off Pennant.
    pub fn split(&mut self) -> Option<Box<Pennant<T>>> {
        match self.middle {
            None => None,
            Some(middle) => {
                let mut new_pennant;
                unsafe {
                    new_pennant = Box::from_raw(middle.as_ptr());
                }

                self.middle = new_pennant.left;
                new_pennant.middle = new_pennant.right;
                new_pennant.left = None;
                new_pennant.right = None;

                self.count /= 2;
                self.k = f32::log2(self.count as f32) as u32;

                new_pennant.count = self.len();
                new_pennant.k = self.degree();

                Some(new_pennant)
            }
        }
    }
}
