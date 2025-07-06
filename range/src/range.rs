pub struct Range(pub f32, pub f32);

impl Range {
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
             Option::Some(Range(self.0,c))
        } else {
            Option::None
        }
    }

    pub fn center(&self) -> f32 {
        return (self.0 + self.1) / 2.;
    }

}