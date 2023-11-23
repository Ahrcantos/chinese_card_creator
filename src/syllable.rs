use std::fmt::Display;

pub struct Syllable(pub Initial, pub Final, pub Tone);

pub enum Tone {
    Flat,
    Rising,
    FallingRising,
    Falling,
    Neutral,
}

pub enum Initial {
    None,
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    L,
    G,
    K,
    H,
    J,
    Q,
    X,
    Z,
    C,
    S,
    ZH,
    CH,
    SH,
    R,
}

pub enum Final {
    I,
    A,
    AI,
    AN,
    ANG,
    AO,
    E,
    EI,
    EN,
    ENG,
    ER,
    IA,
    IAN,
    IANG,
    IAO,
    IE,
    IN,
    ING,
    IONG,
    IOU,
    O,
    ONG,
    OU,
    U,
    UA,
    UAI,
    UAN,
    UANG,
    UO,
    V,
    VAN,
    VE,
    VN,
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Syllable(Initial::Z, Final::I, Tone::Flat) => write!(f, "zī"),
            Syllable(Initial::Z, Final::I, Tone::Rising) => write!(f, "zí"),
            Syllable(Initial::Z, Final::I, Tone::FallingRising) => write!(f, "zǐ"),
            Syllable(Initial::Z, Final::I, Tone::Falling) => write!(f, "zì"),
            Syllable(Initial::Z, Final::I, Tone::Neutral) => write!(f, "zi"),
            _ => write!(f, "Error"),
        }
    }
}
