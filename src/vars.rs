// pub mod sub_a;
// pub mod sub_b;

const MAX_POINT: u32 = 100_000;

pub fn run() {
    // module
    // bin crateから読み込みする用
    println!("Here is vars module");
    // sub_a::func_a();
    // sub_b::func_b();

    let mut x = 5;
    println!("the value x is :{}", x);
    x = 6;
    println!("the value x is :{}", x);

    let _i1 = 3;
    let _f1 = 0.1;
}
