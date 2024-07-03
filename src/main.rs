use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("一辆失控的列车在铁轨上行驶。\n在列车正行进的轨道上，有五个人被绑起来，无法动弹。列车将要碾压过他们。\n你站在改变列车轨道的操纵杆旁。如果拉动此杆，则列车将切换到另一条轨道上。\n但是，另一条轨道上也有一个人被绑着。作为执行者，请你按任意键作出决定。");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input{
        _ => ()
    }
    let mut rand = rand::thread_rng();
    match rand.gen_range(0..10).cmp(&5){
        Ordering::Equal => println!("两边都被创死了。恭喜你，达到完美结局。"),
        Ordering::Less => println!("一个人被创死了。"),
        Ordering::Greater => println!("五个人被创死了。"),
    }
    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut input).unwrap();
    match input{
        _ => ()
    }
}
