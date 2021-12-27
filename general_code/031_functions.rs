fn main() {

    fn answer_to_life_the_universe_and_everything() -> i32 {
        return 42;
    }

    #[warn(overflowing_literals)]
    fn another_function(x: i32) {
        println!("The value of x is: {}", x);
    }

answer_to_life_the_universe_and_everything();

another_function(6);

//// On x86 (32-bit, 4-byte):
//    - 0x00000000 ->  0
//    - 0xffffffff -> -1
//    - 0x7fffffff ->  2147483647 (INT_MAX)
//    - 0x80000000 -> -2147483648 (INT_MIN)

//    0x00000001 + 0x00000002 = 0x00000003 ( 1 + 2 = 3)
//    0xffffffff + 0x00000002 = 0x00000001 (-1 + 2 = 1)
//    0xffffffff + 0xfffffffe = 0xfffffffd (-1 +-2 =-3)

//another_function(2147483648);
}
