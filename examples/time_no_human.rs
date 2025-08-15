use uom::si::f64::Time;
use uom::si::time::second;

fn main() {
    let t = Time::new::<second>(45.0);
    
    // This should not compile without the "human" feature
    // println!("{}", t.get_human());
    
    println!("Time: {} seconds", t.get::<second>());
    println!("Note: get_human() method is only available with the 'human' feature enabled");
}
