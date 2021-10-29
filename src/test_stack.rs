pub enum List{
    empty,
    Ele(i32, Box<List>)
}

#[test]
fn test() {
    let l = List::Ele(1,Box::new(List::empty));
}