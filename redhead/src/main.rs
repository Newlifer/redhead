extern crate uuid;

mod kernel;

use kernel::types::{ CellType,
                     CellFormat,
                     RecFormat,
                     Rec,
                     Table };

use std::sync::{Arc, RwLock};

fn main() {

    {
        let cell_format = CellFormat { name: "id".to_string(),
                                       type_: CellType::Int32(None)
                                     };

        let rec_format = RecFormat::new(vec![cell_format]);
        let rec = Rec::new(rec_format);
    }

    println!("Chpokey!");
}
