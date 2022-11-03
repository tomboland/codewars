#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

use Direction::*;

fn dir_reduc2(arr: &[Direction]) -> Vec<Direction> {
    let mut arv = arr.to_vec();
    if arv.len() < 2 {
        return arv;
    }
    let mut removal_happened = true;
    while removal_happened {
        removal_happened = false;
        for i in 0..(arv.len() - 1) {
            match (arv[i], arv[i + 1]) {
                (North, South) | (South, North) | (East, West) | (West, East) => {
                    arv.remove(i + 1);
                    arv.remove(i);
                    removal_happened = true;
                    break;
                }
                _ => continue,
            }
        }
    }
    arv
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    arr.into_iter()
        .fold(Vec::new(), |mut state, &item| match (item, state.last()) {
            (North, Some(South))
            | (South, Some(North))
            | (East, Some(West))
            | (West, Some(East)) => {
                state.pop();
                state
            }
            _ => {
                state.push(item);
                state
            }
        })
        .into_iter()
        .collect()
}

#[test]
fn basic() {
    let a = [
        Direction::North,
        Direction::South,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::North,
        Direction::West,
    ];
    assert_eq!(dir_reduc(&a), [Direction::West]);

    let a = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    assert_eq!(
        dir_reduc(&a),
        [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East
        ]
    );
    let a = [];
    assert_eq!(dir_reduc(&a), []);
}
