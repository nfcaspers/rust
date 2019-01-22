fn main() {
    //bind closure to f 
    let f = |x| x + 1;
    let y  = 10;
    println!("Closure: {}", f(y));

    //closure without arguments
    let d = || println!("No arguments!!!");
    d();

    //return from closure
    let e = |y| return y;
    println!("e returned: {}", e(230));

    capture_closure();
    pass_closure();
}

fn capture_closure() {
    let mut cnt = 0;
    let mut increment = || {
        cnt += 1;
        println!("Incremented cnt by 1: {}", cnt)
    };
    increment();
    increment();
}

fn pass_closure() {
    let hw = || println!("Hello, world!");
    fn1(hw);

    let mlt = |x| x * 5;
    let result = change(mlt, 5);
    println!("Result of mlt is {}", result); 
}

fn fn1<F>(f: F)
where F: Fn() {
    f();
}

fn change<F>(f: F, val: i32) -> i32
where F: Fn(i32) -> i32 {
    f(val)
}