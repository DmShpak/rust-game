use super::frame::Frame;
use std::f32::consts::PI;
use vector::vector::Vector;

#[derive(Debug)]
pub struct Circle {
    pub location: Vector,
    pub radius: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        return self.radius * self.radius * PI;
    }

    pub fn to_frame(&self) -> Frame {
        return Frame {
            from: Vector(self.location.0 - self.radius, self.location.1 - self.radius),
            to: Vector(self.location.0 + self.radius, self.location.1 + self.radius),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let v = Circle {
            location: Vector(0., 0.),
            radius: 1.,
        };
        let a = v.area();
        assert_eq!(a,PI);
    }

    #[test]
    fn to_frame() {
        let v = Circle {
            location: Vector(0., 0.),
            radius: 1.,
        };
        let a = v.to_frame();
        assert_eq!(a.from.0, -1.);
        assert_eq!(a.from.1, -1.);
        assert_eq!(a.to.0, 1.);
        assert_eq!(a.to.1, 1.);
    }

}
