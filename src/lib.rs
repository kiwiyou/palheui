use std::collections::{HashMap, HashSet, VecDeque};

// Integer type used throughout aheui runtime.
type Integer = i64;

#[derive(Clone, Copy, Debug)]
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
    for line in code.iter() {
        output.push_str(line);
        output.push('\n');
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

impl From<usize> for StorageKind {
    fn from(value: usize) -> Self {
        match value {
            21 => Self::Queue,
            27 => Self::Stream,
            _ => Self::Stack,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    r: usize,
    c: usize,
    direction: Direction,
    speed: usize,
    storage: usize,
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

struct Linearizer<'a> {
    field: &'a Field,
    code: &'a [Syllable],
    blocks: Vec<String>,
    state_memo: HashMap<(State, usize), usize>,
}

impl<'a> Linearizer<'a> {
    fn new(field: &'a Field, code: &'a [Syllable]) -> Self {
        assert!(!code.is_empty(), "`code` must not be empty");
        Self {
            field,
            code,
            blocks: vec![],
            state_memo: HashMap::new(),
        }
    }

    fn linearize(mut self) -> Vec<String> {
        self.linearize_recursive(
            State {
                r: 0,
                c: 0,
                direction: Direction::Down,
                speed: 1,
                storage: 0,
            },
            0,
        );
        self.blocks
    }

    fn linearize_recursive(&mut self, mut state: State, presize: usize) -> usize {
        if let Some(&label) = self.state_memo.get(&(state.clone(), presize)) {
            return label;
        }
        let init_storage = state.storage;
        let entry = self.blocks.len();
        self.state_memo.insert((state.clone(), presize), entry);
        let mut size = [0usize; 28];
        size[state.storage] = presize;
        let mut visited = HashSet::new();
        let mut block = vec![];
        loop {
            if !visited.insert(state.clone()) {
                self.optimize_block(entry, init_storage, presize, &block);
                let i = self.blocks.len();
                self.blocks.push("".into());
                let j = self.linearize_recursive(state, 0);
                self.blocks[i] = format!("    goto B{j};");
                break;
            }
            let prev = state.clone();
            let pos = state.r * self.field.w + state.c;
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
            match self.code[pos].consonant {
                None => {}
                Some(Consonant::Halt) => {
                    self.optimize_block(entry, init_storage, presize, &block);
                    let storage = state.storage;
                    if StorageKind::from(storage) == StorageKind::Queue {
                        self.blocks.push(format!("    flush(&output); return size[{storage}] ? pop_queue(&storage[{storage}].queue) : 0;"));
                    } else {
                        self.blocks.push(format!("    flush(&output); return size[{storage}] ? storage[{storage}].stack.memory[--size[{storage}]] : 0;"));
                    }
                    break;
                }
                Some(
                    c @ (Consonant::Add
                    | Consonant::Subtract
                    | Consonant::Multiply
                    | Consonant::Divide
                    | Consonant::Remainder
                    | Consonant::Compare),
                ) => {
                    if size[state.storage] < 2 {
                        let storage = state.storage;
                        self.optimize_block(entry, init_storage, presize, &block);
                        let i = self.blocks.len();
                        self.blocks.push("".into());
                        let reverse_state = state.reverse_next(self.field);
                        let j = self.linearize_recursive(reverse_state, 0);
                        let k = self.linearize_recursive(prev, 2);
                        self.blocks[i] =
                            format!("    if (size[{storage}] < 2) goto B{j}; else goto B{k};");
                        break;
                    }
                    block.push(c);
                    size[state.storage] = size[state.storage].saturating_sub(1);
                }
                Some(c @ (Consonant::PrintDecimal | Consonant::PrintUnicode | Consonant::Pop)) => {
                    if size[state.storage] < 1 {
                        let storage = state.storage;
                        self.optimize_block(entry, init_storage, presize, &block);
                        let i = self.blocks.len();
                        self.blocks.push("".into());
                        let reverse_state = state.reverse_next(self.field);
                        let j = self.linearize_recursive(reverse_state, 0);
                        let k = self.linearize_recursive(prev, 1);
                        self.blocks[i] =
                            format!("    if (size[{storage}] < 1) goto B{j}; else goto B{k};");
                        break;
                    }
                    block.push(c);
                    size[state.storage] = size[state.storage].saturating_sub(1);
                }
                Some(
                    c @ (Consonant::ScanDecimal | Consonant::ScanUnicode | Consonant::Push(..)),
                ) => {
                    block.push(c);
                    size[state.storage] += 1;
                }
                Some(c @ Consonant::Duplicate) => {
                    if size[state.storage] < 1 {
                        let storage = state.storage;
                        self.optimize_block(entry, init_storage, presize, &block);
                        let i = self.blocks.len();
                        self.blocks.push("".into());
                        let reverse_state = state.reverse_next(self.field);
                        let j = self.linearize_recursive(reverse_state, 0);
                        let k = self.linearize_recursive(prev, 1);
                        self.blocks[i] =
                            format!("    if (size[{storage}] < 1) goto B{j}; else goto B{k};");
                        break;
                    }
                    block.push(c);
                    size[state.storage] += 1;
                }
                Some(c @ Consonant::Exchange) => {
                    if size[state.storage] < 2 {
                        let storage = state.storage;
                        self.optimize_block(entry, init_storage, presize, &block);
                        let i = self.blocks.len();
                        self.blocks.push("".into());
                        let reverse_state = state.reverse_next(self.field);
                        let j = self.linearize_recursive(reverse_state, 0);
                        let k = self.linearize_recursive(prev, 2);
                        self.blocks[i] =
                            format!("    if (size[{storage}] < 2) goto B{j}; else goto B{k};");
                        break;
                    }
                    block.push(c);
                }
                Some(c @ Consonant::Select(s)) => {
                    block.push(c);
                    state.storage = s as usize;
                }
                Some(c @ Consonant::Move(s)) => {
                    if size[state.storage] < 1 {
                        let storage = state.storage;
                        self.optimize_block(entry, init_storage, presize, &block);
                        let i = self.blocks.len();
                        self.blocks.push("".into());
                        let reverse_state = state.reverse_next(self.field);
                        let j = self.linearize_recursive(reverse_state, 0);
                        let k = self.linearize_recursive(prev, 1);
                        self.blocks[i] =
                            format!("    if (size[{storage}] < 1) goto B{j}; else goto B{k};");
                        break;
                    }
                    block.push(c);
                    size[state.storage] = size[state.storage].saturating_sub(1);
                    size[s as usize] += 1;
                }
                Some(Consonant::Branch) => {
                    if size[state.storage] < 1 {
                        let storage = state.storage;
                        self.optimize_block(entry, init_storage, presize, &block);
                        let i = self.blocks.len();
                        self.blocks.push("".into());
                        let reverse_state = state.reverse_next(self.field);
                        let j = self.linearize_recursive(reverse_state, 0);
                        let k = self.linearize_recursive(prev, 1);
                        self.blocks[i] =
                            format!("    if (size[{storage}] < 1) goto B{j}; else goto B{k};");
                        break;
                    }
                    self.optimize_block(entry, init_storage, presize, &block);
                    let i = self.blocks.len();
                    let storage = state.storage;
                    self.blocks.push("".into());
                    let reverse_state = state.reverse_next(self.field);
                    let j = self.linearize_recursive(reverse_state, 0);
                    (state.r, state.c) = self.field.next_pos(&state);
                    let k = self.linearize_recursive(state, 0);
                    if StorageKind::from(storage) == StorageKind::Queue {
                        self.blocks[i] =  format!("    if ((size[{storage}]--, pop_queue(&storage[{storage}].queue))) goto B{k}; else goto B{j};");
                    } else {
                        self.blocks[i] =  format!("    if (storage[{storage}].stack.memory[--size[{storage}]]) goto B{k}; else goto B{j};");
                    }
                    break;
                }
            }
            (state.r, state.c) = self.field.next_pos(&state);
        }
        entry
    }

    fn optimize_block(
        &mut self,
        label: usize,
        init_storage: usize,
        presize: usize,
        block: &[Consonant],
    ) {
        use std::fmt::Write;
        let mut output = String::new();
        let mut id = 0;
        let mut var = vec![VecDeque::new(); 28];
        let mut storage = init_storage;
        for _ in 0..presize {
            if StorageKind::from(storage) == StorageKind::Queue {
                writeln!(
                    output,
                    "    size[{storage}]--; integer v{id} = pop_queue(&storage[{storage}].queue);"
                )
                .ok();
                var[storage].push_back(id);
            } else {
                writeln!(
                    output,
                    "    integer v{id} = storage[{storage}].stack.memory[--size[{storage}]];",
                )
                .ok();
                var[storage].push_front(id);
            }
            id += 1;
        }
        for code in block {
            match code {
                Consonant::Halt | Consonant::Branch => {}
                Consonant::Add => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    let b = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    integer v{id} = v{b} + v{a};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Multiply => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    let b = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    integer v{id} = v{b} * v{a};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Subtract => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    let b = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    integer v{id} = v{b} - v{a};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Divide => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    let b = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    integer v{id} = v{b} / v{a};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Remainder => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    let b = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    integer v{id} = v{b} % v{a};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::PrintDecimal => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    print_decimal(&output, v{a});").ok();
                }
                Consonant::PrintUnicode => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    print_utf8(&output, v{a});").ok();
                }
                Consonant::ScanDecimal => {
                    writeln!(output, "    integer v{id} = scan_decimal(&input);").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::ScanUnicode => {
                    writeln!(output, "    integer v{id} = scan_utf8(&input);").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Select(s) => storage = *s as usize,
                Consonant::Compare => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    let b = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    writeln!(output, "    integer v{id} = v{b} >= v{a};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Exchange => {
                    if StorageKind::from(storage) == StorageKind::Queue {
                        let a = var[storage].pop_front().unwrap();
                        let b = var[storage].pop_front().unwrap();
                        var[storage].push_front(a);
                        var[storage].push_front(b);
                    } else {
                        let a = var[storage].pop_back().unwrap();
                        let b = var[storage].pop_back().unwrap();
                        var[storage].push_back(a);
                        var[storage].push_back(b);
                    }
                }
                Consonant::Duplicate => {
                    if StorageKind::from(storage) == StorageKind::Queue {
                        let a = var[storage].pop_front().unwrap();
                        var[storage].push_front(a);
                        var[storage].push_front(a);
                    } else {
                        let a = var[storage].pop_back().unwrap();
                        var[storage].push_back(a);
                        var[storage].push_back(a);
                    };
                }
                Consonant::Pop => {
                    if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                }
                Consonant::Push(v) => {
                    writeln!(output, "    integer v{id} = {v};").ok();
                    var[storage].push_back(id);
                    id += 1;
                }
                Consonant::Move(s) => {
                    let a = if StorageKind::from(storage) == StorageKind::Queue {
                        var[storage].pop_front().unwrap()
                    } else {
                        var[storage].pop_back().unwrap()
                    };
                    var[*s as usize].push_back(a);
                }
            }
        }
        for (i, storage) in var.into_iter().enumerate() {
            if StorageKind::from(i) == StorageKind::Queue {
                for id in storage {
                    writeln!(
                        output,
                        "    size[{i}]++; push_queue(&storage[{i}].queue, v{id}, size[{i}]);"
                    )
                    .ok();
                }
            } else {
                for id in storage {
                    writeln!(
                        output,
                        "    push_stack(&storage[{i}].stack, size[{i}]++, v{id});"
                    )
                    .ok();
                }
            }
        }
        if output.is_empty() {
            self.blocks.push(format!("B{label}:"));
        } else {
            self.blocks.push(format!("B{label}:{{\n{output}}}"));
        }
    }
}
