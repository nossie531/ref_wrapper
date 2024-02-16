pub struct VecStat<'a> {
    base: &'a Vec<i32>,
    more: i32,
}

impl<'a> VecStat<'a> {
    pub fn new(base: &'a Vec<i32>) -> Self {
        Self { base, more: 0 }
    }

    pub fn base(&'a self) -> &'a Vec<i32> {
        self.base
    }

    pub fn more(&mut self, value: i32) {
        self.more = value
    }

    pub fn summary(&self) -> i32 {
        self.base.iter().sum::<i32>() + self.more
    }
}
