use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl Interval {
    #[inline]
    pub fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn enclose(a: &Self, b: &Self) -> Self {
        let start = a.start.min(b.start);
        let end = a.end.max(b.end);
        Self { start, end }
    }

    #[inline]
    pub fn grow(&mut self, other: &Self) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.end - self.start
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    #[inline]
    pub fn contains(&self, value: f64) -> bool {
        self.start <= value && value <= self.end
    }

    #[inline]
    pub fn surrounds(&self, value: f64) -> bool {
        self.start < value && value < self.end
    }

    #[inline]
    pub fn expand(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.start -= padding;
        self.end += padding;
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}

impl From<(f64, f64)> for Interval {
    fn from(value: (f64, f64)) -> Self {
        Interval::new(value.0, value.1)
    }
}
