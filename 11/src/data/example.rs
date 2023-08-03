// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
//
// Monkey 1:
//   Starting items: 54, 65, 75, 74
//   Operation: new = old + 6
//   Test: divisible by 19
//     If true: throw to monkey 2
//     If false: throw to monkey 0
//
// Monkey 2:
//   Starting items: 79, 60, 97
//   Operation: new = old * old
//   Test: divisible by 13
//     If true: throw to monkey 1
//     If false: throw to monkey 3
//
// Monkey 3:
//   Starting items: 74
//   Operation: new = old + 3
//   Test: divisible by 17
//     If true: throw to monkey 0
//     If false: throw to monkey 1
use crate::monkey::Monkey;

pub fn create() -> Vec<Monkey> {
    vec![
        Monkey::init(
            vec![79, 98],
            |v: i64| v * 19,
            |v: i64| if (v % 23) == 0 { 2 } else { 3 },
        ),
        Monkey::init(
            vec![54, 65, 75, 74],
            |v: i64| v + 6,
            |v: i64| if (v % 19) == 0 { 2 } else { 0 },
        ),
        Monkey::init(
            vec![79, 60, 97],
            |v: i64| v * v,
            |v: i64| if (v % 13) == 0 { 1 } else { 3 },
        ),
        Monkey::init(
            vec![74],
            |v: i64| v + 3,
            |v: i64| if (v % 17) == 0 { 0 } else { 1 },
        ),
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round1() {
        let mut monkeys = create();
        Monkey::round(&mut monkeys);

        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].items.len(), 0);
        assert_eq!(monkeys[3].items.len(), 0);
    }

    #[test]
    fn round10() {
        let mut monkeys = create();
        for _ in 0..10 {
            Monkey::round(&mut monkeys);
        }

        assert_eq!(monkeys[0].items, vec![91, 16, 20, 98]);
        assert_eq!(monkeys[1].items, vec![481, 245, 22, 26, 1092, 30]);
        assert_eq!(monkeys[2].items.len(), 0);
        assert_eq!(monkeys[3].items.len(), 0);
    }

    #[test]
    fn round20() {
        let mut monkeys = create();
        for _ in 0..20 {
            Monkey::round(&mut monkeys);
        }

        assert_eq!(monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].items.len(), 0);
        assert_eq!(monkeys[3].items.len(), 0);
    }
}
