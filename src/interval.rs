#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

#[allow(dead_code, reason = "Intended to be used in other modules")]
impl Interval {
    #[inline(always)]
    pub fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    #[inline(always)]
    pub fn enclose(a: &Self, b: &Self) -> Self {
        let start = a.start.min(b.start);
        let end = a.end.max(b.end);
        Self { start, end }
    }

    #[inline(always)]
    pub fn grow(&mut self, other: &Self) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
    }

    #[inline(always)]
    pub fn size(&self) -> f64 {
        self.end - self.start
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    #[inline(always)]
    pub fn contains(&self, value: f64) -> bool {
        self.start <= value && value <= self.end
    }

    #[inline(always)]
    pub fn surrounds(&self, value: f64) -> bool {
        self.start < value && value < self.end
    }

    #[inline(always)]
    pub fn expand(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.start -= padding;
        self.end += padding;
    }
}
