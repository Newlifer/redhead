use std::sync::{Arc, RwLock};
use std::option::Option;

use uuid::Uuid;


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

pub struct RecFormat {
    cols: Vec<CellFormat>
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

trait Container<T> {
    fn size(&self) -> usize;
    fn push(&mut self, item: T);
    //fn remove
}

pub struct Rec {
    pub guid: Uuid,
    pub cells: Vec<Cell>,
    pub format: Arc<RwLock<RecFormat>>
}

pub fn construct_rec(format: &RecFormat) -> Rec {
    Rec{guid: Uuid::new_v4(),
        cells: vec![],
        Arc::new(RwLock::new(format))}
}

impl Container<Cell> for Rec {

    fn size(&self) -> usize {
        return self.cells.len();
    }

    fn push(&mut self, item: Cell) {
        self.cells.push(item);
    }
}

pub struct Table {
    pub name: String,
    pub rows: Vec<Rec>,
    pub format: Arc<RwLock<RecFormat>>
}

impl Container<Rec> for Table {
    fn size(&self) -> usize {
        return self.rows.len();
    }

    fn push(&mut self, item: Rec) {
        self.rows.push(item);
    }
}
