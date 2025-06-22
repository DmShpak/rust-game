use crate::polar::Polar;

#[derive(PartialEq, Debug)]
pub enum Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn size(&self) -> f32 {
        return (self.0 * self.0 + self.1 * self.1).sqrt();
    }

    pub fn rotation(&self) -> f32 {
        return self.1.atan2(self.0);
    }

    pub fn add(&mut self, a: &Vector) -> &mut Self {
        self.0 += a.0;
        self.1 += a.1;
        return self;
    }

    pub fn sub(&mut self, a: &Vector) -> &mut Self {
        self.0 -= a.0;
        self.1 -= a.1;
        return self;
    }

    pub fn clone_sub(&self, a: &Vector) -> Vector {
        let mut result = self.clone();
        result.sub(a);
        return result;
    }

    pub fn clone_add(&self, a: &Vector) -> Vector {
        let mut result = self.clone();
        result.add(a);
        return result;
    }

    pub fn clone_scale(&self, s: f32) -> Vector {
        let mut result = self.clone();
        result.scale(s);
        return result;
    }


    pub fn invert(&mut self) -> &mut Self {
        self.0 *= -1.0;
        self.1 *= -1.0;
        return self;
    }

    pub fn as_polar(&mut self) -> Polar {
        return Polar {
            length: self.size(),
            rotation: self.rotation(),
        };
    }

    pub fn normalize(&mut self) -> &mut Self {
        if self.size() > 0. {
            self.scale(1.0 / self.size());
        }
        return self;
    }

    pub fn scale(&mut self, s: f32) -> &mut Self {
        self.0 *= s;
        self.1 *= s;
        return self;
    }

    // pub fn orientation(p: &Vector, q: &Vector, r: &Vector) -> Orientation {
    //     let pq = p.clone_sub(q);
    //     let qr = p.clone_sub(r);
    //     return pq.cross(&qr);
    // }

    /**
       The dot product is useful for many things:
       Angle between vectors:
       If A • B > 0: vectors point in roughly the same direction
       If A • B < 0: vectors point in opposite directions
       If A • B = 0: vectors are perpendicular
    */
    pub fn dot(&self, d: &Vector) -> f32 {
        self.0 * d.0 + self.1 * d.1
    }
    /**
     * Given vectors a and b:
        cross(a, b) > 0: b is to the left of a (counter-clockwise turn).
        cross(a, b) < 0: b is to the right of a (clockwise turn).
        cross(a, b) = 0: a and b are collinear (parallel or same line).
     */
    pub fn cross(&self, d: &Vector)  -> Orientation {
        let val = self.0 * d.1 - self.1 * d.0;

        if val > 0.0 {
            Orientation::Clockwise
        } else if val < 0.0 {
            Orientation::Counterclockwise
        } else {
            Orientation::Collinear
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let v = Vector::default();
        assert_eq!(v.0, 0.);
        assert_eq!(v.1, 0.);
    }

    #[test]
    fn size() {
        let v1 = Vector::default();
        assert_eq!(v1.size(), 0.);
        let v2 = Vector(3., 4.);
        assert_eq!(v2.size(), 5.);
    }

    #[test]
    fn add() {
        let mut v1 = Vector(1., 2.0);
        let v2 = Vector(3., 4.0);
        let v3 = v1.add(&v2);
        assert_eq!(v3.0, 4.);
        assert_eq!(v3.1, 6.);
    }

    #[test]
    fn scale() {
        let mut v1 = Vector(1., 2.0);

        let v3 = v1.scale(2.);
        assert_eq!(v3.0, 2.);
        assert_eq!(v3.1, 4.);
    }

    #[test]
    fn normalize() {
        let mut v1 = Vector(3., 0.);
        let mut v2 = Vector(0., 0.);

        let v3 = v1.normalize();
        let v4 = v2.normalize();

        assert_eq!(v3.0, 1.);
        assert_eq!(v3.1, 0.);

        assert_eq!(v4.0, 0.);
        assert_eq!(v4.1, 0.);
    }

    #[test]
    fn dot() {
        let a = Vector(1., 2.);
        let b = Vector(3., 4.);

        let c = a.dot(&b);

        assert_eq!(c, 11.);
    }

    #[test]
    fn cross() {
        let a = Vector(1., 1.);
        let b1 = Vector(1., 4.);
        let b2 = Vector(1., 0.5);
        let b3 = Vector(10., 10.);

        assert_eq!(a.cross(&b1), Orientation::Clockwise);
        assert_eq!(a.cross(&b2), Orientation::Counterclockwise);
        assert_eq!(a.cross(&b3), Orientation::Collinear);
    }
}
