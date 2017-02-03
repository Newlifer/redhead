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
    pub fieldname: String,
    pub fieldtype: CellType
}

pub struct RecFormat {
    pub cols: Vec<CellFormat>
}

impl RecFormat {
    pub fn new(fields: Vec<CellFormat>) -> Arc<RwLock<RecFormat>> {
        return Arc::new(RwLock::new(RecFormat { cols: fields } ));
    }
}

/*
trait Formattable {
    fn own_format(&mut self) {
        self.format = RecFormat::new(self.format.cols);
    }
}
*/

pub struct Rec {
    pub guid: Uuid,
    pub cells: Vec<CellType>,
    pub format: Arc<RwLock<RecFormat>>
}

impl Rec {
    pub fn new(format: Arc<RwLock<RecFormat>>) -> Rec {
        let cells = format
                        .read()
                        .unwrap()
                        .cols
                        .iter()
                        .map(|x| x.fieldtype.clone()).collect::<Vec<CellType>>();
        return Rec { guid: Uuid::new_v4(),
                     cells: cells,
                     format: format
                   };
    }
}

/*
impl Formattable for Rec {
}
*/

pub struct RecCollection {
    pub rows: Vec<Rec>,
    pub format: Arc<RwLock<RecFormat>>
}

impl RecCollection {
    pub fn new(format: Arc<RwLock<RecFormat>>) -> RecCollection {
        return RecCollection {
            format: format,
            rows: Vec::new()
        };
    }
}

pub struct Table {
    pub data: RecCollection,
    pub guid: Uuid,
    pub name: String
}