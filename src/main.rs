mod args;
mod meminfo;





use clap::Parser;

use crate::meminfo::{MemInfo};

use crate::meminfo::{BiMeasure,convert_collection_value };

use crate::meminfo::enrich_memory_data;
use std::thread;
use std::time::Duration;


use crate::args::{AppConfig,Args};

fn main() -> anyhow::Result<()> {
    


    // 1. init

    let config: AppConfig<BiMeasure> =  Args::parse().into();

    let _mesuare = config.mesuare;

    let _format = config.mode;

    let mut _mem_data = MemInfo::create_mem_info()?;

    let _data = &mut _mem_data.map;
  
    // 2. Enrich mem data 

    enrich_memory_data(_data);



    // 3. Converting value 

    let _format_data = &convert_collection_value(_data, _mesuare);


    // 4. Print 


    if config.behavior.count != 0{

        let mut  n = config.behavior.count;

        if config.behavior.interval != 0{


            while n >0{
            usleep(config.behavior.interval);
            _format.display(_format_data);
            n = n-1;
            }

        }else{

            while n >0{
            _format.display(_format_data);
            n = n-1;
            }
        }
    }

    if config.behavior.interval !=0{
        loop{
        

        _format.display(_format_data);
        usleep(config.behavior.interval);


        }

    }
    else{
        loop{
        

        _format.display(_format_data);



        }

    }
    Ok(())
}


fn usleep(microseconds: u64) {
    thread::sleep(Duration::from_micros(microseconds));
}