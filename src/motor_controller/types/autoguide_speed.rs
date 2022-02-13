use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AutoGuideSpeed {
    One,
    ThreeQuarters,
    Half,
    Quarter,
    Eighth,
}

impl AutoGuideSpeed {
    pub fn multiplier(&self) -> f64 {
        match self {
            AutoGuideSpeed::One => 1.0,
            AutoGuideSpeed::ThreeQuarters => 0.75,
            AutoGuideSpeed::Half => 0.5,
            AutoGuideSpeed::Quarter => 0.25,
            AutoGuideSpeed::Eighth => 0.125,
        }
    }

    pub(crate) fn comm_byte(&self) -> u8 {
        match self {
            AutoGuideSpeed::One => b'0',
            AutoGuideSpeed::ThreeQuarters => b'1',
            AutoGuideSpeed::Half => b'2',
            AutoGuideSpeed::Quarter => b'3',
            AutoGuideSpeed::Eighth => b'4',
        }
    }
}

impl PartialOrd<Self> for AutoGuideSpeed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.multiplier().partial_cmp(&other.multiplier())
    }
}

impl Ord for AutoGuideSpeed {
    fn cmp(&self, other: &Self) -> Ordering {
        self.multiplier().partial_cmp(&other.multiplier()).unwrap()
    }
}
