

pub enum List {
    Empty,
    ElemThenEmpty(i32),
    Elem(i32, Box<List>),
}