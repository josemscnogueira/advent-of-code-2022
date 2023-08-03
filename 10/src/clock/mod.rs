use super::instruction::Instruction;

#[derive(Debug)]
pub struct Clock {
    pub register: i64,
    pub cycle: usize,
    pub update: Option<i64>,
    pub draw: bool,
}

impl Clock {
    pub fn init() -> Self {
        Self {
            register: 1,
            cycle: 0,
            update: None,
            draw: false,
        }
    }

    pub fn apply(&mut self, instruction: Instruction) -> Option<Instruction> {
        if let Some(value) = self.update {
            self.register += value;
            self.update = None;
        }
        self.draw = self.is_being_drawn();
        self.cycle += 1;

        match instruction {
            Instruction::Noop => None,
            Instruction::Add(counter, value) => {
                if counter == 1 {
                    // Value can't be added to the register right away but clock
                    // must be ready to take a new instruction in the next apply
                    debug_assert!(self.update.is_none());
                    self.update = Some(value);
                    None
                } else {
                    Some(Instruction::Add(counter - 1, value))
                }
            }
        }
    }

    pub fn run(&mut self, instructions: &mut Vec<Instruction>, cycles: usize) {
        for _ in 0..cycles {
            if let Some(i) = instructions.pop() {
                if let Some(i) = self.apply(i) {
                    instructions.push(i);
                }
            } else {
                break; // early
            }
        }
    }

    pub fn is_being_drawn(&self) -> bool {
        let cursor = (self.cycle % 40) as i64;

        for i in (self.register - 1)..=(self.register + 1) {
            if i == cursor {
                return true;
            }
        }

        false
    }
}
