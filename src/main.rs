mod util {
    pub type AwesomeArray = [i32; 5];

    pub struct BubbleSort {
        pub desc: bool,
        pub arr: AwesomeArray
    }

    pub fn print_array(arr: AwesomeArray) {
        for i in arr {
            println!("Current item: {}", i);
        }
    }

    impl BubbleSort {
        fn cmp(&self, a: i32, b: i32, inv: bool) -> bool {
            return if inv { b > a } else { a > b };
        }

        pub fn sort(&self) -> AwesomeArray {

            let mut buf = [0; 5];

            // copy the array
            // arr[0] = 123; // we don't do that here
            for i in 0..self.arr.len() {
                buf[i] = self.arr[i];
            }

            for i in 0..buf.len() {
                // - i term for growing buuble :)
                for j in 0..buf.len() - 1 - i {
                    if self.cmp(buf[j], buf[j + 1], self.desc) {
                        // and just swap elements
                        buf.swap(j, j + 1);
                    }
                }
            }

            return buf;
        }
    }

    impl Default for BubbleSort {
        fn default() -> BubbleSort {
            BubbleSort {
                arr: [0; 5], 
                desc: false
            }
        }
    }
}

use util::AwesomeArray;
use util::BubbleSort;
use util::print_array;

fn main() {
    let a: AwesomeArray = [3, 4, 5, 1, 2];

    let bsort = BubbleSort { arr: a, ..Default::default() };
    let b = bsort.sort();

    let asort = BubbleSort { arr: a, desc: true };
    let aa = asort.sort();

    print_array(b);

    println!();

    print_array(aa);
}
