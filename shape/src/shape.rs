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
    Segment(Segment),
}

impl Shape {
    pub fn to_frame(&self) -> Frame {
        match self {
            Shape::Dot(p) => Frame {
                from: p.clone(),
                to: p.clone(),
            },
            Shape::Rectangle(r) => r.to_frame(),
            Shape::Circle(c) => c.to_frame(),
            Shape::Segment(s) => s.to_frame(),
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
            }
            (Shape::Dot(a), Shape::Circle(c)) => Shape::dot_in_circle(c, a),
            (Shape::Circle(c), Shape::Dot(a)) => Shape::dot_in_circle(c, a),
            (Shape::Circle(c1), Shape::Circle(c2)) => Shape::circles_collision(c1, c2),
            (Shape::Rectangle(r1), Shape::Rectangle(r2)) => Shape::reactangles_collision(r1, r2),

            // not implemented
            (_, _) => Option::None,
        };
    }

    fn circles_collision(a: &Circle, b: &Circle) -> Option<Vector> {
        let ab = a.location.clone_sub(&b.location);
        let ab_size = ab.size();
        if ab_size == 0. {
            return Some(a.location.clone());
        } else if ab_size > a.radius + b.radius {
            return Option::None;
        } else {
            let ar = b
                .location
                .clone_sub(&a.location)
                .clone_scale(a.radius / ab_size)
                .clone_add(&a.location);
            let br = a
                .location
                .clone_sub(&b.location)
                .clone_scale(b.radius / ab_size)
                .clone_add(&b.location);

            return Option::Some(Vector((ar.0 + br.0) / 2., (ar.1 + br.1) / 2.));
        }
    }

    fn reactangles_collision(a: &Rectangle, b: &Rectangle) -> Option<Vector> {
        let a_frame = a.to_frame();
        let b_frame = b.to_frame();

        let overlap_min_x = a_frame.from.0.max(b_frame.from.0);
        let overlap_max_x = a_frame.to.0.min(b_frame.to.0);
        let overlap_min_y = a_frame.from.1.max(b_frame.from.1);
        let overlap_max_y = a_frame.to.1.min(b_frame.to.1);

        if overlap_min_x < overlap_max_x && overlap_min_y < overlap_max_y {
            let frame = Frame {
                from: Vector(overlap_min_x, overlap_min_y),
                to: Vector(overlap_max_x, overlap_max_y),
            };

            let rect = Rectangle::from_frame(&frame);

            return Some(rect.location);
        } else {
            None
        }
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
        assert_eq!(
            a.to_frame(),
            Frame {
                from: Vector(1., 1.),
                to: Vector(1., 1.)
            }
        );
    }

    #[test]
    fn dots_collision() {
        let a = Shape::Dot(Vector(1., 1.));
        let b = Shape::Dot(Vector(1., 1.));
        let c = Shape::Dot(Vector(2., 2.));

        assert_eq!(a.collision_with(&b), Option::Some(Vector(1., 1.)));
        assert_eq!(a.collision_with(&c), Option::None);
    }

    #[test]
    fn dot_and_circle_collision() {
        let a = Shape::Circle(Circle {
            location: Vector(0., 0.),
            radius: 1.0,
        });

        let b = Shape::Dot(Vector(0.5, 0.5));
        let c = Shape::Dot(Vector(1.5, 1.5));

        assert_eq!(a.collision_with(&b), Option::Some(Vector(0.5, 0.5)));
        assert_eq!(a.collision_with(&c), Option::None);
    }
}
