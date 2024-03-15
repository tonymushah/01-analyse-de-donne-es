use csv::ReaderBuilder;
use data_analyse_01::structs::comandes_data_item::CommandesDataItem;
use std::{fs::File, io::BufReader};

fn main() -> anyhow::Result<()> {
    let file = BufReader::new(File::open("./commandes_data.csv")?);
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let datas = CommandesDataItem::from_csv_reader(&mut reader)?;
    println!("{:#?}", datas);
    Ok(())
}
