use crate::sound::Sound;
use crate::dash_dot::DashDot;

pub(crate) const LETTER_A: &str = ".-";
pub(crate) const LETTER_B: &str = "-...";
pub(crate) const LETTER_C: &str = "-.-.";
pub(crate) const LETTER_D: &str = "-..";
pub(crate) const LETTER_E: &str = ".";
pub(crate) const LETTER_F: &str = "..-.";
pub(crate) const LETTER_G: &str = "--.";
pub(crate) const LETTER_H: &str = "....";
pub(crate) const LETTER_I: &str = "..";
pub(crate) const LETTER_J: &str = ".---";
pub(crate) const LETTER_K: &str = "-.-";
pub(crate) const LETTER_L: &str = ".-..";
pub(crate) const LETTER_M: &str = "--";
pub(crate) const LETTER_N: &str = "-.";
pub(crate) const LETTER_O: &str = "---";
pub(crate) const LETTER_P: &str = ".--.";
pub(crate) const LETTER_Q: &str = "--.-";
pub(crate) const LETTER_R: &str = ".-.";
pub(crate) const LETTER_S: &str = "...";
pub(crate) const LETTER_T: &str = "-";
pub(crate) const LETTER_U: &str = "..-";
pub(crate) const LETTER_V: &str = "...-";
pub(crate) const LETTER_W: &str = ".--";
pub(crate) const LETTER_X: &str = "-..-";
pub(crate) const LETTER_Y: &str = "-.--";
pub(crate) const LETTER_Z: &str = "--..";
pub(crate) const NUMBER_0: &str = "-----";
pub(crate) const NUMBER_1: &str = ".----";
pub(crate) const NUMBER_2: &str = "..---";
pub(crate) const NUMBER_3: &str = "...--";
pub(crate) const NUMBER_4: &str = "....-";
pub(crate) const NUMBER_5: &str = ".....";
pub(crate) const NUMBER_6: &str = "-....";
pub(crate) const NUMBER_7: &str = "--...";
pub(crate) const NUMBER_8: &str = "---..";
pub(crate) const NUMBER_9: &str = "----.";

// pub(crate) const PONCTUATION_PERIOD: &str = ".-.-.-";
// pub(crate) const PONCTUATION_COMMA: &str = "--..--";
// pub(crate) const PONCTUATION_QUESTION: &str = "..--..";
// pub(crate) const PONCTUATION_APOSTROPHE: &str = ".----.";
// pub(crate) const PONCTUATION_EXCLAMATION: &str = "-.-.--";
// pub(crate) const PONCTUATION_SLASH: &str = "-..-.";
// pub(crate) const PONCTUATION_PARENTHESES: &str = "-.--.-";
// pub(crate) const PONCTUATION_AMPERSAND: &str = ".-...";
// pub(crate) const PONCTUATION_COLON: &str = "---...";
// pub(crate) const PONCTUATION_SEMICOLON: &str = "-.-.-.";
// pub(crate) const PONCTUATION_EQUAL: &str = "-...-";
// pub(crate) const PONCTUATION_PLUS: &str = ".-.-.";
// pub(crate) const PONCTUATION_MINUS: &str = "-....-";
// pub(crate) const PONCTUATION_UNDERSCORE: &str = "..--.-";
// pub(crate) const PONCTUATION_QUOTE: &str = ".-..-.";
// pub(crate) const PONCTUATION_DOLLAR: &str = "...-..-";
// pub(crate) const PONCTUATION_AT: &str = ".--.-.";
pub(crate) const PONCTUATION_SPACE: &str = "/";

#[derive(Debug)]
pub enum MorseCodes {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    L0, L1, L2, L3, L4, L5, L6, L7, L8, L9, Space, Dot, EndOfChar,
}

impl  MorseCodes {
    pub fn to_morse_str(&self) -> &str {
        use MorseCodes::*;
        match self {
            A => LETTER_A,
            B => LETTER_B,
            C => LETTER_C,
            D => LETTER_D,
            E => LETTER_E,
            F => LETTER_F,
            G => LETTER_G,
            H => LETTER_H,
            I => LETTER_I,
            J => LETTER_J,
            K => LETTER_K,
            L => LETTER_L,
            M => LETTER_M,
            N => LETTER_N,
            O => LETTER_O,
            P => LETTER_P,
            Q => LETTER_Q,
            R => LETTER_R,
            S => LETTER_S,
            T => LETTER_T,
            U => LETTER_U,
            V => LETTER_V,
            W => LETTER_W,
            X => LETTER_X,
            Y => LETTER_Y,
            Z => LETTER_Z,
            L0 => NUMBER_0,
            L1 => NUMBER_1,
            L2 => NUMBER_2,
            L3 => NUMBER_3,
            L4 => NUMBER_4,
            L5 => NUMBER_5,
            L6 => NUMBER_6,
            L7 => NUMBER_7,
            L8 => NUMBER_8,
            L9 => NUMBER_9,
            Space | Dot => "",
            EndOfChar => PONCTUATION_SPACE,
        }
    }

    pub fn from_morse_str(s: &str) -> Option<MorseCodes> {
        use MorseCodes::*;
        match s {
            LETTER_A => Some(A),
            LETTER_B => Some(B),
            LETTER_C => Some(C),
            LETTER_D => Some(D),
            LETTER_E => Some(E),
            LETTER_F => Some(F),
            LETTER_G => Some(G),
            LETTER_H => Some(H),
            LETTER_I => Some(I),
            LETTER_J => Some(J),
            LETTER_K => Some(K),
            LETTER_L => Some(L),
            LETTER_M => Some(M),
            LETTER_N => Some(N),
            LETTER_O => Some(O),
            LETTER_P => Some(P),
            LETTER_Q => Some(Q),
            LETTER_R => Some(R),
            LETTER_S => Some(S),
            LETTER_T => Some(T),
            LETTER_U => Some(U),
            LETTER_V => Some(V),
            LETTER_W => Some(W),
            LETTER_X => Some(X),
            LETTER_Y => Some(Y),
            LETTER_Z => Some(Z),
            NUMBER_0 => Some(L0),
            NUMBER_1 => Some(L1),
            NUMBER_2 => Some(L2),
            NUMBER_3 => Some(L3),
            NUMBER_4 => Some(L4),
            NUMBER_5 => Some(L5),
            NUMBER_6 => Some(L6),
            NUMBER_7 => Some(L7),
            NUMBER_8 => Some(L8),
            NUMBER_9 => Some(L9),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<MorseCodes> {
        use MorseCodes::*;
        match s.to_lowercase().as_str() {
            "a" => Some(A),
            "b" => Some(B),
            "c" => Some(C),
            "d" => Some(D),
            "e" => Some(E),
            "f" => Some(F),
            "g" => Some(G),
            "h" => Some(H),
            "i" => Some(I),
            "j" => Some(J),
            "k" => Some(K),
            "l" => Some(L),
            "m" => Some(M),
            "n" => Some(N),
            "o" => Some(O),
            "p" => Some(P),
            "q" => Some(Q),
            "r" => Some(R),
            "s" => Some(S),
            "t" => Some(T),
            "u" => Some(U),
            "v" => Some(V),
            "w" => Some(W),
            "x" => Some(X),
            "y" => Some(Y),
            "z" => Some(Z),
            "0" => Some(L0),
            "1" => Some(L1),
            "2" => Some(L2),
            "3" => Some(L3),
            "4" => Some(L4),
            "5" => Some(L5),
            "6" => Some(L6),
            "7" => Some(L7),
            "8" => Some(L8),
            "9" => Some(L9),
            " " => Some(Space),
            "." => Some(Dot),
            _ => None,
        }
    }

    pub fn to_str(&self) -> String {
        use MorseCodes::*;
        match self {
            A => "a",
            B => "b",
            C => "c",
            D => "d",
            E => "e",
            F => "f",
            G => "g",
            H => "h",
            I => "i",
            J => "j",
            K => "k",
            L => "l",
            M => "m",
            N => "n",
            O => "o",
            P => "p",
            Q => "q",
            R => "r",
            S => "s",
            T => "t",
            U => "u",
            V => "v",
            W => "w",
            X => "x",
            Y => "y",
            Z => "z",
            L0 => "0",
            L1 => "1",
            L2 => "2",
            L3 => "3",
            L4 => "4",
            L5 => "5",
            L6 => "6",
            L7 => "7",
            L8 => "8",
            L9 => "9",
            Space => " ",
            Dot => ".",
            _ => "",
        }.to_string()
    }
}

impl Sound for MorseCodes {
    fn play(&self) {
        self.to_morse_str().chars().for_each(|c| {
            if let Some(dd) = DashDot::from_str(&c.to_string()) {
                dd.play();
            }
        });
    }
}
