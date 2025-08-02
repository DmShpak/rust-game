use range::range::Range;
use shape::{circle::Circle, rectangle::Rectangle, shape::Shape};
use vector::vector::Vector;

#[derive(PartialEq,Debug)]
pub struct Collision {
    pub location: Vector,
    pub time_offset: f32,
} 

pub struct PhysObj {
    pub id: Option<usize>,
    pub shape: Shape,
    pub velosity: Vector,
}


impl  PhysObj {

    pub fn shift(&mut self, time: f32) {
        self.shape.shift(self.velosity.scale(time));
    }

    pub fn  predict_collision(a: &PhysObj, b: &PhysObj) -> Option<Collision> {
       match (&a.shape, &b.shape) {
            (Shape::Circle(c1), Shape::Circle(c2)) => {
                return PhysObj::circle_vs_circle(c1, &a.velosity, c2, &b.velosity);
            }
            (Shape::Rectangle(r1), Shape::Rectangle(r2)) => {
                return PhysObj::rect_vs_rect(r1, &a.velosity, r2, &b.velosity);
            }
            _ => Option::None
        }
    }

    fn rect_vs_rect (r1: &Rectangle, v1: &Vector, r2: &Rectangle,v2: &Vector)  -> Option<Collision> {
        let dp = r2.location.clone_sub(&r1.location);
        let dv = v2.clone_sub(v1);
        let d = r1.dementions.clone_add(&r2.dementions).scale(0.5).to_owned();

        // dist (dp + dv * t) = r
        // (dp + dv * t)^2 = r^2
        // || dp + dv * t || = r
        // t1 = (-r - dp) / dv
        // t2 = (r - dp) / dv

        fn asix_collision_time(dp: f32, dv: f32, r: f32) -> Option<Range> {
            if dv == 0. {
                Option::None
            } else {
                let t1 = (-r - dp) / dv;
                let t2  = (r - dp) / dv;
                Option::Some(Range::new(t1, t2))
            }
        }

        let cx = asix_collision_time(dp.0, dv.0, d.0);
        let cy: Option<Range> = asix_collision_time(dp.1, dv.1, d.1);

        if let (Some(range_x), Some(range_y)) = (cx, cy) {
                    if let Some(range_xy) = range_x.intercept(&range_y) {
                        let time_offset = range_xy.0;
                        let moved1 = r1.clone_shift(&v1.clone_scale(time_offset)).to_frame();
                        let moved2 = r2.clone_shift(&v2.clone_scale(time_offset)).to_frame();

                        if let Some(intr) = moved1.intercect(&moved2) {
                            let location = intr.center();
                            return Some(Collision { location, time_offset })
                        } else {
                            // should never happen
                        }
                    }

        } 

        Option::None
    }

    
    fn circle_vs_circle (c1: &Circle, v1: &Vector, c2: &Circle,v2: &Vector)  -> Option<Collision> {

        let dp = c2.location.clone_sub(&c1.location);
        let dv = v2.clone_sub(v1);
        let r = c1.radius + c2.radius;

        // dist (dp + dv * t) = r
        // (adp + adv * t) ^2 + (bdp + bdv * t) ^ 2 = r^2
        // adp^2 + adv^2 * t^2 + 2* adp* adv * t + bdp^2 + bdv ^2 * t ^2 + 2* bdp* bdv * t = r^2
        // (adv^2*t^2 + bdv^2*t^2) + (2 *adp*adv*t + 2*bdp*bdv*t) + (adp^2 +  bdp^2) = r^2
        // (adv^2 + bdv^2)*t^2 + (2 *adp*adv + 2*bdp*bdv)*t + (adp^2+bdp^2) - r^2 = 0
        //
        // A*t^2 + B*t + C = 0
        // A = (adv^2 + bdv^2) 
        // B = 2 (adp*adv + bdp*bdv)
        // C = (adp^2+bdp^2) - r^2

        let a = dv.0 * dv.0 + dv.1 * dv.1;
        let b = 2. * (dp.0 * dv.1  + dp.1 * dv.0);
        let c = dp.0 * dp.0 + dp.1 * dp.1 - r * r;

        let res = PhysObj::equasion(a, b, c);

        match res {
            Option::Some((t1,t2)) => {
                let time_offset = if t1 >= 0.0 {
                    t1
                } else if t2 >= 0.0 {
                    t2
                } else  {
                    return None;
                };

                let new_c1 = c1.location.clone_add(&v1.clone_scale(time_offset));
                let new_c2 = c2.location.clone_add(&v2.clone_scale(time_offset));
                let location = new_c1.clone_add(&new_c2).clone_scale(0.5);

                return Option::Some(
                    Collision{
                    location,
                    time_offset
            
                })
            }
            Option::None => Option::None
        }
    }


    // t1 < t2
    fn equasion (a:f32, b: f32, c: f32) -> Option<(f32, f32)> {
        let d = b * b - 4.0 * a * c;

        if d < 0.0 || a.abs() < 1e-6 {
            return Option::None;
        }

        let sqrt_disc = d.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        return Option::Some((t1,t2))


    }
} 



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circles_collision() {
        let c1 =  PhysObj {
                id: Option::None,
                velosity: Vector(1.,1.),
                shape: Shape::Circle(Circle {
                    location: Vector(0.,0.),
                    radius:1.
                })            
            
        };

        let c2 =  PhysObj {
                id: Option::None,
                velosity: Vector(-1.,-1.),
                shape: Shape::Circle(Circle {
                    location: Vector(10.,10.),
                    radius:1.
                })            
            
        };

        let c3 =  PhysObj {
                id: Option::None,
                velosity: Vector(-1.,-1.),
                shape: Shape::Circle(Circle {
                    location: Vector(10.,12.),
                    radius:1.
                })            
        };

        let x = PhysObj::predict_collision(&c1, &c2);
        let y = PhysObj::predict_collision(&c1, &c3);


        assert_eq!(x, Option::Some(Collision{location:Vector(5., 5.), time_offset: 4.2928934}));
        assert_eq!(y, Option::Some(Collision{location:Vector(5., 6.), time_offset: 5.}));
    }

    #[test]
    fn reactangles_collision() {
        let r1 =  PhysObj {
                id: Option::None,
                velosity: Vector(1.,1.),
                shape: Shape::Rectangle(Rectangle {
                    location: Vector(0.,0.),
                    dementions: Vector(1.,1.),
                })            
        };

        let r2 =  PhysObj {
                id: Option::None,
                velosity: Vector(-1.,-1.),
                shape: Shape::Rectangle(Rectangle {
                    location: Vector(10.,10.),
                    dementions: Vector(1.,1.),
                })            
            
        };

        let r3 =  PhysObj {
                id: Option::None,
                velosity: Vector(-1.,-1.),
                shape: Shape::Rectangle(Rectangle {
                    location: Vector(10.,12.),
                    dementions: Vector(1.,1.),
                })            
        };

        let x = PhysObj::predict_collision(&r1, &r2);
        let y = PhysObj::predict_collision(&r1, &r3);

        assert_eq!(x, Option::Some(Collision{location:Vector(5., 5.), time_offset: 4.5}));
        assert_eq!(y, Option::Some(Collision{location:Vector(5., 6.), time_offset: 5.5}));
    }
    
}
