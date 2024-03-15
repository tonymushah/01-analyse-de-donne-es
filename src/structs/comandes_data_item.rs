use std::io::Read;

use csv::Reader;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use time::{macros::format_description, Date};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CommandesDataItem {
    pub society: String,
    pub n_employe: usize,
    pub city: String,
    pub country: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub command_date: Date,
    pub n_command: usize,
    #[serde(alias = "product_name")]
    pub product: String,
    pub total_price: String,
}

struct CommandesDataItemDateVisitor;

impl<'de> Visitor<'de> for CommandesDataItemDateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            formatter,
            "Excepted string with format [day]/[month]/[year]"
        )
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let format = format_description!("[day]/[month]/[year]");
        Date::parse(v, format).map_err(serde::de::Error::custom)
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(CommandesDataItemDateVisitor)
}

impl CommandesDataItem {
    pub fn from_csv_reader<R: Read>(reader: &mut Reader<R>) -> csv::Result<Vec<Self>> {
        reader.deserialize::<Self>().collect()
    }
}
