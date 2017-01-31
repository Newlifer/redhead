enum Type {
    Int32(i32)
}


struct Row {
    cells: Vec<Type>
}

struct Table {
    rows: Vec<Row>
}
