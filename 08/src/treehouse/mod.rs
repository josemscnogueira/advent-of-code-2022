use std::fs::File;
use std::io::{BufRead, BufReader};

use grid::Grid;
use itertools::Itertools;

#[derive(Debug)]
pub struct Treehouse {
    pub forest: Grid<u8>,
    pub visible: Grid<Option<bool>>,
    pub scenic: Grid<usize>,
}

impl Treehouse {
    pub fn parse(filepath: &str) -> Self {
        // Open a file and read from it
        let file = File::open(filepath)
            .expect(&format!("Error while opening file {}", filepath));
        let reader = BufReader::new(file);
        let mut forest = Grid::new(0, 0);

        reader.lines().for_each(|l| {
            forest.push_row(
                l.unwrap()
                    .chars()
                    .into_iter()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec(),
            );
        });

        let mut result = Self {
            scenic: Grid::init(forest.rows(), forest.cols(), 0),
            visible: Grid::init(forest.rows(), forest.cols(), None),
            forest,
        };

        result.check_visible();
        result.check_sceninc();
        result
    }

    pub fn count_visible(&self) -> usize {
        self.visible.iter().filter(|v| **v == Some(true)).count()
    }


    fn initialize_edges(&mut self) {
        self.visible.iter_row_mut(0).for_each(|v| *v = Some(true));
        self.visible.iter_col_mut(0).for_each(|v| *v = Some(true));
        self.visible
            .iter_row_mut(self.forest.rows() - 1)
            .for_each(|v| *v = Some(true));
        self.visible
            .iter_col_mut(self.forest.cols() - 1)
            .for_each(|v| *v = Some(true));
    }

    fn check_sceninc(&mut self) {
        let rows = self.forest.rows();
        let cols = self.forest.cols();
        debug_assert_eq!(rows, self.scenic.rows());
        debug_assert_eq!(cols, self.scenic.cols());

        for (r, c) in
            (0..(rows - 1)).into_iter().cartesian_product(0..(cols - 1))
        {
            self.scenic[r][c] = 1;
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut alpha = 1;
                loop {
                    let rc = r.checked_add_signed(dr * alpha);
                    let cc = c.checked_add_signed(dc * alpha);

                    match (rc, cc) {
                        (Some(rc), Some(cc)) if rc < rows && cc < cols => {
                            if self.forest[r][c] > self.forest[rc][cc] {
                                alpha += 1;
                            } else {
                                break;
                            }
                        }
                        (_, _) => {
                            alpha -= 1;
                            break;
                        }
                    }
                }
                self.scenic[r][c] *= alpha as usize;
            }
        }
    }

    fn check_visible(&mut self) {
        let rows = self.forest.rows();
        let cols = self.forest.cols();
        debug_assert_eq!(rows, self.visible.rows());
        debug_assert_eq!(cols, self.visible.cols());
        self.initialize_edges();

        let mut not_finished = true;
        while not_finished {
            not_finished = false;

            for (r, c) in
                (1..(rows - 1)).into_iter().cartesian_product(1..(cols - 1))
            {
                if self.visible[r][c].is_none() {
                    let mut counter = 0;

                    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (mut rc, mut cc) = (Some(r), Some(c));

                        let is_taller = loop {
                            rc = rc.unwrap().checked_add_signed(dr);
                            cc = cc.unwrap().checked_add_signed(dc);

                            match (rc, cc) {
                                (Some(rc), Some(cc))
                                    if rc < rows && cc < cols =>
                                {
                                    if self.forest[r][c] <= self.forest[rc][cc]
                                    {
                                        break false;
                                    }
                                }
                                (_, _) => break true,
                            }
                        };

                        if is_taller {
                            self.visible[r][c] = Some(true);
                            break;
                        } else {
                            counter += 1;
                        }
                    }

                    // Tree visibility is invalidated by the vinicity
                    if counter == 4 {
                        self.visible[r][c] = Some(false);
                    // If it was not possible to evaluate the current state,
                    // don't let the while loop finish
                    } else if self.visible[r][c].is_none() {
                        not_finished = true;
                    }
                }
            }
        }
    }
}
