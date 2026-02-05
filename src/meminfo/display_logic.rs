use std::{collections::HashMap};

    const W_LABEL: usize = 10; // Ширина заголовка (Mem:, Swap:)
    const W_COL: usize = 16;   // Ширина столбцов с цифрами (было 12, ставим больше)
    const W_COL_LINE: usize = 8;
pub enum DisplayFormat{
    Default,
    Wide,
    Lohi,
    Line

}

impl DisplayFormat{

    pub fn display(&self, data: &HashMap<String, String>) {


      

        match &self{
            DisplayFormat::Default => def_out(data),
            DisplayFormat::Wide => wide_out(data),
            DisplayFormat::Lohi => lohi_out(data),
            DisplayFormat::Line => line_out(data),
        }


    }
}

fn def_out(data : &HashMap<String, String>){


    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}", 
        "", "total", "used", "free", "shared", "buff/cache", "available", wl = W_LABEL, wc = W_COL );

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}", 
        "Mem:",
        val(data, "MemTotal"),
        val(data, "MemUsed"),
        val(data, "MemFree"),
        val(data, "Shared"), // или что там в стандартном
        val(data, "BuffCache"),
        val(data, "MemAvailable"),
        wl = W_LABEL, wc = W_COL 
    );

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}", 
        "Swap:",
        val(data, "SwapTotal"),
        val(data, "SwapUsed"),
        val(data, "SwapFree"),
        wl = W_LABEL, wc = W_COL 
    );


    



}


fn wide_out(data : &HashMap<String, String>){

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}", 
        "", "total", "used", "free", "shared", "buffers", "cache","available", wl = W_LABEL, wc = W_COL   );

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}", 
        "Mem:",
        val(data, "MemTotal"),
        val(data, "MemUsed"),
        val(data, "MemFree"),
        val(data, "Shared"), // или что там в стандартном
        val(data, "Buffers"),
        val(data, "Cached"),
        val(data, "MemAvailable"),
        wl = W_LABEL, wc = W_COL 
    );

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}", 
        "Swap:",
        val(data, "SwapTotal"),
        val(data, "SwapUsed"),
        val(data, "SwapFree"),
        wl = W_LABEL, wc = W_COL 

    );

}


fn lohi_out(data : &HashMap<String, String>){




    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}", 
        "", "total", "used", "free", "shared", "buff/cache", "available", wl = W_LABEL, wc = W_COL);

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}{:>wc$}", 
        "Mem:",
        val(data, "MemTotal"),
        val(data, "MemUsed"),
        val(data, "MemFree"),
        val(data, "Shared"), // или что там в стандартном
        val(data, "BuffCache"),
        val(data, "MemAvailable"),
        wl = W_LABEL, wc = W_COL
    );

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}", 
        "Low:",
        val(data, "LowTotal"),
        val(data, "LowUsed"),
        val(data, "LowFree"),
        wl = W_LABEL, wc = W_COL

    );

    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}", 
        "High:",
        val(data, "HighTotal"),
        val(data, "HighUsed"),
        val(data, "HighFree"),
        wl = W_LABEL, wc = W_COL

    );


    println!("{:<wl$}{:>wc$}{:>wc$}{:>wc$}", 
        "Swap:",
        val(data, "SwapTotal"),
        val(data, "SwapUsed"),
        val(data, "SwapFree"),
        wl = W_LABEL, wc = W_COL

    );

}


fn line_out(data : &HashMap<String, String>){


    println!("{:<8}{:>wc$}{:>wcl$}{:>wc$}{:>wcl$}{:>wc$}{:>wcl$}{:>wc$}", 
        "SwapUse",
        val(data, "SwapUsed"),
        "CachUse",
        val(data, "BuffCache"),
        "MemUse", // или что там в стандартном
        val(data, "MemUsed"),
        "MemFree",
        val(data, "MemFree"),
        wcl = W_COL_LINE, wc = W_COL
    );
   
}


fn val<'a>(data: &'a HashMap<String, String>, key: &str) -> &'a str {
    data.get(key).map(|s| s.as_str()).unwrap_or("0")
}