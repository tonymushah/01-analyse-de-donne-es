use std::io::Read;

use csv::Reader;
use serde::{Deserialize, Serialize};
use time::Date;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CommandesDataItem {
    pub society: String,
    pub n_employe: usize,
    pub city: String,
    pub country: String,
    pub command_date: Date,
    pub n_command: usize,
    pub product: String,
    pub total_price: String,
}

impl CommandesDataItem {
    pub fn from_csv_reader<R: Read>(reader: &mut Reader<R>) -> Vec<Self> {
        reader.deserialize::<Self>().flatten().collect()
    }
}
