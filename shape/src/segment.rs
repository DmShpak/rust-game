use vector::vector::Vector;

use crate::frame::Frame;

/**
 * Segment described as start point and vector
 */
#[derive(Debug,PartialEq)]
pub struct Segment {
    pub location: Vector,
    pub vector: Vector,
}

impl Segment {
    /**
     * Get vector end point
     */
    pub fn end(&self) -> Vector {
        return self.location.clone().add(&self.vector).to_owned();
    }

    pub fn to_frame(&self) -> Frame {
        let end = self.end();

        return Frame {
            from: Vector(
                f32::min(self.location.0, end.0),
                f32::min(self.location.1, end.1),
            ),
            to: Vector(
                f32::max(self.location.0, end.0),
                f32::max(self.location.1, end.1),
            ),
        };
    }

    pub fn scale(&mut self, s: f32) -> &mut Self {
        self.vector.scale(s);
        return self;
    }

    pub fn shift(&mut self, s: &Vector)-> &mut Self  {
        self.location.add(s);
        return self;
    }

    pub fn project_point(&self, p: &Vector) -> Vector {
        let a = self.location.clone();
        let b = self.end();

        let ab = b.clone_sub(&a);
        let ap = p.clone_sub(&a);

        let ab_dot_ab = ab.dot(&ab);

        if ab_dot_ab == 0.0 {
            return a.clone(); // line is a point
        }

        let t = ap.dot(&ab) / ab_dot_ab;
        return self.location.clone_add(&ab.clone_scale(t));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end() {
        let r = Segment {
            location: Vector(1., 2.),
            vector: Vector(3., 4.),
        };
        let a = r.end();
        assert_eq!(a, Vector(4., 6.));
    }

    #[test]
    fn to_frame() {
        let r = Segment {
            location: Vector(1., 2.),
            vector: Vector(3., -4.),
        };
        let a = r.to_frame();
        assert_eq!(
            a,
            Frame {
                from: Vector(1., -2.),
                to: Vector(4., 2.)
            }
        );
    }

    #[test]
    fn scale() {
        let mut r = Segment {
            location: Vector(1., 2.),
            vector: Vector(3., -4.),
        };
        r.scale(2.);
        assert_eq!(
            r,
            Segment {
                location: Vector(1., 2.),
                vector: Vector(6., -8.),
            }
        );
    }

    #[test]
    fn shift() {
        let mut r = Segment {
            location: Vector(1., 2.),
            vector: Vector(3., -4.),
        };
        r.shift(&Vector(1., 2.));

        assert_eq!(
            r,
            Segment {
                location: Vector(2., 4.),
                vector: Vector(3., -4.),
            }
        );
        
    }

    #[test]
    fn project_point() {
        let s = Segment {
            location: Vector(0., 0.),
            vector: Vector(0., 2.),
        };
        let p = s.project_point(&Vector(1., 1.));

        assert_eq!(
            p,
            Vector(0., 1.),
        );
        
    }

}
