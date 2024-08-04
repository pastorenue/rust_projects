mod generic_funcs;
mod generic_struct;
mod compute;

use generic_struct::{Point, Point2};
use compute::{list_sum, list_product};

fn main() {
   let wont_work = Point2 { x: 5, y: 5.0 };
   let will_work = Point2 { x: 5, y: 4.0 };
   let point1: Point<i32> = Point::new(5, 5);
   let point2: Point<i32> = Point::new(6, 5);

   println!("{:?}", wont_work.mixup(will_work));
   println!("{:?}", point1 < point2);
   list_sum(vec![2,344,2,1]);
   list_product(vec![2,4,2,1]);
}
