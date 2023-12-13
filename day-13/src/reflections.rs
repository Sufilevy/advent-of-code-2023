pub struct Pattern {
    pattern: Vec<Vec<char>>,
}

impl Pattern {
    pub fn from(input: &str) -> Self {
        Self {
            pattern: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn find_reflection(&self, smudges_allowed: bool) -> Reflection {
        if let Some(reflection) = self.find_reflection_column(smudges_allowed) {
            reflection
        } else {
            self.find_reflection_row(smudges_allowed)
                .unwrap_or_else(|| panic!("{:?}", self.pattern))
        }
    }

    fn find_reflection_column(&self, smudges_allowed: bool) -> Option<Reflection> {
        let len = self.pattern[0].len();
        for col_idx in 0..len - 1 {
            if self.try_col(col_idx, col_idx + 1, len, smudges_allowed) {
                return Some(Reflection::Column(col_idx));
            }
        }
        None
    }

    fn try_col(
        &self,
        mut left_idx: usize,
        mut right_idx: usize,
        len: usize,
        smudges_allowed: bool,
    ) -> bool {
        let mut smudge = false;
        while right_idx < len {
            for row in self.pattern.iter() {
                if row[left_idx] != row[right_idx] {
                    if smudges_allowed {
                        if smudge {
                            return false;
                        } else {
                            smudge = true;
                        }
                    } else {
                        return false;
                    }
                }
            }
            right_idx += 1;
            if let Some(idx) = left_idx.checked_sub(1) {
                left_idx = idx;
            } else {
                break;
            }
        }
        if smudges_allowed {
            smudge
        } else {
            true
        }
    }

    fn find_reflection_row(&self, smudges_allowed: bool) -> Option<Reflection> {
        let len = self.pattern.len();
        for row_idx in 0..len - 1 {
            if self.try_row(row_idx, row_idx + 1, len, smudges_allowed) {
                return Some(Reflection::Row(row_idx));
            }
        }
        None
    }

    fn try_row(
        &self,
        mut up_idx: usize,
        mut down_idx: usize,
        len: usize,
        smudges_allowed: bool,
    ) -> bool {
        let mut smudge = false;
        while down_idx < len {
            for col_idx in 0..self.pattern[0].len() {
                if self.pattern[up_idx][col_idx] != self.pattern[down_idx][col_idx] {
                    if smudges_allowed {
                        if smudge {
                            return false;
                        } else {
                            smudge = true;
                        }
                    } else {
                        return false;
                    }
                }
            }
            down_idx += 1;
            if let Some(idx) = up_idx.checked_sub(1) {
                up_idx = idx;
            } else {
                break;
            }
        }
        if smudges_allowed {
            smudge
        } else {
            true
        }
    }
}

pub enum Reflection {
    Column(usize),
    Row(usize),
}

impl Reflection {
    pub fn value(&self) -> u32 {
        (match self {
            Self::Column(idx) => idx + 1,
            Self::Row(idx) => (idx + 1) * 100,
        }) as u32
    }
}
