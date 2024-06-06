use crate::{Key, Sequence};
use crate::Key::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Q, S, ShiftLeft, T, U, V, W, X, Y, Z};
use crate::views::definition::{P, R};

impl TryFrom<&str> for Sequence {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "q" => Ok(Sequence::new(vec![P(Q).into(), R(Q).into()])),
            "w" => Ok(Sequence::new(vec![P(W).into(), R(W).into()])),
            "e" => Ok(Sequence::new(vec![P(E).into(), R(E).into()])),
            "r" => Ok(Sequence::new(vec![P(Key::R).into(), R(Key::R).into()])),
            "t" => Ok(Sequence::new(vec![P(T).into(), R(T).into()])),
            "y" => Ok(Sequence::new(vec![P(Y).into(), R(Y).into()])),
            "u" => Ok(Sequence::new(vec![P(U).into(), R(U).into()])),
            "i" => Ok(Sequence::new(vec![P(I).into(), R(I).into()])),
            "o" => Ok(Sequence::new(vec![P(O).into(), R(O).into()])),
            "p" => Ok(Sequence::new(vec![P(Key::P).into(), R(Key::P).into()])),
            "a" => Ok(Sequence::new(vec![P(A).into(), R(A).into()])),
            "s" => Ok(Sequence::new(vec![P(S).into(), R(S).into()])),
            "d" => Ok(Sequence::new(vec![P(D).into(), R(D).into()])),
            "f" => Ok(Sequence::new(vec![P(F).into(), R(F).into()])),
            "g" => Ok(Sequence::new(vec![P(G).into(), R(G).into()])),
            "h" => Ok(Sequence::new(vec![P(H).into(), R(H).into()])),
            "j" => Ok(Sequence::new(vec![P(J).into(), R(J).into()])),
            "k" => Ok(Sequence::new(vec![P(K).into(), R(K).into()])),
            "l" => Ok(Sequence::new(vec![P(L).into(), R(L).into()])),
            "z" => Ok(Sequence::new(vec![P(Z).into(), R(Z).into()])),
            "x" => Ok(Sequence::new(vec![P(X).into(), R(X).into()])),
            "c" => Ok(Sequence::new(vec![P(C).into(), R(C).into()])),
            "v" => Ok(Sequence::new(vec![P(V).into(), R(V).into()])),
            "b" => Ok(Sequence::new(vec![P(B).into(), R(B).into()])),
            "n" => Ok(Sequence::new(vec![P(N).into(), R(N).into()])),
            "m" => Ok(Sequence::new(vec![P(M).into(), R(M).into()])),
            // todo support any shift, not only left
            "Q" => Ok((ShiftLeft, P(Q).into()).into()),
            "W" => Ok((ShiftLeft, P(W).into()).into()),
            "E" => Ok((ShiftLeft, P(E).into()).into()),
            "R" => Ok((ShiftLeft, P(Key::R).into()).into()),
            "T" => Ok((ShiftLeft, P(T).into()).into()),
            "Y" => Ok((ShiftLeft, P(Y).into()).into()),
            "U" => Ok((ShiftLeft, P(U).into()).into()),
            "I" => Ok((ShiftLeft, P(I).into()).into()),
            "O" => Ok((ShiftLeft, P(O).into()).into()),
            "P" => Ok((ShiftLeft, P(Key::P).into()).into()),
            "A" => Ok((ShiftLeft, P(A).into()).into()),
            "S" => Ok((ShiftLeft, P(S).into()).into()),
            "D" => Ok((ShiftLeft, P(D).into()).into()),
            "F" => Ok((ShiftLeft, P(F).into()).into()),
            "G" => Ok((ShiftLeft, P(G).into()).into()),
            "H" => Ok((ShiftLeft, P(H).into()).into()),
            "J" => Ok((ShiftLeft, P(J).into()).into()),
            "K" => Ok((ShiftLeft, P(K).into()).into()),
            "L" => Ok((ShiftLeft, P(L).into()).into()),
            "Z" => Ok((ShiftLeft, P(Z).into()).into()),
            "X" => Ok((ShiftLeft, P(X).into()).into()),
            "C" => Ok((ShiftLeft, P(C).into()).into()),
            "V" => Ok((ShiftLeft, P(V).into()).into()),
            "B" => Ok((ShiftLeft, P(B).into()).into()),
            "N" => Ok((ShiftLeft, P(N).into()).into()),
            "M" => Ok((ShiftLeft, P(M).into()).into()),
            value => Err(format!("Fail to parse Sequence from unsupported text: '{value}'"))
        }
    }
}