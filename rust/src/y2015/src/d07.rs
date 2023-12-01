use prelude::*;

pub struct Day07;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Var(String);

#[derive(Debug, Clone)]
pub enum Operand {
    Var(Var),
    Literal(u16),
}

#[derive(Debug, Clone)]
pub enum Operation {
    Assign(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, u8),
    RShift(Operand, u8),
}

#[derive(Debug, Clone)]
pub struct Instruction(Operation, Var);

pub trait CapturesExtensions {
    fn next_var(&mut self) -> Var;
    fn next_operand(&mut self) -> Operand;
}

impl CapturesExtensions for inputs::Captures<'_> {
    fn next_var(&mut self) -> Var {
        Var(self.next_string())
    }

    #[allow(clippy::option_if_let_else)]
    fn next_operand(&mut self) -> Operand {
        let val = self.next_string();

        match val.parse::<u16>() {
            Ok(literal) => Operand::Literal(literal),
            Err(_) => Operand::Var(Var(val)),
        }
    }
}

impl AocSolution for Day07 {
    type Input = Vec<Instruction>;
    fn process_input(input: &str) -> Self::Input {
        inputs::regexes(
            input,
            vec![
                (
                    r#"(\w+) -> (\w+)"#,
                    Box::new(move |mut c| {
                        Instruction(Operation::Assign(c.next_operand()), c.next_var())
                    }),
                ),
                (
                    r#"NOT (\w+) -> (\w+)"#,
                    Box::new(move |mut c| {
                        Instruction(Operation::Not(c.next_operand()), Var(c.next_string()))
                    }),
                ),
                (
                    r#"(\w+) AND (\w+) -> (\w+)"#,
                    Box::new(move |mut c| {
                        Instruction(
                            Operation::And(c.next_operand(), c.next_operand()),
                            c.next_var(),
                        )
                    }),
                ),
                (
                    r#"(\w+) OR (\w+) -> (\w+)"#,
                    Box::new(move |mut c| {
                        Instruction(
                            Operation::Or(c.next_operand(), c.next_operand()),
                            c.next_var(),
                        )
                    }),
                ),
                (
                    r#"(\w+) LSHIFT (\d+) -> (\w+)"#,
                    Box::new(move |mut c| {
                        Instruction(
                            Operation::LShift(c.next_operand(), c.next_u8()),
                            Var(c.next_string()),
                        )
                    }),
                ),
                (
                    r#"(\w+) RSHIFT (\d+) -> (\w+)"#,
                    Box::new(move |mut c| {
                        Instruction(
                            Operation::RShift(c.next_operand(), c.next_u8()),
                            Var(c.next_string()),
                        )
                    }),
                ),
            ],
        )
    }

    const PART1_SOLUTION: Solution = solution(3176);
    fn part1(input: &Self::Input) -> impl ToSolution {
        run_instructions(input)[&Var("a".to_owned())]
    }

    const PART2_SOLUTION: Solution = solution(14710);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let a_state = run_instructions(input)[&Var("a".to_owned())];

        let mut instructions = input.clone();

        let var_b = Var("b".to_owned());

        // Replace anything assigning to 'b' with a literal assignment
        instructions = instructions
            .into_iter()
            .map(|i| match i {
                Instruction(_, var) if var == var_b => {
                    Instruction(Operation::Assign(Operand::Literal(a_state)), var_b.clone())
                }
                _ => i,
            })
            .collect();

        run_instructions(&instructions)[&Var("a".to_owned())]
    }
}

fn run_instructions(instructions: &[Instruction]) -> HashMap<Var, u16> {
    let mut vars: HashMap<Var, u16> = HashMap::new();
    let mut instructions = instructions.to_vec();

    while !instructions.is_empty() {
        instructions.retain(|i| run_instruction(i, &mut vars).is_err());
    }

    vars
}

fn run_instruction(instruction: &Instruction, vars: &mut HashMap<Var, u16>) -> Result<(), ()> {
    let Instruction(operation, dst) = instruction;
    *vars.entry(dst.clone()).or_insert(0) = match operation {
        Operation::Assign(src) => get(vars, src)?,
        Operation::Not(src) => !get(vars, src)?,
        Operation::And(s1, s2) => get(vars, s1)? & get(vars, s2)?,
        Operation::Or(s1, s2) => get(vars, s1)? | get(vars, s2)?,
        Operation::LShift(src, shift) => get(vars, src)? << shift,
        Operation::RShift(src, shift) => get(vars, src)? >> shift,
    };

    Ok(())
}

fn get(vars: &mut HashMap<Var, u16>, src: &Operand) -> Result<u16, ()> {
    match src {
        Operand::Var(s) => vars.get(s).ok_or(()).map(|r| *r),
        Operand::Literal(lit) => Ok(*lit),
    }
}
