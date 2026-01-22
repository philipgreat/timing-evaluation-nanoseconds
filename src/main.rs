mod date_time_tool;
mod system_info;
mod high_resolution_timer;
use crate::date_time_tool::current_timestamp;
use crate::system_info::report_sys_info;
use crate::high_resolution_timer::HighResolutionCounter;

pub fn print_performance_stats(start_ns: u64, end_ns: u64, loop_count: u64) {
    if end_ns < start_ns {
        println!("Error: end time must be after start time");
        return;
    }
    
    let elapsed_ns = end_ns - start_ns;
    
    println!("Time consumed: \t\t{} ns", elapsed_ns);
    println!("Loop count: \t\t{}", loop_count);
    
    if loop_count > 0 {
        let ns_per_call = elapsed_ns / loop_count;
        if ns_per_call==0 {
             let ns_per_call = (elapsed_ns as f64 )/(loop_count as f64) ;
             println!("Time per call: \t\t{} ns", ns_per_call);

        }else{
            println!("Time per call: \t\t{} ns", ns_per_call);
        }

    } else {
        println!("Time per call: \t\tN/A (loop count is 0)");
    }
}

fn main() {
    
    report_sys_info();
    
    println!("\n---------- System call SystemTime::now() -------------\n" );
    
    let  start = current_timestamp();
    let  loop_count = 10_000_000;
    let mut last = 0;
    for _ in 0..loop_count {
        last = current_timestamp();
    }
    let end=current_timestamp();

    print_performance_stats(start,end,loop_count);
    println!("show last to prevent optimized by compiler {} \n",last);

    
    println!("\n---------- High Resolution Time with CPU tick-------------\n" );


    let start = current_timestamp();
    let loop_count = 10_000_000;
    let tenth_of_giga = 100_000_000;
    
    let timer = HighResolutionCounter::start(28*tenth_of_giga);
    let mut last = 0;
    for _ in 0..loop_count {       
        last = timer.ns();
    }
    println!("show last to prevent optimized by compiler {} \n",last);
    let end = current_timestamp();
    print_performance_stats(start,end,loop_count);


    println!("\n====================================================\n" );

    

}

