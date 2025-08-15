use uom::si::f64::{Time, Information, InformationRate};
use uom::si::time::{second, minute};
use uom::si::information::{byte, megabyte, gigabyte};
use uom::si::information_rate::byte_per_second;

fn main() {
    println!("=== Combined Human-Readable UOM Demo ===");
    println!();
    
    // Simulate a file transfer scenario
    let file_size = Information::new::<megabyte>(250.0);
    let transfer_time = Time::new::<minute>(2.5);
    let transfer_rate: InformationRate = (file_size / transfer_time).into();
    
    println!("📁 File Transfer Scenario");
    println!("File size:     {} ({})", file_size.get_human(), file_size.get_human_binary());
    println!("Transfer time: {} ({})", transfer_time.get_human(), transfer_time.get_human_nanos());
    println!("Transfer rate: {} ({})", transfer_rate.get_human(), transfer_rate.get_human_binary());
    println!();
    
    // Simulate a backup scenario
    let backup_size = Information::new::<gigabyte>(1.5);
    let backup_time = Time::new::<second>(450.0);
    let backup_rate: InformationRate = (backup_size / backup_time).into();
    
    println!("💾 Backup Scenario");
    println!("Backup size:   {} ({})", backup_size.get_human(), backup_size.get_human_binary());
    println!("Backup time:   {} ({})", backup_time.get_human(), backup_time.get_human_nanos());
    println!("Backup rate:   {} ({})", backup_rate.get_human(), backup_rate.get_human_binary());
    println!();
    
    // Simulate a streaming scenario
    let stream_rate = InformationRate::new::<byte_per_second>(2_500_000.0); // 2.5 MB/s
    let stream_duration = Time::new::<minute>(120.0); // 2 hours
    let total_streamed: Information = (stream_rate * stream_duration).into();
    
    println!("📺 Streaming Scenario");
    println!("Stream rate:   {} ({})", stream_rate.get_human(), stream_rate.get_human_binary());
    println!("Duration:      {} ({})", stream_duration.get_human(), stream_duration.get_human_nanos());
    println!("Total data:    {} ({})", total_streamed.get_human(), total_streamed.get_human_binary());
    println!();
    
    // Show precision differences
    let precise_size = Information::new::<byte>(1048576.0 + 512.0); // 1 MiB + 512 bytes
    let precise_time = Time::new::<second>(1.234567890);
    let precise_rate: InformationRate = (precise_size / precise_time).into();
    
    println!("🔍 Precision Comparison");
    println!("Size:          {} vs {}", precise_size.get_human(), precise_size.get_human_binary());
    println!("Time:          {} vs {}", precise_time.get_human(), precise_time.get_human_nanos());
    println!("Rate:          {} vs {}", precise_rate.get_human(), precise_rate.get_human_binary());
    println!();
    
    println!("✨ Human-Readable UOM Benefits:");
    println!("• Time: Clean output vs full precision (get_human vs get_human_nanos)");
    println!("• Information: Decimal vs Binary prefixes (get_human vs get_human_binary)");
    println!("• InformationRate: Consistent rate formatting with /s suffix");
    println!("• Automatic unit selection for optimal readability");
    println!("• Seamless interoperability between related quantity types");
    println!("• Feature-gated to keep core library lightweight");
}
