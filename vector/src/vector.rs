use crate::polar::Polar;

#[derive(Default, Clone, Debug)]

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
}
