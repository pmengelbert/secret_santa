use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize)]
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

    let givers = vec![
        Meghan, Greg, Michael, Kirsten, Mark, Lina, Peter, Claire, John, Colin,
    ];
    let mut receivers = vec![
        Meghan, Greg, Michael, Kirsten, Mark, Lina, Peter, Claire, John, Colin,
    ];

    let mut verboten = HashMap::new();
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

    let mut collection = vec![];
    backtrack(0, &verboten, &givers, &mut receivers, &mut collection);

    let mut json_results = vec![];
    for i in 0..collection.len() {
        let mut result = vec![];
        for j in 0..givers.len() {
            result.push(json!({
                "giver": givers[j],
                "recvr": collection[i][j],
            }));
        }

        json_results.push(json!({
            "num": i + 1,
            "result": result
        }));
    }

    println!("{}", serde_json::Value::Array(json_results).to_string());
}

fn backtrack(
    first: usize,
    verboten: &HashMap<Sibling, Sibling>,
    givers: &Vec<Sibling>,
    receivers: &mut Vec<Sibling>,
    collection: &mut Vec<Vec<Sibling>>,
) {
    if first == receivers.len() {
        for (i, &r) in receivers.iter().enumerate() {
            if givers[i] == r {
                return;
            } else if let Some(&bad) = verboten.get(&givers[i]) {
                if r == bad {
                    return;
                }
            }
        }

        collection.push(receivers.clone());
        return;
    }

    for i in first..receivers.len() {
        match verboten.get(&givers[i]) {
            Some(&receiver) if receiver == receivers[i] => {
                continue;
            }
            None => unreachable!(),
            _ => {
                receivers.swap(first, i);
                backtrack(first + 1, verboten, givers, receivers, collection);
                receivers.swap(first, i);
            }
        }
    }
}
