use anyhow::Result;
use anyhow::anyhow;
use bitflags::bitflags;
use core::fmt;

bitflags! {
    #[derive(Hash, PartialEq, Eq, Copy, Clone)]
    pub struct Stroke : u32 {
        const HASH = 1 << 0;
        const START_S = 1 << 1;
        const START_T = 1 << 2;
        const START_K = 1 << 3;
        const START_P = 1 << 4;
        const START_W = 1 << 5;
        const START_H = 1 << 6;
        const START_R = 1 << 7;
        const START_A = 1 << 8;
        const START_O = 1 << 9;
        const STAR = 1 << 10;
        const END_E = 1 << 11;
        const END_U = 1 << 12;
        const END_F = 1 << 13;
        const END_R = 1 << 14;
        const END_P = 1 << 15;
        const END_B = 1 << 16;
        const END_L = 1 << 17;
        const END_G = 1 << 18;
        const END_T = 1 << 19;
        const END_S = 1 << 20;
        const END_D = 1 << 21;
        const END_Z = 1 << 22;
    }
}

/*
        const HASH = 1 << 0;
        const STAR = 1 << 10;

*/

impl Stroke {
    pub fn from_str(stroke_str: &str) -> Result<Stroke> {
        let mut stroke = Stroke::empty();

        let mut seen_s = false;
        let mut seen_t = false;
        let mut seen_r = false;
        let mut seen_p = false;

        for c in stroke_str.chars() {
            match c {
                '#' => {
                    stroke |= Stroke::HASH;
                }
                'S' | 's' => {
                    if !seen_s {
                        stroke |= Stroke::START_S;
                    } else {
                        stroke |= Stroke::END_S;
                    }
                    seen_s = true;
                }
                'T' | 't' => {
                    seen_s = true;
                    if !seen_t {
                        stroke |= Stroke::START_T;
                    } else {
                        stroke |= Stroke::END_T;
                    }
                    seen_t = true;
                }
                'K' | 'k' => {
                    seen_s = true;
                    seen_t = true;
                    stroke |= Stroke::START_K;
                }
                'P' | 'p' => {
                    seen_s = true;
                    seen_t = true;
                    if !seen_p {
                        stroke |= Stroke::START_P;
                    } else {
                        stroke |= Stroke::END_P;
                    }
                    seen_p = true;
                }
                'W' | 'w' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    stroke |= Stroke::START_W;
                }
                'H' | 'h' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    stroke |= Stroke::START_H;
                }
                'R' | 'r' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    if !seen_r {
                        stroke |= Stroke::START_R;
                    } else {
                        stroke |= Stroke::END_R;
                    }
                    seen_r = true;
                }
                'A' | 'a' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::START_A;
                }
                'O' | 'o' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::START_O;
                }
                '*' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::STAR;
                }
                'E' | 'e' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_E;
                }
                'U' | 'u' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_U;
                }
                'F' | 'f' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_F;
                }
                'B' | 'b' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_B;
                }
                'L' | 'l' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_L;
                }
                'G' | 'g' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_G;
                }
                'D' | 'd' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_D;
                }
                'Z' | 'z' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                    stroke |= Stroke::END_Z;
                }
                '-' => {
                    seen_s = true;
                    seen_t = true;
                    seen_p = true;
                    seen_r = true;
                }
                _ => {
                    return Err(anyhow!(format!("Failed to parse stroke: {}", stroke_str)));
                }
            }
        }

        Ok(stroke)
    }

    pub fn as_tape(&self) -> String {
        let keys = [
            (Stroke::HASH, '#'),
            (Stroke::START_S, 'S'),
            (Stroke::START_T, 'T'),
            (Stroke::START_K, 'K'),
            (Stroke::START_P, 'P'),
            (Stroke::START_W, 'W'),
            (Stroke::START_H, 'H'),
            (Stroke::START_R, 'R'),
            (Stroke::START_A, 'A'),
            (Stroke::START_O, 'O'),
            (Stroke::STAR, '*'),
            (Stroke::END_E, 'E'),
            (Stroke::END_U, 'U'),
            (Stroke::END_F, 'F'),
            (Stroke::END_R, 'R'),
            (Stroke::END_P, 'P'),
            (Stroke::END_B, 'B'),
            (Stroke::END_L, 'L'),
            (Stroke::END_G, 'G'),
            (Stroke::END_T, 'T'),
            (Stroke::END_S, 'S'),
            (Stroke::END_D, 'D'),
            (Stroke::END_Z, 'Z'),
        ];

        keys.iter()
            .map(|&(key, char)| if self.contains(key) { char } else { ' ' })
            .collect()
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
            res.push('S');
        }
        if self.contains(Stroke::END_D) {
            res.push('D');
        }
        if self.contains(Stroke::END_Z) {
            res.push('Z');
        }

        write!(f, "{}", res)
    }
}
