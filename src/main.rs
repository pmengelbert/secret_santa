use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Sibling {
    Meghan = 0,
    Greg = 1,
    Michael = 2,
    Kirsten = 3,
    Mark = 4,
    Lina = 5,
    Peter = 6,
    Claire = 7,
    John = 8,
    Colin = 9,
}

fn main() {
    use Sibling::*;

    let givers = vec![
        Meghan, Greg, Michael, Kirsten, Mark, Lina, Peter, Claire, John, Colin,
    ];

    let results = include_bytes!("../out.bin");

    let random_index = thread_rng().gen_range(0..400_192) * 10;

    for (i, &b) in results[random_index..random_index + 10].iter().enumerate() {
        println!("{:?} gives to {:?}", givers[i], to_sibling(b));
    }
}

fn to_sibling(n: u8) -> Sibling {
    use Sibling::*;
    match n {
        0 => Meghan,
        1 => Greg,
        2 => Michael,
        3 => Kirsten,
        4 => Mark,
        5 => Lina,
        6 => Peter,
        7 => Claire,
        8 => John,
        9 => Colin,
        _ => unreachable!(),
    }
}
