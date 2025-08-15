use uom::si::f64::Time;
use uom::si::time::{second, minute, hour, millisecond, microsecond, nanosecond};

fn main() {
    // Test various time durations
    let t1 = Time::new::<second>(45.0);
    let t2 = Time::new::<minute>(2.5);
    let t3 = Time::new::<hour>(1.0);
    let t4 = Time::new::<millisecond>(1500.0);
    let t5 = Time::new::<second>(90.0);
    
    // Test with sub-millisecond precision
    let t6 = Time::new::<microsecond>(1500.0); // 1.5ms
    let t7 = Time::new::<nanosecond>(1500000.0); // 1.5ms in nanoseconds
    let t8 = Time::new::<second>(1.0) + Time::new::<millisecond>(500.0) + Time::new::<microsecond>(250.0) + Time::new::<nanosecond>(125.0);

    println!("=== Comparison: get_human() vs get_human_nanos() ===");
    println!();
    
    println!("45 seconds:");
    println!("  get_human(): {}", t1.get_human());
    println!("  get_human_nanos(): {}", t1.get_human_nanos());
    println!();
    
    println!("2.5 minutes:");
    println!("  get_human(): {}", t2.get_human());
    println!("  get_human_nanos(): {}", t2.get_human_nanos());
    println!();
    
    println!("1 hour:");
    println!("  get_human(): {}", t3.get_human());
    println!("  get_human_nanos(): {}", t3.get_human_nanos());
    println!();
    
    println!("1500 milliseconds:");
    println!("  get_human(): {}", t4.get_human());
    println!("  get_human_nanos(): {}", t4.get_human_nanos());
    println!();
    
    println!("90 seconds:");
    println!("  get_human(): {}", t5.get_human());
    println!("  get_human_nanos(): {}", t5.get_human_nanos());
    println!();
    
    println!("1500 microseconds (1.5ms):");
    println!("  get_human(): {}", t6.get_human());
    println!("  get_human_nanos(): {}", t6.get_human_nanos());
    println!();
    
    println!("1500000 nanoseconds (1.5ms):");
    println!("  get_human(): {}", t7.get_human());
    println!("  get_human_nanos(): {}", t7.get_human_nanos());
    println!();
    
    println!("1s + 500ms + 250μs + 125ns:");
    println!("  get_human(): {}", t8.get_human());
    println!("  get_human_nanos(): {}", t8.get_human_nanos());
    println!();

    // Test negative duration
    let t_neg = Time::new::<second>(-30.5);
    println!("-30.5 seconds:");
    println!("  get_human(): {}", t_neg.get_human());
    println!("  get_human_nanos(): {}", t_neg.get_human_nanos());
}
