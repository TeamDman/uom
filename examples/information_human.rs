use uom::si::f64::Information;
use uom::si::information::{byte, kilobyte, megabyte, gigabyte, kibibyte, mebibyte, gibibyte, terabyte, tebibyte};

fn main() {
    println!("=== Information Human-Readable Formatting ===");
    println!();
    
    // Test various information sizes
    let sizes = [
        ("512 bytes", Information::new::<byte>(512.0)),
        ("1024 bytes", Information::new::<byte>(1024.0)),
        ("1536 bytes", Information::new::<byte>(1536.0)),
        ("5.5 KB", Information::new::<kilobyte>(5.5)),
        ("250 MB", Information::new::<megabyte>(250.0)),
        ("1.5 GB", Information::new::<gigabyte>(1.5)),
        ("2.25 TB", Information::new::<terabyte>(2.25)),
        ("5.5 KiB", Information::new::<kibibyte>(5.5)),
        ("250 MiB", Information::new::<mebibyte>(250.0)),
        ("1.5 GiB", Information::new::<gibibyte>(1.5)),
        ("2.25 TiB", Information::new::<tebibyte>(2.25)),
    ];

    println!("| Input | Decimal (SI) | Binary (IEC) | Raw Bytes |");
    println!("|----|----|----|----| ");
    
    for (description, size) in sizes.iter() {
        let bytes = size.get::<byte>() as u64;
        println!("| {} | {} | {} | {} bytes |", 
                description,
                size.get_human(),
                size.get_human_binary(),
                bytes);
    }
    
    println!();
    println!("=== Edge Cases ===");
    
    // Test edge cases
    let zero = Information::new::<byte>(0.0);
    let very_small = Information::new::<byte>(1.0);
    let very_large = Information::new::<byte>(9_999_999_999_999_999_999.0);
    
    println!("Zero bytes:");
    println!("  Decimal: {}", zero.get_human());
    println!("  Binary:  {}", zero.get_human_binary());
    println!();
    
    println!("1 byte:");
    println!("  Decimal: {}", very_small.get_human());
    println!("  Binary:  {}", very_small.get_human_binary());
    println!();
    
    println!("Very large (~10 EB):");
    println!("  Decimal: {}", very_large.get_human());
    println!("  Binary:  {}", very_large.get_human_binary());
    println!();
    
    println!("=== Key Differences ===");
    println!("• get_human() uses decimal prefixes (KB = 1000 bytes)");
    println!("• get_human_binary() uses binary prefixes (KiB = 1024 bytes)");
    println!("• Both handle edge cases gracefully");
}
