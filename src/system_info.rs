use std::time::{SystemTime, UNIX_EPOCH};

pub fn report_sys_info()  {
    println!("\n ---------------OS and CPU info----------------- \n" );
    //time::Instant::now().elapsed().as_nanos() as u64
    println!("Operation system: \t{}", std::env::consts::OS);
    println!("OS Family: \t\t{}", std::env::consts::FAMILY);
    
    // 架构
    println!("Architecture: \t\t{}", std::env::consts::ARCH);
}

