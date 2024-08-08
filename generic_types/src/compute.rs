#![allow(dead_code)]

use std::fmt;

pub fn list_sum(list: Vec<i32>) -> i32 {
    list.iter().sum()
}

pub fn list_product(list: Vec<i32>) -> i32 {
    list.iter().product()
}

#[derive(fmt::Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: fmt::Display> Point<T> {
    fn as_str(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

pub fn use_point(p1: Point<f32>, p2: Point<f32>) -> String {
    let new_point = Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y
    };
    new_point.as_str()
}