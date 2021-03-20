#[derive(Debug, Clone)]
pub struct PrimitiveInstruction {
    kind: PrimitiveInstructionKind,
    conditional: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveInstructionKind {
    Break,
    NoOperation,
    Jump,
    JumpToSubroutine,
    PushNextInstructionPointer,
    PushDataStackPointer,
    SetInterruptDisable,
    ClearInterruptDisable,
    Stash(DataInstructionSize),
    FetchMemory(DataInstructionSize),
    StoreMemory(DataInstructionSize),
    AddWithCarry(DataInstructionSize),
    SubtractWithBorrow(DataInstructionSize),
    BitwiseNot(DataInstructionSize),
    And(DataInstructionSize),
    InclusiveOr(DataInstructionSize),
    ExclusiveOr(DataInstructionSize),
    ShiftLeft(DataInstructionSize),
    ShiftRight(DataInstructionSize),
    Increment(StackSpecifier),
    Add(StackSpecifier),
    Drop(StackSpecifier),
    Decrement(StackSpecifier),
    Duplicate(StackSpecifier),
    Subtract(StackSpecifier),
    Swap(StackSpecifier),
    EqualToZero(StackSpecifier),
    Over(StackSpecifier),
    NotEqualToZero(StackSpecifier),
    RotateForward(StackSpecifier),
    GreaterThan(StackSpecifier),
    RotateBackward(StackSpecifier),
    LesserThan(StackSpecifier),
}

#[derive(Debug, Clone, Copy)]
pub enum DataInstructionSize {
    S1 = 1,
    S2 = 2,
    S4 = 4,
    S8 = 8,
}

#[derive(Debug, Clone, Copy)]
pub enum StackSpecifier {
    AddressStack,
    DataStack(DataInstructionSize),
}

#[derive(Debug, Clone, Copy)]
pub enum YotType {
    Y8 = 1,
    Y16 = 2,
    Y32 = 4,
    Y64 = 8,
}

/*
pub fn emit(instruction: &Instruction, yot_type: YotType) -> Vec<u8> {
    let base: u8 = match instruction.kind {
        InstructionKind::Unsized(unsized_kind) => unsized_kind as u8,
        InstructionKind::Sized(sized_kind, size) => sized_kind as u8 + size as u8 * 0x10,
    } + instruction.conditional as u8 * 0x80;

    let mut encoded_instruction: Vec<u8> = vec![base];

    let mut push_literal = |size: usize| {
        let mut shifted_literal = instruction.literal.unwrap();
        for _ in 0..size {
            encoded_instruction.push((shifted_literal & 0xff) as u8);
            shifted_literal >>= 8;
        }
    };

    if let InstructionKind::Sized(SizedInstructionKind::PushLiteral, size) = instruction.kind {
        push_literal(size as usize);
    } else if let InstructionKind::Unsized(UnsizedInstructionKind::PushLiteral) = instruction.kind {
        push_literal(yot_type as usize);
    }

    encoded_instruction
}
*/
