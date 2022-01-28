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
        Meghan as u8,
        Greg as u8,
        Michael as u8,
        Kirsten as u8,
        Mark as u8,
        Lina as u8,
        Peter as u8,
        Claire as u8,
        John as u8,
        Colin as u8,
    ];
    let mut receivers = vec![
        Meghan as u8,
        Greg as u8,
        Michael as u8,
        Kirsten as u8,
        Mark as u8,
        Lina as u8,
        Peter as u8,
        Claire as u8,
        John as u8,
        Colin as u8,
    ];

    let verboten = [
        Greg as u8,
        Meghan as u8,
        Kirsten as u8,
        Michael as u8,
        Lina as u8,
        Mark as u8,
        Claire as u8,
        Peter as u8,
        Colin as u8,
        John as u8,
    ];

    let mut collection = vec![];
    backtrack(0, &verboten, &givers, &mut receivers, &mut collection);

    std::fs::write("out.bin", &collection);
}

fn backtrack(
    first: usize,
    verboten: &[u8],
    givers: &[u8],
    receivers: &mut Vec<u8>,
    collection: &mut Vec<u8>,
) {
    if first == receivers.len() {
        for (i, &r) in receivers.iter().enumerate() {
            if givers[i] == r {
                return;
            } else if let Some(&bad) = verboten.get(givers[i] as usize) {
                if r == bad {
                    return;
                }
            }
        }

        collection.extend_from_slice(&receivers);
        return;
    }

    for i in first..receivers.len() {
        match verboten.get(givers[i] as usize) {
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
