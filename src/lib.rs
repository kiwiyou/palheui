use std::collections::{HashMap, HashSet};

// Integer type used throughout aheui runtime.
type Integer = i32;

#[derive(Clone, Debug)]
enum Consonant {
    Halt,
    Add,
    Multiply,
    Subtract,
    Divide,
    Remainder,
    PrintDecimal,
    PrintUnicode,
    Pop,
    ScanDecimal,
    ScanUnicode,
    Push(Integer),
    Duplicate,
    Exchange,
    Select(u32),
    Move(u32),
    Compare,
    Branch,
}

#[derive(Clone, Debug)]
enum Vowel {
    Up,
    Down,
    Left,
    Right,
    UpTwo,
    DownTwo,
    LeftTwo,
    RightTwo,
    HorizontalFlip,
    VerticalFlip,
    Flip,
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Default, Debug)]
struct Syllable {
    consonant: Option<Consonant>,
    vowel: Option<Vowel>,
}

#[derive(Debug)]
struct Field {
    w: usize,
    h: usize,
}

impl Field {
    fn next_pos(&self, state: &State) -> (usize, usize) {
        assert!(state.r < self.h, "`state.r` must be in range 0..{}", self.h);
        assert!(state.c < self.w, "`state.c` must be in range 0..{}", self.w);
        match state.direction {
            Direction::Up => {
                if let Some(next) = state.r.checked_sub(state.speed) {
                    (next, state.c)
                } else {
                    (self.h - 1, state.c)
                }
            }
            Direction::Down => {
                let mut next = state.r + state.speed;
                if next >= self.h {
                    next = 0;
                }
                (next, state.c)
            }
            Direction::Left => {
                if let Some(next) = state.c.checked_sub(state.speed) {
                    (state.r, next)
                } else {
                    (state.r, self.w - 1)
                }
            }
            Direction::Right => {
                let mut next = state.c + state.speed;
                if next >= self.w {
                    next = 0;
                }
                (state.r, next)
            }
        }
    }
}

pub fn transpile(code: &str) -> String {
    let (w, h) = code
        .lines()
        .fold((0, 0), |(w, h), line| (w + line.chars().count(), h + 1));
    let mut syllables = vec![Syllable::default(); w * h];
    for (line, row) in code.lines().zip(syllables.chunks_mut(w)) {
        for (c, cell) in line.chars().zip(row) {
            let syllable = 0xAC00..=0xD7AF;
            let mut i = c as u32;
            if !syllable.contains(&i) {
                continue;
            }
            i -= 0xAC00;
            let trail = i % 28;
            i /= 28;
            let vowel = i % 21;
            i /= 21;
            let lead = i;
            cell.vowel = match vowel {
                // ㅏ
                0 => Some(Vowel::Right),
                // ㅑ
                2 => Some(Vowel::RightTwo),
                // ㅓ
                4 => Some(Vowel::Left),
                // ㅕ
                6 => Some(Vowel::LeftTwo),
                // ㅗ
                8 => Some(Vowel::Up),
                // ㅛ
                12 => Some(Vowel::UpTwo),
                // ㅜ
                13 => Some(Vowel::Down),
                // ㅠ
                17 => Some(Vowel::DownTwo),
                // ㅡ
                18 => Some(Vowel::VerticalFlip),
                // ㅢ
                19 => Some(Vowel::Flip),
                // ㅣ
                20 => Some(Vowel::HorizontalFlip),
                _ => None,
            };
            cell.consonant = match lead {
                // ㄴ
                2 => Some(Consonant::Divide),
                // ㄷ
                3 => Some(Consonant::Add),
                // ㄸ
                4 => Some(Consonant::Multiply),
                // ㄹ
                5 => Some(Consonant::Remainder),
                // ㅁ
                6 => Some(match trail {
                    // ㅇ
                    21 => Consonant::PrintDecimal,
                    // ㅎ
                    27 => Consonant::PrintUnicode,
                    _ => Consonant::Pop,
                }),
                // ㅂ
                7 => Some(match trail {
                    // None
                    0 => Consonant::Push(0),
                    // ㄱ
                    1 => Consonant::Push(2),
                    // ㄲ
                    2 => Consonant::Push(4),
                    // ㄳ
                    3 => Consonant::Push(4),
                    // ㄴ
                    4 => Consonant::Push(2),
                    // ㄵ
                    5 => Consonant::Push(5),
                    // ㄶ
                    6 => Consonant::Push(5),
                    // ㄷ
                    7 => Consonant::Push(3),
                    // ㄹ
                    8 => Consonant::Push(5),
                    // ㄺ
                    9 => Consonant::Push(7),
                    // ㄻ
                    10 => Consonant::Push(9),
                    // ㄼ
                    11 => Consonant::Push(9),
                    // ㄽ
                    12 => Consonant::Push(7),
                    // ㄾ
                    13 => Consonant::Push(9),
                    // ㄿ
                    14 => Consonant::Push(9),
                    // ㅀ
                    15 => Consonant::Push(8),
                    // ㅁ
                    16 => Consonant::Push(4),
                    // ㅂ
                    17 => Consonant::Push(4),
                    // ㅄ
                    18 => Consonant::Push(6),
                    // ㅅ
                    19 => Consonant::Push(2),
                    // ㅆ
                    20 => Consonant::Push(4),
                    // ㅇ
                    21 => Consonant::ScanDecimal,
                    // ㅈ
                    22 => Consonant::Push(3),
                    // ㅊ
                    23 => Consonant::Push(4),
                    // ㅋ
                    24 => Consonant::Push(3),
                    // ㅌ
                    25 => Consonant::Push(4),
                    // ㅍ
                    26 => Consonant::Push(4),
                    // ㅎ
                    27 => Consonant::ScanUnicode,
                    _ => unreachable!("trailing jamo out of range: {}", trail),
                }),
                // ㅃ
                8 => Some(Consonant::Duplicate),
                // ㅅ
                9 => Some(Consonant::Select(trail)),
                // ㅆ
                10 => Some(Consonant::Move(trail)),
                // ㅈ
                12 => Some(Consonant::Compare),
                // ㅊ
                14 => Some(Consonant::Branch),
                // ㅌ
                16 => Some(Consonant::Subtract),
                // ㅍ
                17 => Some(Consonant::Exchange),
                // ㅎ
                18 => Some(Consonant::Halt),
                _ => None,
            };
        }
    }
    let code = Linearizer::new(&Field { w, h }, &syllables).linearize();
    let mut output = include_str!("header.c").to_owned();
    for (i, bytecode) in code.bytecode.into_iter().enumerate() {
        use std::fmt::Write;
        if code.reference.contains(&i) {
            writeln!(output, "LN_{i}:").unwrap();
        }
        match bytecode {
            Bytecode::Nop => output.push_str("    NOP;\n"),
            Bytecode::Halt => output.push_str("    HALT;\n"),
            Bytecode::Add => output.push_str("    ADD;\n"),
            Bytecode::Multiply => output.push_str("    MULTIPLY;\n"),
            Bytecode::Subtract => output.push_str("    SUBTRACT;\n"),
            Bytecode::Divide => output.push_str("    DIVIDE;\n"),
            Bytecode::Remainder => output.push_str("    REMAINDER;\n"),
            Bytecode::PrintDecimal => output.push_str("    PRINT_DECIMAL;\n"),
            Bytecode::PrintUnicode => output.push_str("    PRINT_UNICODE;\n"),
            Bytecode::ScanDecimal => output.push_str("    SCAN_DECIMAL;\n"),
            Bytecode::ScanUnicode => output.push_str("    SCAN_UNICODE;\n"),
            Bytecode::Select(n) => writeln!(output, "    SELECT({n});").unwrap(),
            Bytecode::Compare => output.push_str("    COMPARE;\n"),
            Bytecode::JumpNotEqualZero(label) => {
                writeln!(output, "    JUMP_NOT_EQUAL_ZERO(LN_{label});").unwrap()
            }
            Bytecode::Pop0(StorageKind::Queue) => output.push_str("    QUEUE_POP0;\n"),
            Bytecode::Pop1(StorageKind::Queue) => output.push_str("    QUEUE_POP1;\n"),
            Bytecode::Push0(StorageKind::Queue) => output.push_str("    QUEUE_PUSH0;\n"),
            Bytecode::Push1(StorageKind::Queue) => output.push_str("    QUEUE_PUSH1;\n"),
            Bytecode::Push(StorageKind::Queue, v) => {
                writeln!(output, "    QUEUE_PUSH({v});").unwrap()
            }
            Bytecode::Push0To(n) => writeln!(output, "    PUSH0_TO({n});").unwrap(),
            Bytecode::Pop0(_) => output.push_str("    STACK_POP0;\n"),
            Bytecode::Pop1(_) => output.push_str("    STACK_POP1;\n"),
            Bytecode::Push0(_) => output.push_str("    STACK_PUSH0;\n"),
            Bytecode::Push1(_) => output.push_str("    STACK_PUSH1;\n"),
            Bytecode::Push(_, v) => writeln!(output, "    STACK_PUSH({v});").unwrap(),
            Bytecode::PushFront0 => output.push_str("    PUSH_FRONT_0;\n"),
            Bytecode::PushFront1 => output.push_str("    PUSH_FRONT_1;\n"),
            Bytecode::JumpSizeNotLess(n, label) => {
                writeln!(output, "    JUMP_SIZE_NOT_LESS({n}, LN_{label});").unwrap()
            }
            Bytecode::Jump(label) => writeln!(output, "    JUMP(LN_{label});").unwrap(),
        }
    }
    output.push_str(include_str!("footer.c"));
    output
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum StorageKind {
    Stack,
    Queue,
    Stream,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Bytecode {
    Nop,
    Halt,
    // set 0 as 0 + 1
    Add,
    Multiply,
    Subtract,
    Divide,
    Remainder,
    PrintDecimal,
    PrintUnicode,
    ScanDecimal,
    ScanUnicode,
    Select(u32),
    Compare,
    JumpNotEqualZero(usize),
    Pop0(StorageKind),
    Pop1(StorageKind),
    Push0(StorageKind),
    Push1(StorageKind),
    Push0To(u32),
    Push(StorageKind, Integer),
    PushFront0,
    PushFront1,
    JumpSizeNotLess(usize, usize),
    Jump(usize),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    r: usize,
    c: usize,
    direction: Direction,
    speed: usize,
    storage: StorageKind,
}

impl State {
    fn reverse_next(&self, field: &Field) -> Self {
        let mut next = Self {
            direction: match self.direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            },
            ..*self
        };
        (next.r, next.c) = field.next_pos(&next);
        next
    }
}

struct LinearCode {
    reference: HashSet<usize>,
    bytecode: Vec<Bytecode>,
}

struct Linearizer<'a> {
    field: &'a Field,
    code: &'a [Syllable],
    result: LinearCode,
    state_memo: HashMap<State, usize>,
}

impl<'a> Linearizer<'a> {
    fn new(field: &'a Field, code: &'a [Syllable]) -> Self {
        assert!(!code.is_empty(), "`code` must not be empty");
        Self {
            field,
            code,
            result: LinearCode {
                reference: HashSet::new(),
                bytecode: vec![],
            },
            state_memo: HashMap::new(),
        }
    }

    fn linearize(mut self) -> LinearCode {
        self.linearize_recursive(State {
            r: 0,
            c: 0,
            direction: Direction::Down,
            speed: 1,
            storage: StorageKind::Stack,
        });
        self.result
    }

    fn linearize_recursive(&mut self, mut state: State) {
        loop {
            let pos = state.r * self.field.w + state.c;
            let i = self.result.bytecode.len();
            match self.code[pos].vowel {
                None => {}
                Some(Vowel::Up) => {
                    state.direction = Direction::Up;
                    state.speed = 1;
                }
                Some(Vowel::Down) => {
                    state.direction = Direction::Down;
                    state.speed = 1;
                }
                Some(Vowel::Left) => {
                    state.direction = Direction::Left;
                    state.speed = 1;
                }
                Some(Vowel::Right) => {
                    state.direction = Direction::Right;
                    state.speed = 1;
                }
                Some(Vowel::UpTwo) => {
                    state.direction = Direction::Up;
                    state.speed = 2;
                }
                Some(Vowel::DownTwo) => {
                    state.direction = Direction::Down;
                    state.speed = 2;
                }
                Some(Vowel::LeftTwo) => {
                    state.direction = Direction::Left;
                    state.speed = 2;
                }
                Some(Vowel::RightTwo) => {
                    state.direction = Direction::Right;
                    state.speed = 2;
                }
                Some(Vowel::HorizontalFlip) => {
                    state.direction = match state.direction {
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                        other => other,
                    };
                }
                Some(Vowel::VerticalFlip) => {
                    state.direction = match state.direction {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                        other => other,
                    };
                }
                Some(Vowel::Flip) => {
                    state.direction = match state.direction {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                    };
                }
            }
            use std::collections::hash_map::Entry;
            match self.state_memo.entry(state.clone()) {
                Entry::Occupied(occupied) => {
                    let j = *occupied.get();
                    self.result.bytecode.push(Bytecode::Jump(j));
                    self.result.reference.insert(j);
                    break;
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(i);
                }
            }
            match self.code[pos].consonant {
                None => {}
                Some(Consonant::Halt) => {
                    self.result
                        .bytecode
                        .push(Bytecode::JumpSizeNotLess(1, i + 2));
                    self.result.reference.insert(i + 2);
                    self.result.bytecode.push(Bytecode::Push(state.storage, 0));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Halt);
                    self.state_memo.insert(state, i);
                    break;
                }
                Some(Consonant::Add) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Add);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::Subtract) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Subtract);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::Multiply) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Multiply);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::Divide) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Divide);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::Remainder) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Remainder);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::PrintDecimal) => {
                    self.require_size(&state, 1);
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::PrintDecimal);
                }
                Some(Consonant::PrintUnicode) => {
                    self.require_size(&state, 1);
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::PrintUnicode);
                }
                Some(Consonant::Pop) => {
                    self.require_size(&state, 1);
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                }
                Some(Consonant::ScanDecimal) => {
                    self.result.bytecode.push(Bytecode::ScanDecimal);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::ScanUnicode) => {
                    self.result.bytecode.push(Bytecode::ScanUnicode);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::Push(v)) => {
                    self.result.bytecode.push(Bytecode::Push(state.storage, v));
                }
                Some(Consonant::Duplicate) => {
                    self.require_size(&state, 1);
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    if state.storage == StorageKind::Queue {
                        self.result.bytecode.push(Bytecode::PushFront0);
                        self.result.bytecode.push(Bytecode::PushFront0);
                    } else {
                        self.result.bytecode.push(Bytecode::Push0(state.storage));
                        self.result.bytecode.push(Bytecode::Push0(state.storage));
                    }
                }
                Some(Consonant::Exchange) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    if state.storage == StorageKind::Queue {
                        self.result.bytecode.push(Bytecode::PushFront1);
                        self.result.bytecode.push(Bytecode::PushFront0);
                    } else {
                        self.result.bytecode.push(Bytecode::Push1(state.storage));
                        self.result.bytecode.push(Bytecode::Push0(state.storage));
                    }
                }
                Some(Consonant::Select(storage)) => {
                    self.result.bytecode.push(Bytecode::Select(storage));
                    state.storage = match storage {
                        // ㅇ
                        21 => StorageKind::Queue,
                        // ㅎ
                        27 => StorageKind::Stream,
                        _ => StorageKind::Stack,
                    };
                }
                Some(Consonant::Move(storage)) => {
                    self.require_size(&state, 1);
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Push0To(storage));
                }
                Some(Consonant::Compare) => {
                    self.require_size(&state, 2);
                    self.result.bytecode.push(Bytecode::Pop1(state.storage));
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    self.result.bytecode.push(Bytecode::Compare);
                    self.result.bytecode.push(Bytecode::Push0(state.storage));
                }
                Some(Consonant::Branch) => {
                    self.require_size(&state, 1);
                    self.result.bytecode.push(Bytecode::Pop0(state.storage));
                    let reverse_state = state.reverse_next(self.field);
                    let i = self.result.bytecode.len();
                    self.result.bytecode.push(Bytecode::Nop);
                    self.linearize_recursive(reverse_state);
                    let j = self.result.bytecode.len();
                    self.result.bytecode[i] = Bytecode::JumpNotEqualZero(j);
                    self.result.reference.insert(j);
                }
            }
            (state.r, state.c) = self.field.next_pos(&state);
        }
    }

    fn require_size(&mut self, state: &State, size: usize) {
        let i = self.result.bytecode.len();
        self.result.bytecode.push(Bytecode::Nop);
        let reverse_state = state.reverse_next(self.field);
        self.linearize_recursive(reverse_state);
        let j = self.result.bytecode.len();
        self.result.bytecode[i] = Bytecode::JumpSizeNotLess(size, j);
        self.result.reference.insert(j);
    }
}
