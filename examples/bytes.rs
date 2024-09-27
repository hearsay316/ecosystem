use bytes::{BufMut, BytesMut};

fn main() -> anyhow::Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello word\\");

    buf.put(&b"goodbye word"[..]);
    buf.put_i32(12);
    println!("{:?}", buf);
    let a = buf.split();
    let mut b = a.freeze();
    // let data_to_search = &b"10"[..];
    // let pos = b.binary_search(data_to_search).unwrap();
    let c = b.split_to(12);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?} ", buf);
    Ok(())
}
