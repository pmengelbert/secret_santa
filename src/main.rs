use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Sibling {
    Alice,
    Bob,
    Charlie,
    Dan,
    Ethyl,
    Fred,
}

const SIBLINGS: [Sibling; 6] = [
    Sibling::Alice,
    Sibling::Bob,
    Sibling::Charlie,
    Sibling::Dan,
    Sibling::Ethyl,
    Sibling::Fred,
];

fn main() {
    use Sibling::*;

    let mut givers: HashSet<Sibling> = HashSet::new();
    let mut receivers: HashSet<Sibling> = HashSet::new();

    let mut final_results: HashSet<Vec<(Sibling, Sibling)>> = HashSet::new();
    let mut result: Vec<Sibling> = Vec::new();
    let mut other_result: HashSet<(Sibling, Sibling)> = HashSet::new();

    let mut verboten: HashMap<Sibling, Sibling> = HashMap::new();
    verboten.insert(Alice, Bob);
    verboten.insert(Bob, Alice);
    verboten.insert(Charlie, Dan);
    verboten.insert(Dan, Charlie);
    verboten.insert(Ethyl, Fred);
    verboten.insert(Fred, Ethyl);

    r(
        &mut final_results,
        &mut result,
        &mut other_result,
        0,
        &mut givers,
        &mut receivers,
        &verboten,
    );

    let mut count = 0;
    for result in final_results {
        count += 1;
        for (g, r) in result {
            println!("{:?} gives to {:?}", g, r);
        }
        println!("");
    }

    println!("total of {} results", count);
}

fn r(
    final_result: &mut HashSet<Vec<(Sibling, Sibling)>>,
    result: &mut Vec<Sibling>,
    other_result: &mut HashSet<(Sibling, Sibling)>,
    depth: usize,
    givers: &mut HashSet<Sibling>,
    receivers: &mut HashSet<Sibling>,
    verboten: &HashMap<Sibling, Sibling>,
) {
    if depth == SIBLINGS.len() * 2 {
        let mut new_result = Vec::new();
        for r in other_result.iter() {
            new_result.push(*r);
        }
        new_result.sort();
        final_result.insert(new_result);
        return;
    }

    // even
    if depth & 1 == 0 {
        for sibling in &SIBLINGS {
            if !givers.contains(sibling) {
                givers.insert(*sibling);
                result.push(*sibling);

                r(
                    final_result,
                    result,
                    other_result,
                    depth + 1,
                    givers,
                    receivers,
                    verboten,
                );
                givers.remove(sibling);
                result.pop();
            }
        }
    } else {
        for sibling in &SIBLINGS {
            let last = result[result.len() - 1];
            if last == *sibling || verboten.get(&last) == Some(sibling) {
                continue;
            }

            if !receivers.contains(sibling) {
                receivers.insert(*sibling);
                result.push(*sibling);
                other_result.insert((last, *sibling));

                r(
                    final_result,
                    result,
                    other_result,
                    depth + 1,
                    givers,
                    receivers,
                    verboten,
                );
                receivers.remove(sibling);
                other_result.remove(&(last, *sibling));
                result.pop();
            }
        }
    }
}
