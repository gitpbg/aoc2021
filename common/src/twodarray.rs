pub struct TwoDArray<T> {
    w: usize,
    h: usize,
    data: Vec<T>,
}
impl<T> TwoDArray<T>
where
    T: Default + Clone + Copy,
{
    pub fn new(w: usize, h: usize) -> TwoDArray<T> {
        let mut rv = TwoDArray {
            w: w,
            h: h,
            data: Vec::new(),
        };

        rv.data.resize(w * h, Default::default());
        rv
    }

    pub fn index(&self, r: i32, c: i32) -> Option<usize> {
        if r >= 0 && c >= 0 && (r as usize) < self.h && (c as usize) < self.w {
            Some(r as usize * self.w + c as usize)
        } else {
            None
        }
    }

    pub fn get(&self, r: i32, c: i32) -> Option<T> {
        if let Some(idx) = self.index(r, c) {
            Some(self.data[idx])
        } else {
            None
        }
    }

    pub fn set(&mut self, r: i32, c: i32, v: T) {
        if let Some(idx) = self.index(r, c) {
            self.data[idx] = v;
        }
    }
}
