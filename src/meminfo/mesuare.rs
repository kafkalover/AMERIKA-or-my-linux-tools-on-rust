use std::collections::HashMap;

pub trait ToMesuare{
    fn convert(&self, num: u64) -> String;
}
pub enum BiMeasure{
    Bytes,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi
}


pub enum DefaultMeasure{
    Bytes,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta
}

impl ToMesuare for  BiMeasure{
    fn convert(&self, num: u64) -> String {
        
        match self {
            Self::Bytes => format!("{} B", num * 1000),
            Self::Kibi => format!("{} KB", num / 1000),
            Self::Mebi => format!("{} MB", num / 1000 / 1000),
            Self::Gibi => format!("{} GB", num / 1000 / 1000 / 1000),
            Self::Tebi => format!("{} TB", num / 1000 / 1000 / 1000 / 1000),
            Self::Pebi => format!("{} PB", num / 1000 / 1000 / 1000 / 1000 / 1000),
    }
}
}



impl ToMesuare for  DefaultMeasure{
    fn convert(&self, num: u64) -> String {
        match self{
            Self::Bytes => format!("{} B", num * 1000),
            Self::Kilo => format!("{} KiB", num),
            Self::Mega => format!("{} MiB", num / 1024),
            Self::Giga => format!("{} GiB", num / 1024 / 1024),
            Self::Tera => format!("{} TiB", num / 1024 / 1024 / 1024),
            Self::Peta => format!("{} PiB", num / 1024 / 1024 / 1024 / 1024),
        }
    }
}

pub fn convert_collection_value<T: ToMesuare>(data: &mut HashMap<String, u64>, to_mesuare: T ) -> HashMap<String, String>{

    let mut _converting_collection:HashMap<String, String> = HashMap::new();
    
    for (k,i) in data{

        _converting_collection.insert(k.clone(), to_mesuare.convert(*i));

    }

    _converting_collection



}