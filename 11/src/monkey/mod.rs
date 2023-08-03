// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<i64>,
    operation: fn(i64) -> i64,
    throw: fn(i64) -> usize,
    pub business: usize,
}

impl Monkey {
    pub fn init(
        items: Vec<i64>,
        operation: fn(i64) -> i64,
        throw: fn(i64) -> usize,
    ) -> Self {
        Self {
            items,
            operation,
            throw,
            business: 0,
        }
    }

    pub fn action(&mut self) -> Vec<(i64, usize)> {
        self.items
            .drain(..)
            .map(|i| {
                self.business += 1;
                let worry = (self.operation)(i) / 3;
                (worry, (self.throw)(worry))
            })
            .collect()
    }

    pub fn round(monkeys: &mut [Monkey]) {
        for index in 0..monkeys.len() {
            let items = monkeys[index].action();
            for (worry, target) in items {
                monkeys[target].items.push(worry);
            }
        }
    }

    pub fn monkey_business(monkeys: &mut [Monkey]) -> usize {
        let mut values = monkeys.iter().map(|m| m.business).collect::<Vec<_>>();
        values.sort_by_key(|v| usize::MAX - v);

        debug_assert!(monkeys.len() > 1);
        values[0] * values[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print() {
        let monkey = Monkey::init(
            vec![79, 98],
            |v: i64| v * 19,
            |v: i64| if (v % 23) == 0 { 2 } else { 3 },
        );

        println!("{:?}", monkey);
        println!("{:?}", (monkey.operation)(1));
        println!("{:?}", (monkey.throw)(24));
        println!("{:?}", (monkey.throw)(23));
    }
}
