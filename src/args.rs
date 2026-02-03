use clap::Parser;




// Args for free
#[derive(Parser, Debug)]
pub struct Args{


    #[arg(short,long)]
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
    pub total: bool,


    #[arg(short,long)]
    pub seconds: i32,

    #[arg(short,long)]
    pub lohi: bool,

    // formatting
    #[arg(short,long)]
    pub wide: bool,

    #[arg(short = 'L',long)]
    pub line: bool,

    #[arg(short = 'v',long)]
    pub comitted: bool,




    // version
    #[arg(short = 'V',long)]
    pub version: bool,

   
}
