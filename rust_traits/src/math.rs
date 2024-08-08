pub mod interfaces {
    pub trait PolygonalMath {
        fn area(&self) -> i32;
        fn perimeter(&self) -> i32;
    }

    pub trait VolumetricMath {
        fn volume(&self) -> i32;
    }
}

pub mod shapes {
    use super::interfaces::PolygonalMath;

    pub struct Square {
        pub side: i32
    }

    pub struct Rectangle {
        pub width: i32,
        pub height: i32
    }

    pub struct Triangle {
        side1: i32,
        side2: i32,
        side3: i32
    }

    pub struct Circle {
        radius: i32
    }

    pub struct Trapezoid {
        base1: i32,
        base2: i32,
        height: i32
    }

    impl Square {
        /// A string representation of the `Square` struct.
        /// ::returns:: -> &str
        pub fn as_str(&self) -> String {
            format!("<Square: (side: {})>", self.side)
        }
    }

    impl PolygonalMath for Square {
        /// Calculates the area of a square.
        /// ::returns:: -> i32
        fn area(&self) -> i32 {
            self.side * self.side
        }

        /// Calculates the perimeter of a square.
        /// ::returns:: -> i32
        fn perimeter(&self) -> i32 {
            self.side * 4
        }
    }

    impl PolygonalMath for Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }

        fn perimeter(&self) -> i32 {
            (self.width + self.height) * 2
        }
    }

    impl Rectangle {
        pub fn as_str(&self) -> String {
            format!("<Rectangle: (width: {}, height: {})>", self.width, self.height)
        }
    }
}

pub mod solids {
    use super::interfaces::VolumetricMath;

    pub enum Solid {
        Cube {side: i32},
        Cuboid {side: i32},
        Sphere {radius: i32},
        Cylinder {radius: i32, height: i32}
    }

    struct Cube {
        pub side: i32
    }

    struct Cuboid {
        pub side: i32
    }

    struct Sphere {
        pub radius: i32
    }

    struct Cylinder {
        pub radius: i32,
        pub height: i32
    }

    impl VolumetricMath for Solid {
        fn volume(&self) -> i32 {
            match self {
                Solid::Cube {side} => side * side * side,
                Solid::Cuboid {side} => side * side * side,
                Solid::Sphere {radius} => {
                    let vol = ((4.0 / 3.0) * std::f64::consts::PI) as i32 * radius * radius * radius;
                    vol as i32
                }
                Solid::Cylinder {radius, height} => {
                    let vol = std::f64::consts::PI as i32 * radius * radius * height;
                    vol as i32
                }
            }
        }
    }
}


mod tests {
    use super::interfaces::{PolygonalMath, VolumetricMath};

    #[test]
    fn test_square() {
        let square = super::shapes::Square { side: 10 };
        assert_eq!(square.as_str(), "<Square: (side: 10)>");
        assert_eq!(square.area(), 100);
        assert_eq!(square.perimeter(), 40);
    }

    #[test]
    fn test_rectangle() {
        let rec = super::shapes::Rectangle {width:5, height: 4};
        assert_eq!(rec.as_str(), "<Rectangle: (width: 5, height: 4)>");
        assert_eq!(rec.area(), 20);
        assert_eq!(rec.perimeter(), 18);
    }

    #[test]
    fn test_cube() {
        let cube = super::solids::Solid::Cube { side: 10 };
        assert_eq!(cube.volume(), 1000);
    }

    #[test]
    fn test_cubiod() {
        let cuboid = super::solids::Solid::Cuboid { side: 10 };
        assert_eq!(cuboid.volume(), 1000);
    }
}