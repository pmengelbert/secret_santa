use rand::Rng;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Sibling {
    Meghan,
    Greg,
    Michael,
    Kirsten,
    Mark,
    Lina,
    Peter,
    Claire,
    John,
    Colin,
}

fn main() {
    use Sibling::*;
    let siblings = [
        Meghan, Greg, Michael, Kirsten, Mark, Lina, Peter, Claire, John, Colin,
    ];

    // nobody should get their spouse
    let mut verboten: HashMap<Sibling, Sibling> = HashMap::new();
    verboten.insert(Meghan, Greg);
    verboten.insert(Greg, Meghan);
    verboten.insert(Michael, Kirsten);
    verboten.insert(Kirsten, Michael);
    verboten.insert(Mark, Lina);
    verboten.insert(Lina, Mark);
    verboten.insert(Peter, Claire);
    verboten.insert(Claire, Peter);
    verboten.insert(John, Colin);
    verboten.insert(Colin, John);

    // nobody should get assigned as a receiver more than once, so
    // we need to keep track of who has been "used up" already
    let mut taken = BTreeSet::new();

    let mut matchups: HashMap<Sibling, Sibling> = HashMap::new();
    for giving in siblings {
        let mut index = rand::thread_rng().gen_range(0..10);
        let mut receiving = siblings[index];
        while taken.contains(&receiving)
            || &receiving == verboten.get(&giving).unwrap()
            || giving == receiving
        {
            index = rand::thread_rng().gen_range(0..10);
            receiving = siblings[index];
        }
        matchups.insert(giving, receiving);
        taken.insert(receiving);
    }

    for k in matchups.keys() {
        println!(
            "{:?}\t\twill give a gift to\t\t{:?}",
            k,
            *matchups.get(&k).unwrap()
        );
    }
}