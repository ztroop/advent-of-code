mod one;
mod two;

fn main() {
    let p1_answer = one::answer();
    println!("Part one answer is: {:?}", p1_answer);

    let p2_answer = two::answer();
    println!("Part two answer is: {:?}", p2_answer);
}
