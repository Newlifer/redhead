use std::sync::{Arc, RwLock};
use std::option::Option;
use std::clone::Clone;

use uuid::Uuid;


type OptInt32 = Option<i32>;
type OptInt64 = Option<i64>;
type OptString = Option<String>;


#[derive(Clone)]
pub enum CellType {
    Int32 (OptInt32),
    Int64 (OptInt64),
    Text  (OptString)
}

pub struct CellFormat {
    pub name: String,
    pub type_: CellType
}

pub struct RecFormat {
    pub cols: Vec<CellFormat>
}

impl RecFormat {
    pub fn new(fields: Vec<CellFormat>) -> Arc<RwLock<RecFormat>> {
        return Arc::new(RwLock::new(RecFormat{cols: fields}));
    }
}

pub fn construct_recformat(fields: Vec<CellFormat>) -> Arc<RwLock<RecFormat>> {
    return Arc::new(RwLock::new(RecFormat{cols: fields}));
}

pub struct Rec {
    pub guid: Uuid,
    pub cells: Vec<CellType>,
    pub format: Arc<RwLock<RecFormat>>
}

pub fn construct_rec(format: Arc<RwLock<RecFormat>>) -> Rec {
    let cells = format.read().unwrap().cols.iter().map(|x| x.type_.clone()).collect::<Vec<CellType>>();
    return Rec{guid: Uuid::new_v4(),
        cells: cells,
        format: format}
}

pub struct Table {
    pub name: String,
    pub rows: Vec<Rec>,
    pub format: Arc<RwLock<RecFormat>>
}
