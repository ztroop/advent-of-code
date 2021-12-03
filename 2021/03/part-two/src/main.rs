trait BinaryPredicate<T> {
    fn pred(l: &T, r: &T) -> bool;
}

struct EqPred;
impl<T: Eq> BinaryPredicate<T> for EqPred {
    fn pred(l: &T, r: &T) -> bool {
        *l == *r
    }
}

struct NeqPred;
impl<T: Eq> BinaryPredicate<T> for NeqPred {
    fn pred(l: &T, r: &T) -> bool {
        *l != *r
    }
}

fn predicate<P: BinaryPredicate<bool>>(l: bool, r: bool) -> bool {
    P::pred(&l, &r)
}

fn get_life_support_rating(numbers: Vec<u32>) -> u32 {
    fn get(numbers: Vec<u32>, predicate: impl Fn(bool, bool) -> bool) -> u32 {
        (0..12)
            .rev()
            .scan(numbers, |v, i| {
                let b = v.iter().filter(|n| *n & 1 << i > 0).count() >= (v.len() + 1) / 2;
                v.retain(|n| predicate(*n & 1 << i > 0, b));
                v.first().copied()
            })
            .last()
            .unwrap()
    }

    let oxygen_generator_rating: u32 = get(numbers.clone(), predicate::<NeqPred>);
    let co2_scrubber_rating: u32 = get(numbers, predicate::<EqPred>);

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let numbers = include_str!("../input")
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    println!("The answer is: {}", get_life_support_rating(numbers))
}
