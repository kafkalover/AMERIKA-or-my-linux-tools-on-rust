use crate::meminfo::mem_info_struct::MemInfo;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader };
use anyhow::{Context};



impl MemInfo{

    pub fn create_mem_info() -> anyhow::Result<Self>{
        
        
        const MEM_PATH: &str = "/proc/meminfo";

        //open file
        let file = File::open(MEM_PATH).with_context(|| format!("Failed to open file at {}", MEM_PATH))?;


        // create reader
        let reader = BufReader::new(file);

        let mut map = HashMap::new();


        // reading /proc/meminfo

        for line in reader.lines() {


            let line = line?;

            // parsing 
            if let Some((key, value_str)) = line.split_once(":") {

                let value = value_str
                .trim()
                .trim_end_matches("kB")
                .trim();


                if let Ok(val) = value.parse::<u128>(){
                    map.insert(key.to_string() , val);
                }
            }

        }


        // return 
        Ok(Self{map})

    }


}