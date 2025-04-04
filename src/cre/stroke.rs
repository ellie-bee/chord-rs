use anyhow::Result;
use bitflags::bitflags;
use core::fmt;

bitflags! {
    #[derive(Hash, PartialEq, Eq)]
    pub struct Stroke : u32 {
        const HASH = 1 << 0;
        const START_S = 1 << 1;
        const START_T = 1 << 2;
        const START_K = 1 << 3;
        const START_P = 1 << 4;
        const START_W = 1 << 4;
        const START_H = 1 << 5;
        const START_R = 1 << 6;
        const START_A = 1 << 7;
        const START_O = 1 << 8;
        const STAR = 1 << 9;
        const END_E = 1 << 10;
        const END_U = 1 << 11;
        const END_F = 1 << 12;
        const END_R = 1 << 13;
        const END_P = 1 << 14;
        const END_B = 1 << 15;
        const END_L = 1 << 16;
        const END_G = 1 << 17;
        const END_T = 1 << 18;
        const END_S = 1 << 19;
        const END_D = 1 << 20;
        const END_Z = 1 << 21;
    }
}

impl Stroke {
    pub fn from_str(_stroke_str: &str) -> Result<Stroke> {
        todo!()
    }
}

impl fmt::Debug for Stroke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();

        let mut seen_s = false;
        let mut seen_t = false;
        let mut seen_p = false;
        let mut seen_r = false;

        if self.contains(Stroke::HASH) {
            res.push('#');
        }
        if self.contains(Stroke::START_S) {
            seen_s = true;
            res.push('S');
        }
        if self.contains(Stroke::START_T) {
            seen_s = true;
            seen_t = true;
            res.push('T');
        }
        if self.contains(Stroke::START_K) {
            seen_s = true;
            seen_t = true;
            res.push('K');
        }
        if self.contains(Stroke::START_P) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            res.push('P');
        }
        if self.contains(Stroke::START_W) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            res.push('W');
        }
        if self.contains(Stroke::START_H) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            res.push('H');
        }
        if self.contains(Stroke::START_R) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('R');
        }
        if self.contains(Stroke::START_A) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('A');
        }
        if self.contains(Stroke::START_O) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('O');
        }
        if self.contains(Stroke::STAR) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('*');
        }
        if self.contains(Stroke::END_E) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('E');
        }
        if self.contains(Stroke::END_U) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('U');
        }
        if self.contains(Stroke::END_F) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            seen_r = true;
            res.push('F');
        }
        if self.contains(Stroke::END_R) {
            seen_s = true;
            seen_t = true;
            seen_p = true;
            if !seen_r {
                res.push('-');
            }
            res.push('R');
        }
        if self.contains(Stroke::END_P) {
            seen_s = true;
            seen_t = true;
            if !seen_p {
                res.push('-');
            }
            res.push('P');
        }
        if self.contains(Stroke::END_B) {
            seen_s = true;
            seen_t = true;
            res.push('B');
        }
        if self.contains(Stroke::END_L) {
            seen_s = true;
            seen_t = true;
            res.push('L');
        }
        if self.contains(Stroke::END_G) {
            seen_s = true;
            seen_t = true;
            res.push('G');
        }
        if self.contains(Stroke::END_T) {
            seen_s = true;
            if !seen_t {
                res.push('-');
            }
            res.push('T');
        }
        if self.contains(Stroke::END_S) {
            if !seen_s {
                res.push('-');
            }
            res.push('G');
        }
        if self.contains(Stroke::END_D) {
            res.push('G');
        }
        if self.contains(Stroke::END_Z) {
            res.push('G');
        }

        write!(f, "{}", res)
    }
}
