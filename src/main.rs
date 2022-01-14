#![allow(unused_variables, dead_code, unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Participant {
    Giver(Sibling),
    Receiver(Sibling),
}

fn main() {
    use Participant::*;
    use Sibling::*;

    let givers = vec![
        Giver(Meghan),
        Giver(Michael),
        Giver(Mark),
        Giver(Peter),
        Giver(John),
        Giver(Greg),
        Giver(Kirsten),
        Giver(Lina),
        Giver(Claire),
        Giver(Colin),
    ];

    let receivers = vec![
        Receiver(Meghan),
        Receiver(Michael),
        Receiver(Mark),
        Receiver(Peter),
        Receiver(John),
        Receiver(Greg),
        Receiver(Kirsten),
        Receiver(Lina),
        Receiver(Claire),
        Receiver(Colin),
    ];

    let participants = vec![
        Giver(Meghan),
        Giver(Michael),
        Giver(Mark),
        Giver(Peter),
        Giver(John),
        Giver(Greg),
        Giver(Kirsten),
        Giver(Lina),
        Giver(Claire),
        Giver(Colin),
        Receiver(Meghan),
        Receiver(Michael),
        Receiver(Mark),
        Receiver(Peter),
        Receiver(John),
        Receiver(Greg),
        Receiver(Kirsten),
        Receiver(Lina),
        Receiver(Claire),
        Receiver(Colin),
    ];

    // take each giver in turn
}
