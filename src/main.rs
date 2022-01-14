#![allow(unused_variables, dead_code, unused_imports)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

    let mut givers = VecDeque::from_iter(
        [Giver(Meghan), Giver(Michael), Giver(Greg), Giver(Kirsten)].into_iter(),
    );

    let mut receivers = VecDeque::from_iter(
        [
            Receiver(Meghan),
            Receiver(Michael),
            Receiver(Greg),
            Receiver(Kirsten),
        ]
        .into_iter(),
    );

    let mut verboten = HashMap::new();
    verboten.insert(Meghan, Meghan);
    verboten.insert(Greg, Greg);
    verboten.insert(Meghan, Greg);
    verboten.insert(Greg, Meghan);

    verboten.insert(Michael, Michael);
    verboten.insert(Kirsten, Kirsten);
    verboten.insert(Michael, Kirsten);
    verboten.insert(Kirsten, Michael);

    let mut builder = vec![];
    let mut output = vec![];

    let first = givers.pop_front().unwrap();
    r(
        &mut givers,
        &mut receivers,
        &mut builder,
        &mut output,
        &verboten,
        (first, None),
    );

    // take each giver in turn
    // meghan
}

fn r(
    givers: &mut VecDeque<Participant>,
    receivers: &mut VecDeque<Participant>,
    builder: &mut Vec<(Participant, Participant)>,
    output: &mut Vec<Vec<(Participant, Participant)>>,
    verboten: &HashMap<Sibling, Sibling>,
    mut current: (Participant, Option<Participant>),
) {
    use Participant::*;

    let participant = match current {
        (g, None) => g,
        (Giver(g), Some(Receiver(rcv))) => {
            if let Some(&recv) = verboten.get(&g) {
                if recv == rcv {
                    return;
                } else {
                    builder.push((Giver(g), Receiver(rcv)));
                    if builder.len() == 2 {
                        output.push(builder.clone());
                        return;
                    }
                    if let Some(next_giver) = givers.pop_front() {
                        r(
                            givers,
                            receivers,
                            builder,
                            output,
                            verboten,
                            (next_giver, None),
                        );
                    } else {
                        return;
                    }
                }
            }
            Receiver(rcv)
        }
        _ => unreachable!(),
    };

    // match participant {
    //     Giver(g) => {
    //         let mut frozen = receivers.clone();
    //         while let Some(r) = frozen.pop_front() {}
    //     }
    //     Receiver(r) => {}
    // }
}
