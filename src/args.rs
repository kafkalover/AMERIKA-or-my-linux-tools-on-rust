use clap::{Parser, ArgGroup};
use crate::meminfo::{BiMeasure,  DisplayFormat};



pub struct LoopConfig {
    pub interval: u64,
    pub count: usize,
}

pub struct AppConfig<T> {
    
    pub mesuare: T,             // type_size
    pub mode: DisplayFormat,       // display
    pub behavior: LoopConfig, // behavior (Option, т.к. может быть один проход)
}


impl From<Args> for AppConfig<BiMeasure>{

    fn from(args: Args) -> Self{

        let mes;
        let mode;





            mes = if args.bytes{
               BiMeasure::Bytes
            } else if args.mega{
               BiMeasure::Mebi
            } else if args.giga{
               BiMeasure::Gibi
            }
            else if args.tera{
               BiMeasure::Tebi
            }
            else if args.peta{
               BiMeasure::Pebi
            } else {
                BiMeasure::Kibi
            };
        



        //

        mode = if args.wide{
            DisplayFormat::Wide
        } else if args.line{
            DisplayFormat::Line
        } else if args.lohi{
            DisplayFormat::Lohi
        } else { 
            DisplayFormat::Default
        };


        //

        let sec = if args.seconds.is_some(){
            args.seconds.unwrap()
        } else {
            0
        };

        let count = if args.count.is_some(){
            args.count.unwrap()
        } else {
            0
        };       

        let _beh = LoopConfig{
            interval: sec,
            count: count
        };

        Self { mesuare: mes
            , mode
            , behavior: _beh  }




    }
}


// Args for free
#[derive(Parser, Debug)]
#[command(author, version, about = "Clone of free utility written in Rust", long_about = None)]
#[command(group(
    ArgGroup::new("mesaure")
        .required(false)
        .args(["bytes", "kilo", "mega", "giga", "tera", "peta",  "human", "mebi","gibi","tebi", "pebi"])
    ))]
#[command(group(
    ArgGroup::new("display_mode")
        .required(false)
        .args(["lohi", "wide", "line"])
    ))]
pub struct Args{


    #[arg(short = 'H',long)]
    pub human: bool,

    // base bi size
    #[arg(short,long)]
    pub bytes: bool,

    #[arg(short,long, default_value_t = true)]
    pub kibi: bool,

    #[arg(short,long)]
    pub mebi: bool,

    #[arg(short,long)]
    pub gibi: bool,

    // rare size

    // other pebi size 
    #[arg(long)]
    pub tebi: bool,

    #[arg(long)]
    pub pebi: bool,

    // default size 
    #[arg(long)]
    pub kilo: bool,
    
    #[arg(long)]
    pub mega: bool,

    #[arg(long)]
    pub giga: bool,

    #[arg(long)]
    pub tera: bool,

    #[arg(long)]
    pub peta: bool,


    // display full info about memory


    #[arg(short,long)]
    pub lohi: bool,

    // formatting
    #[arg(short,long)]
    pub wide: bool,

    #[arg(short = 'L',long)]
    pub line: bool,

  


    //mode work

    /// Display continuously every N seconds
    #[arg(short = 's', long)]
    seconds: Option<u64>,

    /// Number of updates (used with -s)
    #[arg(short = 'c', long)]
    count: Option<usize>,



   
}
