extern crate uuid;

use uuid::Uuid;

use std::option::Option;

type OptInt32 = Option<i32>;
type OptInt64 = Option<i64>;
type OptString = Option<String>;

pub enum CellType {
    Int32 (OptInt32),
    Int64 (OptInt64),
    Text  (OptString)
}

trait Metainf {
    fn name(&self) -> String;
}

impl Metainf for CellType {
    fn name(&self) -> String {
        return match *self {
            CellType::Int32(_) => "INT32".to_string(),
            CellType::Int64(_) => "INT64".to_string(),
            CellType::Text(_) => "TEXT".to_string()
        }
    }
}

pub struct CellFormat {
    name: String,
    type_: CellType
}

trait Metaformat<T> {
    fn construct(format_: CellFormat) -> T;
}

pub struct Cell {
    value: CellType
}

impl Metaformat<Cell> for Cell {
    fn construct(format_: CellFormat) -> Cell {
        return match format_.type_ {
            CellType::Int32(_) => Cell { value: CellType::Int32(None) },
            CellType::Int64(_) => Cell { value: CellType::Int64(None) },
            CellType::Text(_) => Cell { value: CellType::Text(None) }
        }
    }
}

pub struct RowFormat {
    cols: Vec<CellFormat>
}

trait Container<T> {
    fn size(&self) -> usize;
    fn push(&mut self, item: T);
    //fn remove
}

pub struct Row {
    guid: Uuid,
    cells: Vec<Cell>
}

impl Container<Cell> for Row {
    fn size(&self) -> usize {
        return self.cells.len();
    }

    fn push(&mut self, item: Cell) {
        self.cells.push(item);
    }
}

pub struct Table {
    name: String,
    rows: Vec<Row>
}

impl Container<Row> for Table {
    fn size(&self) -> usize {
        return self.rows.len();
    }

    fn push(&mut self, item: Row) {
        self.rows.push(item);
    }
}
