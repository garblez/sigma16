use std::convert::From;

pub enum ConditionCode {
    IntegerGreaterThan,
    NaturalGreaterThan,
    Equals,
    NaturalLessThan,
    IntegerLessThan,
    IntegerOverflow,
    NaturalOverflow,
    Carry,
    StackOverflow,
    StackUnderflow,
    LogicFunctionResult,
}


impl From<ConditionCode> for u8 {
    fn from(condition: ConditionCode) -> Self {
        use ConditionCode::*;
        match condition {
            IntegerGreaterThan => 0,
            NaturalGreaterThan => 1,
            Equals => 2,
            NaturalLessThan => 3,
            IntegerLessThan => 4,
            IntegerOverflow => 5,
            NaturalOverflow => 6,
            Carry => 7,
            StackOverflow => 8,
            StackUnderflow => 9,
            LogicFunctionResult => 10,
        }
    }
}
