pub mod shapes {
    use std::fmt::Display;

    pub struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        pub fn x(&self) -> &T {
            &self.x
        }

        pub fn y(&self) -> &U {
            &self.y
        }
    }

    impl <T, U> Point<T, U> {
        pub fn new(x: T, y: U) -> Point<T, U> {
            Point {
                x,
                y,
            }
        }
    }
    impl<T: Display + PartialOrd, U: Display + PartialOrd> Point<T, U> where T: PartialOrd<U> {
        pub fn cmp_display(&self) {
            if self.x > self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}


