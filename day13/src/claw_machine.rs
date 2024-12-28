use nalgebra::{Matrix2, Matrix2x1};

pub struct ClawMachine {
    a_button: AButton,
    b_button: BButton,
    prize: (usize, usize),
    corrected: bool,
}

impl ClawMachine {
    pub fn new(a_button: AButton, b_button: BButton, prize: (usize, usize)) -> Self {
        ClawMachine {
            a_button,
            b_button,
            prize,
            corrected: false,
        }
    }
    pub fn new_corrected(a_button: AButton, b_button: BButton, prize: (usize, usize)) -> Self {
        const ERR_COMP: usize = 10000000000000;
        ClawMachine {
            a_button,
            b_button,
            prize: (prize.0 + ERR_COMP, prize.1 + ERR_COMP),
            corrected: true,
        }
    }

    pub fn cheapest_win(&self) -> Option<usize> {
        let (a_tokens, b_tokens) = self.cheapest_win_impl();

        let validate_token = if self.corrected {
            validate_token2
        } else {
            validate_token1
        };

        if let Some(a_tokens) = validate_token(a_tokens) {
            if let Some(b_tokens) = validate_token(b_tokens) {
                return Some(a_tokens * AButton::COST + b_tokens * BButton::COST);
            }
        }
        None
    }

    fn cheapest_win_impl(&self) -> (f64, f64) {
        let ax = self.a_button.0 as f64;
        let ay = self.a_button.1 as f64;
        let bx = self.b_button.0 as f64;
        let by = self.b_button.1 as f64;
        let px = self.prize.0 as f64;
        let py = self.prize.1 as f64;

        let m_mat = Matrix2::new(ax, bx, ay, by);
        let p_mat = Matrix2x1::new(px, py);

        let im_mat = m_mat.try_inverse().unwrap();
        let c_mat = im_mat * p_mat;

        let a_tokens = *c_mat.get(0).unwrap();
        let b_tokens = *c_mat.get(1).unwrap();
        (a_tokens, b_tokens)
    }
}

fn validate_token1(tokens: f64) -> Option<usize> {
    if (tokens >= 0.0) && (tokens <= 100.0) {
        let tokens_rounded = tokens.round();
        if (tokens - tokens_rounded).abs() < 0.0001 {
            return Some(tokens_rounded as usize);
        }
    }
    None
}

fn validate_token2(tokens: f64) -> Option<usize> {
    if tokens >= 0.0 {
        let tokens_rounded = tokens.round();
        if (tokens - tokens_rounded).abs() < 0.0001 {
            return Some(tokens_rounded as usize);
        }
    }
    None
}

pub trait Button {
    const COST: usize;
}

pub struct AButton(pub usize, pub usize);

impl Button for AButton {
    const COST: usize = 3;
}

pub struct BButton(pub usize, pub usize);

impl Button for BButton {
    const COST: usize = 1;
}

#[cfg(test)]
mod test_part1 {

    use super::*;

    #[test]
    fn test1() {
        let a = AButton(94, 34);
        let b = BButton(22, 67);
        let p = (8400, 5400);

        let claw_machine = ClawMachine::new(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert_eq!(280, coins.unwrap())
    }

    #[test]
    fn test2() {
        let a = AButton(26, 66);
        let b = BButton(67, 21);
        let p = (12748, 12176);

        let claw_machine = ClawMachine::new(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert!(coins.is_none())
    }

    #[test]
    fn test3() {
        let a = AButton(17, 86);
        let b = BButton(84, 37);
        let p = (7870, 6450);

        let claw_machine = ClawMachine::new(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert_eq!(200, coins.unwrap())
    }

    #[test]
    fn test4() {
        let a = AButton(69, 23);
        let b = BButton(27, 71);
        let p = (18641, 10279);

        let claw_machine = ClawMachine::new(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert!(coins.is_none())
    }
}

#[cfg(test)]
mod test_part2 {

    use super::*;

    #[test]
    fn test1() {
        let a = AButton(94, 34);
        let b = BButton(22, 67);
        let p = (8400, 5400);

        let claw_machine = ClawMachine::new_corrected(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert!(coins.is_none())
    }

    #[test]
    fn test2() {
        let a = AButton(26, 66);
        let b = BButton(67, 21);
        let p = (12748, 12176);

        let claw_machine = ClawMachine::new_corrected(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert!(coins.is_some())
    }

    #[test]
    fn test3() {
        let a = AButton(17, 86);
        let b = BButton(84, 37);
        let p = (7870, 6450);

        let claw_machine = ClawMachine::new_corrected(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert!(coins.is_none())
    }

    #[test]
    fn test4() {
        let a = AButton(69, 23);
        let b = BButton(27, 71);
        let p = (18641, 10279);

        let claw_machine = ClawMachine::new_corrected(a, b, p);
        let coins = claw_machine.cheapest_win();
        assert!(coins.is_some())
    }
}
