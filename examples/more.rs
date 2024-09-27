use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Copy, Clone, From, Add, Into, Display)]
struct MyInt(i32);

#[derive(PartialEq)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(PartialEq, From, Add, Display, Debug)]
enum MyEnum {
    #[display("int :{_0}")]
    Int(i32),
    Unit(u32),
    #[display("nothing")]
    Noting,
}
fn main() -> anyhow::Result<()> {
    let a = MyInt::from(10);
    let _b = MyInt::from(10);
    let c: MyInt = 10.into();
    println!("{}", a + c);
    let e: MyEnum = 10i32.into();
    let e1: MyEnum = 11u32.into();
    let e3: MyEnum = MyEnum::Noting;
    println!("e: {e:?}  e1: {e1:?}  e3 {e3:?}");
    Ok(())
}
