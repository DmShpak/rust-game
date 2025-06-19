use vector::vector::Vector;


use super::frame::Frame;

#[derive(Debug)]
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
            from: self.location.clone().sub(half).to_owned(),
            to: self.location.clone().add(half).to_owned(),
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
}
