use std::{collections::HashMap};

// обогащает изначальную коллекцию
pub fn enrich_memory_data(data: &mut HashMap<String, u64>) {


    // 1. Базовая математика для Mem 

    // Сырье
    let total = *data.get("MemTotal").unwrap_or(&0);
    let free = *data.get("MemFree").unwrap_or(&0);
    let buffers = *data.get("Buffers").unwrap_or(&0);
    let cached = *data.get("Cached").unwrap_or(&0);
    let reclaimable = *data.get("SReclaimable").unwrap_or(&0);
    let swap_total = *data.get("SwapTotal").unwrap_or(&0);
    let swap_free = *data.get("SwapFree").unwrap_or(&0);
    

    // Стандартный MemUsed (без кэшей)
    let total_cache = cached + reclaimable;
    let main_used = total.saturating_sub(free + buffers + total_cache);
    data.insert("MemUsed".to_string(), main_used);

    // SwapUsed
    let swap_used = swap_total.saturating_sub(swap_free);
    data.insert("SwapUsed".to_string(), swap_used);

    // BuffCache

    // cache
    let total_cache_size = cached + reclaimable; 
    
    // Buff/Cache 
    let buff_cache = buffers + total_cache_size;
    data.insert("BuffCache".to_string(), buff_cache);





    // 2. Логика для LoHi 
    
    
    lohi_enrich(data, &total, &free);

}


fn lohi_enrich(data: &mut HashMap<String, u64>, total: &u64, free: &u64){




        
    
    // Проверяем, есть ли HighTotal в файле (признак 32-битной системы с PAE)
    let has_high_mem = data.contains_key("HighTotal");

    let (low_total, low_free, high_total, high_free);

    if has_high_mem {
        // Если ядро отдало эти данные — берем их "как есть"
        low_total = *data.get("LowTotal").unwrap_or(&0);
        low_free = *data.get("LowFree").unwrap_or(&0);
        high_total = *data.get("HighTotal").unwrap_or(&0);
        high_free = *data.get("HighFree").unwrap_or(&0);
    } else {
        // Если данных нет (64-бит) — ЭМУЛИРУЕМ
        // Вся память считается Low, High отсутствует
        low_total = *total;
        low_free = *free; 
        high_total = 0;
        high_free = 0;
    }

    // 3. Считаем Used для LoHi
    
    let low_used = low_total.saturating_sub(low_free);
    let high_used = high_total.saturating_sub(high_free);

    // 4. Записываем вычисленные (или скопированные) значения обратно

    data.insert("LowTotal".to_string(), low_total);
    data.insert("LowFree".to_string(), low_free);
    data.insert("LowUsed".to_string(), low_used); 

    data.insert("HighTotal".to_string(), high_total);
    data.insert("HighFree".to_string(), high_free);
    data.insert("HighUsed".to_string(), high_used); 
}


