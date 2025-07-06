use vector::vector::Vector;


use super::frame::Frame;

#[derive(Debug,Clone)]
pub struct Rectangle {
    // The center point of rectangle
    pub location: Vector,
    // width and height
    pub dementions: Vector,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        return self.dementions.0 * self.dementions.1;
    }

    pub fn to_frame(&self) -> Frame {
        let mut dementions = self.dementions.clone();
        let half = dementions.scale(0.5);
        return Frame {
            from: self.location.clone_sub(half).to_owned(),
            to: self.location.clone_add(half).to_owned(),
        };
    }

    pub fn shift(&mut self, offs: &Vector) {
        self.location.add(offs);
        return;
    }

    pub fn clone_shift(&mut self, offs: &Vector) -> Rectangle{
        let mut res = self.clone();
        res.shift(offs);
        return res;
    }

    pub fn from_frame(f: &Frame) -> Rectangle {
        let  dementions = f.to.clone_sub(&f.from);
        let  location = f.from.clone_add(&f.to).scale(0.5).to_owned();

        return Rectangle {
            dementions,
            location,
        };
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let r = Rectangle{
                location: Vector(0.,0.),
                dementions: Vector(3.,4.),
        };
        let a = r.area();
        assert_eq!(a, 12.);
    }

    
    #[test]
    fn to_frame() {
        let r = Rectangle{
                location: Vector(0.,0.),
                dementions: Vector(3.,4.),
        };
        let f = r.to_frame();
        assert_eq!(f.from, Vector(-1.5,-2.));
        assert_eq!(f.to, Vector(1.5,2.));
    }

}
