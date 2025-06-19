use vector::vector::Vector;
/**
 * Frame modeling the rectungle boundary of shape 
 */


#[derive(Debug, PartialEq)]
pub struct Frame {
    pub from: Vector,
    pub to: Vector,
}


impl Frame {

    pub fn center(&self) -> Vector {
        return Vector(
            (self.from.0 + self.to.0) / 2.0,
            (self.from.1 + self.to.1) / 2.0,
        );
    }

    pub fn dementions(&self) -> Vector {
        return Vector(self.to.0 - self.from.0, self.to.1 - self.from.1);
    }

    pub fn is_intercected(&self, other: &Frame) -> bool {
        if self.to.0 < other.from.0 {
            return false;
        } else if self.from.0 > other.to.0 {
            return false;
        } else if self.to.1 < other.from.1 {
            return false;
        } else if self.from.1 > other.to.1 {
            return false;
        } else {
            return true;
        }
    }
    
    pub fn intercect(&self, other: &Frame) -> Option<Frame> {
        if self.is_intercected(other) {
            return Option::Some(Frame {
                from: Vector(
                    f32::max(self.from.0, other.from.0),
                    f32::max(self.from.1, other.from.1),
                ),
                to: Vector(f32::min(self.to.0, other.to.0), f32::min(self.to.1, other.to.1)),
            });
        } else {
            return Option::None;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dementions() {
        let f = Frame{
                from: Vector(0.,0.),
                to: Vector(1.,1.),
        };
        let d = f.dementions();
        assert_eq!(d.0,1.);
        assert_eq!(d.1,1.);
    }

    
    #[test]
    fn is_intercected() {
        let f = Frame{
                from: Vector(0.,0.),
                to: Vector(2.,2.),
        };
        let i1 = f.is_intercected(&Frame{
                from: Vector(1.,1.),
                to: Vector(3.,3.),
        });
        let i2 = f.is_intercected(&Frame{
                from: Vector(4.,4.),
                to: Vector(5.,5.),
        });

        assert_eq!(i1,true);
        assert_eq!(i2,false);
    }
    #[test]
    fn intercect () {
        let f = Frame{
                from: Vector(0.,0.),
                to: Vector(2.,2.),
        };
        let i1 = f.intercect(&Frame{
                from: Vector(1.,1.),
                to: Vector(3.,3.),
        });
        assert_eq!(i1,Option::Some(Frame {
                from: Vector(1.,1.),
                to: Vector(2.,2.),
        }));
        
    }
}
