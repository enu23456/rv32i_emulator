mod rv32i;

fn main() {
    let mut my_rv32i = rv32i::CpuOfRV32I::new();
    my_rv32i.rom.write_example();
    println!("first: {:?},", my_rv32i.get_array());
    for i in 0..65 {
        print!("{:>2}th, ", i);
        my_rv32i.next();
        println!("      after EX: {:?}", my_rv32i.x);
    }
    println!("result: {:?},", my_rv32i.get_array());
    return;
}
