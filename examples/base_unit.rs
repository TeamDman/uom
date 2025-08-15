#[macro_use]
extern crate uom;

use crate::ours::*;
use humansize::DECIMAL;
use uom::si::frequency::hertz;
use uom::si::information::byte;
use uom::si::information_rate::byte_per_second;
use uom::si::ratio::ratio;
use uom::si::time::millisecond;

mod ours {
    ISQ!(
        uom::si,
        u64,
        (meter, gram, millisecond, ampere, kelvin, mole, candela)
    );
}

fn main() {
    let total_size_bytes = 10240;
    dbg!(&total_size_bytes);
    let total_size = Information::new::<byte>(total_size_bytes);
    dbg!(&total_size);

    let entry_size_bytes = 1024;
    dbg!(&entry_size_bytes);
    let entry_size = Information::new::<byte>(entry_size_bytes);
    dbg!(&entry_size);

    let processed_bytes = 2048;
    dbg!(&processed_bytes);
    let processed_size = Information::new::<byte>(processed_bytes);
    dbg!(&processed_size);

    let elapsed_time_millis = 100;
    dbg!(&elapsed_time_millis);
    let elapsed_time = uom::si::u64::Time::new::<millisecond>(elapsed_time_millis);
    dbg!(&elapsed_time);
    assert!(elapsed_time.value == 0); // it stores in seconds so this is zero
    let elapsed_time = Time::new::<millisecond>(elapsed_time_millis);
    dbg!(&elapsed_time);
    assert!(elapsed_time.value > 0); // ours stores in milliseconds so this is ok

    let processing_throughput: ours::InformationRate = (total_size / elapsed_time).into();
    dbg!(&processing_throughput);
    let processing_throughput_bps = processing_throughput.get::<byte_per_second>();
    dbg!(&processing_throughput_bps);

    let remaining_size = total_size - processed_size;
    dbg!(&remaining_size);
    let remaining_bytes = remaining_size.get::<byte>();
    dbg!(&remaining_bytes);

    let remaining_time = remaining_size / processing_throughput;
    dbg!(&remaining_time);
    let remaining_time_millis = remaining_time.get::<millisecond>();
    dbg!(&remaining_time_millis);

    let processed_items = processed_size / entry_size;
    dbg!(&processed_items);
    let _processed_items_count = processed_items.get::<ratio>();
    let processed_items_count = processed_items.value;
    dbg!(&processed_items_count);

    let process_item_rate = processed_items / elapsed_time;
    dbg!(&process_item_rate);
    let item_per_second = process_item_rate.get::<hertz>();
    dbg!(&item_per_second);

    let total_items = total_size / entry_size;
    dbg!(&total_items);
    let total_items_count = total_items.get::<ratio>();
    dbg!(&total_items_count);

    println!(
        "Processed {processed_items_count}/{total_items_count} ({item_per_second}) {bps}",
        bps = humansize::format_size(processing_throughput_bps, DECIMAL)
    );
}
