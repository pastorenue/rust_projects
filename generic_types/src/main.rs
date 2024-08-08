mod generic_funcs;
mod generic_struct;
mod compute;

use compute::{Point as FloatPoint, use_point};

fn main() {
   let point1: FloatPoint<f32> = FloatPoint{x: 6.0, y: 5.0};
   let point2: FloatPoint<f32> = FloatPoint{x: 6.0, y: 5.0};

   println!("{:?}", use_point(point1, point2));
}
