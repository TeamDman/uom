use uom::si::f64::Time;
use uom::si::time::{second, millisecond, microsecond, nanosecond};

fn main() {
    println!("=== Demonstrating Precision Difference ===");
    println!();
    
    // Create a time with high precision: 1.234567890 seconds
    let precise_time = Time::new::<second>(1.0) + 
                      Time::new::<millisecond>(234.0) + 
                      Time::new::<microsecond>(567.0) + 
                      Time::new::<nanosecond>(890.0);
    
    println!("High precision time: 1s + 234ms + 567μs + 890ns");
    println!("get_human():       {}", precise_time.get_human());
    println!("get_human_nanos(): {}", precise_time.get_human_nanos());
    println!();
    
    // Test with very small durations
    let tiny_time = Time::new::<microsecond>(123.456);
    println!("123.456 microseconds:");
    println!("get_human():       {}", tiny_time.get_human());
    println!("get_human_nanos(): {}", tiny_time.get_human_nanos());
    println!();
    
    // Test with sub-millisecond that rounds up
    let sub_ms = Time::new::<microsecond>(999.9);
    println!("999.9 microseconds:");
    println!("get_human():       {}", sub_ms.get_human());
    println!("get_human_nanos(): {}", sub_ms.get_human_nanos());
    println!();
    
    println!("=== Key Differences ===");
    println!("• get_human() truncates after milliseconds for cleaner output");
    println!("• get_human_nanos() shows full precision including μs and ns");
    println!("• Both handle negative durations and edge cases");
}
