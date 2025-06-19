use crate::vector::Vector;

/**
 * Polar representation of vector
 */
#[derive(Default, Clone, Debug)]
pub struct Polar {
    pub length: f32,
    pub rotation: f32,
}

impl Polar {
    pub fn as_vector (&self) -> Vector {
        let x = self.length * self.rotation.cos();
        let y = self.length * self.rotation.sin();
        return Vector(x,y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_vector() {

        let p = Polar{length: 1., rotation: 0.};
        let v = p.as_vector();

        assert_eq!(v.0, 1.);
        assert_eq!(v.1, 0.);
    }

}