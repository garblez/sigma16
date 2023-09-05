mod processor;
use processor::Processor;

fn main() {
    let cpu = Processor::new();
    println!("{:?}", cpu.state);
}
