use super::motion::Motion;

#[derive(Debug, Clone)]
pub struct Bridge<const N: usize> {
    pub head: (i64, i64),
    pub nodes: [(i64, i64); N],
    pub trail: Vec<(i64, i64)>,
}

impl<const N: usize> Bridge<N> {
    pub fn new() -> Self {
        Self {
            head: (0, 0),
            nodes: [(0, 0); N],
            trail: vec![(0, 0)],
        }
    }

    pub fn apply_moveset(motion: &Vec<Motion>) -> Self {
        let mut result = Self::new();
        for m in motion {
            result.apply(*m);
        }
        result
    }

    pub fn apply(&mut self, motion: Motion) {
        match motion {
            Motion::Up(value) if value != 0 => {
                self.head.1 += 1;
                self.apply(Motion::Up(value - 1));
            }
            Motion::Down(value) if value != 0 => {
                self.head.1 -= 1;
                self.apply(Motion::Down(value - 1));
            }
            Motion::Left(value) if value != 0 => {
                self.head.0 -= 1;
                self.apply(Motion::Left(value - 1));
            }
            Motion::Right(value) if value != 0 => {
                self.head.0 += 1;
                self.apply(Motion::Right(value - 1));
            }
            _ => (),
        };
        self.tail_update();
    }

    #[allow(dead_code)]
    pub fn tail(&self) -> &(i64, i64) {
        &self.nodes[N - 1]
    }

    fn tail_update(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..N {
                let head = if i == 0 { self.head } else { self.nodes[i - 1] };
                let tail = &mut self.nodes[i];

                let dx = tail.0 - head.0;
                let dy = tail.1 - head.1;
                let d2 = dx * dx + dy * dy;

                if d2 > 2 {
                    changed = true;
                    tail.0 -= dx.signum();
                    tail.1 -= dy.signum();
                    if i == N - 1 {
                        self.trail.push(*tail);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_nothing() {
        let mut bridge = Bridge::<1>::new();

        for m in [
            Motion::Right(0),
            Motion::Left(0),
            Motion::Up(0),
            Motion::Down(0),
        ] {
            bridge.apply(m);
            assert_eq!(bridge.head, (0, 0));
            assert_eq!(*bridge.tail(), (0, 0));
        }
    }

    #[test]
    fn move_right() {
        let mut bridge = Bridge::<1>::new();

        bridge.apply(Motion::Right(1));
        assert_eq!(bridge.head, (1, 0));
        assert_eq!(*bridge.tail(), (0, 0));

        bridge.apply(Motion::Right(1));
        assert_eq!(bridge.head, (2, 0));
        assert_eq!(*bridge.tail(), (1, 0));

        bridge.apply(Motion::Right(4));
        assert_eq!(bridge.head, (6, 0));
        assert_eq!(*bridge.tail(), (5, 0));
    }

    #[test]
    fn move_left() {
        let mut bridge = Bridge::<1>::new();

        bridge.apply(Motion::Left(1));
        assert_eq!(bridge.head, (-1, 0));
        assert_eq!(*bridge.tail(), (0, 0));

        bridge.apply(Motion::Left(1));
        assert_eq!(bridge.head, (-2, 0));
        assert_eq!(*bridge.tail(), (-1, 0));

        bridge.apply(Motion::Left(4));
        assert_eq!(bridge.head, (-6, 0));
        assert_eq!(*bridge.tail(), (-5, 0));
    }

    #[test]
    fn move_up() {
        let mut bridge = Bridge::<1>::new();

        bridge.apply(Motion::Up(1));
        assert_eq!(bridge.head, (0, 1));
        assert_eq!(*bridge.tail(), (0, 0));

        bridge.apply(Motion::Up(1));
        assert_eq!(bridge.head, (0, 2));
        assert_eq!(*bridge.tail(), (0, 1));

        bridge.apply(Motion::Up(4));
        assert_eq!(bridge.head, (0, 6));
        assert_eq!(*bridge.tail(), (0, 5));
    }

    #[test]
    fn move_down() {
        let mut bridge = Bridge::<1>::new();

        bridge.apply(Motion::Down(1));
        assert_eq!(bridge.head, (0, -1));
        assert_eq!(*bridge.tail(), (0, 0));

        bridge.apply(Motion::Down(1));
        assert_eq!(bridge.head, (0, -2));
        assert_eq!(*bridge.tail(), (0, -1));

        bridge.apply(Motion::Down(4));
        assert_eq!(bridge.head, (0, -6));
        assert_eq!(*bridge.tail(), (0, -5));
    }

    #[test]
    fn move_diagonal() {
        let mut bridge = Bridge::<1>::new();

        bridge.apply(Motion::Right(1));
        bridge.apply(Motion::Up(1));
        assert_eq!(bridge.head, (1, 1));
        assert_eq!(*bridge.tail(), (0, 0));

        bridge.apply(Motion::Right(1));
        assert_eq!(bridge.head, (2, 1));
        assert_eq!(*bridge.tail(), (1, 1));
    }
}
