pub enum Type {
    Int32(i32)
}

pub struct Format {
}

pub struct Row {
    cells: Vec<Type>
}

pub struct Table {
    rows: Vec<Row>
}
