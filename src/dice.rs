use crate::types::*;

use std::rc::*;

use rand::Rng;

/// The primitive type used to represent a die value.
pub type Value = isize;

/// Represents an n-sided die
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Die(pub Value);

/// An arbitrary expression of dice. No guarantee is given as to its structure.
#[derive(Debug,Clone)]
pub enum DiceExpr {
    Die(Die),
    Times(usize, Rc<DiceExpr>),
    Plus(Rc<DiceExpr>, Rc<DiceExpr>),
    Const(Value),
}

/// The result of rolling a `DiceExpr`, including all intermediate values. This is suitable for
/// storing a "zero-entropy" copy of this data for posterity.
#[derive(Debug,Clone)]
pub enum DiceRoll {
    Die(Die, Value),
    Times(usize, Rc<DiceExpr>, Vec<DiceRoll>),
    Plus(Rc<DiceExpr>, Rc<DiceExpr>, Rc<DiceRoll>, Rc<DiceRoll>),
    Const(Value),
}

impl DiceExpr {
    /// Roll the DiceExpr using the entropy source.
    pub fn roll<R: Rng>(&self, rng: &mut R) -> DiceRoll {
        match self {
            DiceExpr::Die(d) => DiceRoll::Die(*d, rng.gen_range(1, d.0 + 1)),
            DiceExpr::Times(n, ex) => DiceRoll::Times(*n, Rc::clone(ex),
                (0..*n).map(|_| ex.roll(rng)).collect(),
            ),
            DiceExpr::Plus(xa, xb) => DiceRoll::Plus(
                Rc::clone(xa), Rc::clone(xb),
                Rc::new(xa.roll(rng)), Rc::new(xb.roll(rng)),
            ),
            DiceExpr::Const(v) => DiceRoll::Const(*v),
        }
    }

    /// Cumulative probability--the probability that, given underlying distribution X, the
    /// resulting value gives x <= i. Note that this is a "roll under"; see `prob_pass` below.
    ///
    /// This implementation intentionally has a number of unimplemented cases due to the general
    /// intractability of the binomial distribution. Implementations should strive to put as much
    /// of the calculation into `i` as possible.
    pub fn cum_prob(&self, i: Value) -> f64 {
        match self {
            DiceExpr::Die(d) => {
                if i <= 0 {
                    0.0
                } else if i >= d.0 {
                    1.0
                } else {
                    (i as f64) / (d.0 as f64)
                }
            },
            DiceExpr::Const(c) => {
                if i < *c {
                    0.0
                } else {
                    1.0
                }
            },
            _ => unimplemented!(),
        }
    }

    /// Probability of a roll "at or over" a target. Uses `cum_prob` internally, and thus inherits
    /// all of its limitations.
    pub fn prob_pass(&self, check: Value) -> f64 {
        1.0 - self.cum_prob(check - 1)
    }
}

impl ExpectedValue for DiceExpr {
    fn expected(&self) -> f64 {
        match self {
            DiceExpr::Die(d) => (1.0 + (d.0 as f64)) / 2.0,
            DiceExpr::Times(n, x) => (*n as f64) * x.expected(),
            DiceExpr::Plus(xa, xb) => xa.expected() + xb.expected(),
            DiceExpr::Const(v) => *v as f64,
        }
    }
}

impl DiceRoll {
    /// Get the numerical value of a DiceRoll.
    pub fn value(&self) -> Value {
        match self {
            DiceRoll::Die(_, v) => *v,
            DiceRoll::Times(_, _, drs) => drs.iter().map(DiceRoll::value).sum(),
            DiceRoll::Plus(_, _, va, vb) => va.value() + vb.value(),
            DiceRoll::Const(v) => *v,
        }
    }

    /// Reconstruct the original expression that resulted in this roll.
    pub fn expr(&self) -> DiceExpr {
        match self {
            DiceRoll::Die(d, _) => DiceExpr::Die(*d),
            DiceRoll::Times(n, x, _) => DiceExpr::Times(*n, Rc::clone(x)),
            DiceRoll::Plus(xa, xb, _, _) => DiceExpr::Plus(Rc::clone(xa), Rc::clone(xb)),
            DiceRoll::Const(v) => DiceExpr::Const(*v),
        }
    }
}
