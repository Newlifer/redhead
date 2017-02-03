extern crate uuid;

mod kernel;

use kernel::types::{CellType,
                    CellFormat,
                    RecFormat,
                    Rec,
                    Table,
                    construct_rec};

use std::sync::{Arc, RwLock};

fn main() {

    {
        let cell_format = ArcCellFormat{name: "id".to_string(),
                                     type_: CellType::Int32(None)};
        let rec_format = Arc::new(RwLock::new(RecFormat{cols: vec![cell_format]}));
        let rec = construct_rec(rec_format);
    }

    println!("Chpokey!");
}
