// https://leetcode.com/problems/min-stack/submissions/1196310650

use std::cmp;

pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push((val, cmp::min(val, self.get_min())))
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().copied().map(|(_, min)| min).unwrap_or(i32::MAX)
    }
}
