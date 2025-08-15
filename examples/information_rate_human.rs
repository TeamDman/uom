use uom::si::f64::{Information, Time, InformationRate};
use uom::si::information::{byte, kilobyte, megabyte, gigabyte, kibibyte, mebibyte, gibibyte};
use uom::si::information_rate::{byte_per_second, kilobyte_per_second, megabyte_per_second, kibibyte_per_second, mebibyte_per_second};
use uom::si::time::second;

fn main() {
    println!("=== InformationRate Human-Readable Formatting ===");
    println!();
    
    // Test various information rates
    let rates = [
        ("512 B/s", InformationRate::new::<byte_per_second>(512.0)),
        ("1.5 kB/s", InformationRate::new::<kilobyte_per_second>(1.5)),
        ("100 MB/s", InformationRate::new::<megabyte_per_second>(100.0)),
        ("2.5 KiB/s", InformationRate::new::<kibibyte_per_second>(2.5)),
        ("50 MiB/s", InformationRate::new::<mebibyte_per_second>(50.0)),
    ];

    println!("| Input | Decimal (SI) | Binary (IEC) | Raw B/s |");
    println!("|----|----|----|----| ");
    
    for (description, rate) in rates.iter() {
        let bps = rate.get::<byte_per_second>() as u64;
        println!("| {} | {} | {} | {} B/s |", 
                description,
                rate.get_human(),
                rate.get_human_binary(),
                bps);
    }
    
    println!();
    println!("=== Calculated Rates from Transfer Scenarios ===");
    
    // Scenario 1: Download
    let file_size = Information::new::<megabyte>(500.0);
    let download_time = Time::new::<second>(120.0); // 2 minutes
    let download_rate: InformationRate = (file_size / download_time).into();
    
    println!("📥 Download Scenario:");
    println!("  File: {}", file_size.get_human());
    println!("  Time: {}", download_time.get_human());
    println!("  Rate: {} ({})", download_rate.get_human(), download_rate.get_human_binary());
    println!();
    
    // Scenario 2: Network backup
    let backup_size = Information::new::<gibibyte>(10.0);
    let backup_time = Time::new::<second>(3600.0); // 1 hour
    let backup_rate: InformationRate = (backup_size / backup_time).into();
    
    println!("💾 Network Backup:");
    println!("  Size: {} ({})", backup_size.get_human(), backup_size.get_human_binary());
    println!("  Time: {}", backup_time.get_human());
    println!("  Rate: {} ({})", backup_rate.get_human(), backup_rate.get_human_binary());
    println!();
    
    // Scenario 3: USB transfer
    let usb_size = Information::new::<gigabyte>(4.7); // DVD size
    let usb_time = Time::new::<second>(285.0); // ~4.75 minutes
    let usb_rate: InformationRate = (usb_size / usb_time).into();
    
    println!("💽 USB Transfer:");
    println!("  Size: {} ({})", usb_size.get_human(), usb_size.get_human_binary());
    println!("  Time: {}", usb_time.get_human());
    println!("  Rate: {} ({})", usb_rate.get_human(), usb_rate.get_human_binary());
    println!();
    
    println!("=== Edge Cases ===");
    let zero_rate = InformationRate::new::<byte_per_second>(0.0);
    let tiny_rate = InformationRate::new::<byte_per_second>(1.0);
    let huge_rate = InformationRate::new::<byte_per_second>(999_999_999_999.0);
    
    println!("Zero rate: {} / {}", zero_rate.get_human(), zero_rate.get_human_binary());
    println!("1 B/s: {} / {}", tiny_rate.get_human(), tiny_rate.get_human_binary());
    println!("~1 TB/s: {} / {}", huge_rate.get_human(), huge_rate.get_human_binary());
    println!();
    
    println!("✨ InformationRate Formatting Benefits:");
    println!("• Automatic unit selection (B/s → kB/s → MB/s → GB/s → ...)");
    println!("• Decimal vs Binary prefix options for different contexts");
    println!("• Consistent '/s' suffix for clear rate indication");
    println!("• Seamless integration with Information and Time quantities");
}
