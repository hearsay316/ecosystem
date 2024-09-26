use std::ops::Deref;
use anyhow::Result;
use serde::Serialize;
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr, VariantArray, VariantNames};
use crate::Color::Yellow;

#[allow(unused)]
#[derive(Display, Debug,Serialize)]
enum Color {
    #[strum(serialize = "redred",to_string="red")]
    Red,
    Green {
        range: usize
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize
    }
}


#[allow(unused)]
#[derive(Debug, EnumString, EnumCount, EnumDiscriminants,
    EnumIter, EnumIs, IntoStaticStr, VariantNames)]
enum MyEnum {
    A,
    B(String),
    C,
    D
}
fn main() -> Result<()> {
    MyEnum::VARIANTS.iter().for_each(|v| println!("{:?}", v));
println!("total:{:?}",MyEnum::COUNT);
    MyEnum::iter().for_each(|v| println!("{:?}", v));
    let my_enum = MyEnum::B("hello".to_owned());
    println!("{:?}", my_enum.is_c());
    let s: &'static str = my_enum.into();
    println!("{:?}", s);
    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(20);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 30 };
    println!("{:?}", yellow);
    println!("{} ,{} ,{} ,{} <{}>", red, green, blue, yellow, purple);

    let serde_str_red = serde_json::to_string(&red)?;
    println!("{:?}",serde_str_red);
    Ok(())
}