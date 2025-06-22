use crate::frame::Frame;

use super::circle::Circle;
use super::rectangle::Rectangle;
use super::segment::Segment;
use vector::vector::Vector;



#[derive(Debug)]
pub enum Shape {
    Dot(Vector),
    Rectangle(Rectangle),
    Circle(Circle),
    Segment(Segment)
}


impl Shape {
    pub fn to_frame(&self) -> Frame {
        match self {
            Shape::Dot(p) => {
                Frame {
                    from: p.clone(),
                    to: p.clone()
                }
            }
            Shape::Rectangle(r) => {
                r.to_frame()
            }
            Shape::Circle(c) => {
                c.to_frame()
            }
            Shape::Segment(s) => {
                s.to_frame()
            }
        }
    }

    pub fn collision_with(&self, shape: &Shape) -> Option<Vector> {
        return match (self, shape) {
            (Shape::Dot(a), Shape::Dot(b)) => {
                if a.eq(b) {
                    Option::Some(a.clone())
                } else {
                    Option::None
                }
            },
            (Shape::Dot(a), Shape::Circle(c)) => {
                Shape::dot_in_circle(c,a)
            },
            (Shape::Circle(c), Shape::Dot(a) ) => {
                Shape::dot_in_circle(c,a)
            },
            // not implemented 
            (_, _) => Option::None,
        };
    }

    fn dot_in_circle(c: &Circle, d: &Vector) -> Option<Vector> {

        let l = c.location.clone_sub(d).size();

        if l < c.radius {
            Option::Some(d.clone())
        } else {
            Option::None
        }

    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_to_frame() {
        let a = Shape::Dot(Vector(1., 1.));
        assert_eq!(a.to_frame(), Frame{from: Vector(1., 1.), to: Vector(1., 1.)});
    }


    #[test]
    fn dots_collision() {
        let a = Shape::Dot(Vector(1., 1.));
        let b = Shape::Dot(Vector(1., 1.));
        let c = Shape::Dot(Vector(1., 1.));

        assert_eq!(a.collision_with(&b), Option::Some(Vector(1., 1.)));
        assert_eq!(a.collision_with(&c), Option::None);
    }


    #[test]
    fn dot_and_circle_collision() {
        let a = Shape::Circle(Circle { location: Vector(0.,0.), radius: 1.0});

        let b = Shape::Dot(Vector(0.5, 0.5));
        let c = Shape::Dot(Vector(1.5, 1.5));

        assert_eq!(a.collision_with(&b), Option::Some(Vector(0.5, 0.5)));
        assert_eq!(a.collision_with(&c), Option::None);
    }



}
