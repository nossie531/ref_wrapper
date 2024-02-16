pub struct VecEdit<'a> {
    base: &'a mut Vec<i32>,
}

impl<'a> VecEdit<'a> {
    pub fn new(base: &'a mut Vec<i32>) -> Self {
        Self { base }
    }

    pub fn base(&'a self) -> &'a Vec<i32> {
        self.base
    }

    pub fn add(&mut self, value: i32) {
        self.base.iter_mut().for_each(|x| *x += value);
    }
}
