#[derive(Debug, Clone, PartialEq)]
pub struct Range(pub f32, pub f32);

impl Range {
    pub fn new(start: f32, end: f32) -> Self {
        if start < end {
            Range(start, end)
        } else {
            Range(end, start)
        }
    }

    pub fn intercept(&self, r: &Range) -> Option<Range> {
        let start = self.0.max(r.0);
        let end = self.1.min(r.1);

        if start <= end {
            Some(Range(start, end))
        } else {
            None
        }
    }

    pub fn clamp_start(&self, x: f32) -> Option<Range> {
        let c = self.0.max(x);
        if c <= self.1 {
            Option::Some(Range(c, self.1))
        } else {
            Option::None
        }
    }

    pub fn clamp_end(&self, y: f32) -> Option<Range> {
        let c = self.1.min(y);
        if c >= self.0 {
            Option::Some(Range(self.0, c))
        } else {
            Option::None
        }
    }

    pub fn center(&self) -> f32 {
        return (self.0 + self.1) / 2.;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intercept() {
        let r1 = Range(1., 3.);
        let r2 = Range(2., 4.);
        assert_eq!(r1.intercept(&r1), Some(Range(1., 3.)));
        assert_eq!(r1.intercept(&r2), Some(Range(2., 3.)));
        assert_eq!(r2.intercept(&r1), Some(Range(2., 3.)));
    }

    #[test]
    fn clanp_start() {
        let r1 = Range(1., 3.);
        assert_eq!(r1.clamp_start(2.), Some(Range(2., 3.)));
        assert_eq!(r1.clamp_start(3.), Some(Range(3., 3.)));
        assert_eq!(r1.clamp_start(4.), None);
        assert_eq!(r1.clamp_start(-2.), Some(Range(1., 3.)));
    }

    #[test]
    fn clanp_end() {
        let r1 = Range(1., 3.);
        assert_eq!(r1.clamp_end(-1.), None);
        assert_eq!(r1.clamp_end(1.), Some(Range(1., 1.)));
        assert_eq!(r1.clamp_end(2.), Some(Range(1., 2.)));

        assert_eq!(r1.clamp_end(20.), Some(Range(1., 3.)));
    }
}
