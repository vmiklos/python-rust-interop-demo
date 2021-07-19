trait Tr {
    fn foo(&self) -> i64;
}

#[derive(Debug)]
struct S {
}

impl Tr for S {
    fn foo(&self) -> i64 {
        0
    }
}

fn f<T: Tr>(arg: T) -> T {
    arg.foo();
    arg
}

fn main() {
    let s = S{};
    let r = f(s);
    println!("r is now {:?}", r);
}
