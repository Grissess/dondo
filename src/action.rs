use crate::damage::{DamageKind, Damage};
use crate::dice::*;
use crate::space::Area;
use crate::types::*;
use crate::basetraits::*;
use crate::util;

use std::rc::Rc;
use std::cmp::max;

use rand::Rng;

/// Represents a roll one would make to do damage of a certain kind (attacks can possess more than
/// one damage roll--generally, one per kind of damage).
#[derive(Debug,Clone)]
pub struct DamageRoll(pub DiceExpr, pub DamageKind);

impl DamageRoll {
    fn roll<R: Rng>(&self, rng: &mut R) -> (Damage, DiceRoll) {
        let dr = self.0.roll(rng);
        (
            Damage(util::clamp_isize(dr.value()), self.1),
            dr
        )
    }
}

impl ExpectedValue for DamageRoll {
    fn expected(&self) -> f64 {
        self.0.expected()
    }
}

/// Expresses how many targets an action can affect. Exactly indicates that only the exact number
/// can be targeted; Area indicates that an area is targeted. See also `AreaEffectDensity`. The
/// default is Exactly(1).
#[derive(Debug,Clone)]
pub enum Target {
    Exactly(usize),
    Area(Area),
}

impl Default for Target {
    fn default() -> Target {
        Target::Exactly(1)
    }
}

/// A saving throw DC versus an effect. The granting ability is implicit in most monsters, and
/// requires some work to derive; for example, all dragons have Con-granted breath weapon DCs, and
/// Cha-granted Frightful Presence DCs.
#[derive(Debug,Clone)]
pub enum SavingDC {
    Granted(Ability),
    Exactly(usize),
}

impl Default for SavingDC {
    fn default() -> SavingDC {
        SavingDC::Exactly(10)
    }
}

impl SavingDC {
    pub fn def_class(&self, mods: &AMods, prof: ProfBonus) -> usize {
        match self {
            SavingDC::Granted(ab) => util::clamp_isize(8 + prof.0 + mods.0[*ab]),
            SavingDC::Exactly(dc) => *dc,
        }
    }
}

/// The kind of saving throw for an effect.
#[derive(Debug,Clone)]
pub enum SaveKind {
    Ability(Ability),
    Death,
}

impl SaveKind {
    pub fn modifier(&self, mods: &AMods) -> isize {
        match self {
            SaveKind::Ability(ab) => mods.0[*ab],
            SaveKind::Death => 0,
        }
    }
}

/// The effects that a successful save can have.
#[derive(Debug,Clone)]
pub enum SaveEffect {
    ReducesDamage(f64),
}

/// The actual description of a saving throw.
#[derive(Debug,Clone)]
pub struct Save(pub SaveKind, pub SavingDC, pub SaveEffect);

/// How many uses the effect has in combat.
#[derive(Debug,Clone)]
pub enum Uses {
    Indefinite,
    PerDay(usize),
    Recharge(Value, Die),
}

/// Which kind of attack this is (controls which modifiers, if any, are selected).
#[derive(Debug,Clone)]
pub enum AttackKind {
    Melee,
    Ranged,
    Special,
}

impl AttackKind {
    /// The "to hit" modifier for this kind of attack; see 5e PHB, p. 195
    pub fn modifier(&self, mods: &AMods) -> isize {
        match self {
            AttackKind::Melee => mods.0.str,
            AttackKind::Ranged => mods.0.dex,
            AttackKind::Special => 0,
        }
    }
}

/// The full description of an attack.
#[derive(Debug,Clone)]
pub struct Attack {
    pub kind: AttackKind,
    pub save: Option<Save>,
    pub target: Target,
    pub dmg_rolls: Vec<DamageRoll>,
    pub dmg_bonus: isize,  // Added to the first kind of damage in dmg_rolls
    pub to_hit_bonus: isize,
    pub finesse: bool,
    pub proficient: bool,
    pub range: usize,
}

impl Default for Attack {
    fn default() -> Attack {
        Attack {
            kind: AttackKind::Melee,
            save: None,
            target: Default::default(),
            dmg_rolls: Vec::new(),
            dmg_bonus: 0,
            to_hit_bonus: 0,
            finesse: false,
            proficient: false,
            range: 5,
        }
    }
}

impl Attack {
    /// The "to hit" modifier; see 5e PHB, p. 194
    pub fn modifier(&self, mods: &AMods, prof: ProfBonus) -> isize {
        self.to_hit_bonus + if self.proficient { prof.0 } else { 0 } + match self.kind {
            AttackKind::Special => 0,
            ref k => if self.finesse {
                max(AttackKind::Melee.modifier(mods), AttackKind::Ranged.modifier(mods))
            } else {
                k.modifier(mods)
            }
        }
    }
}

/// A kind of action that a creature can take.
#[derive(Debug,Clone)]
pub enum ActionKind {
    Attack(Rc<Attack>),
    Multiattack(Vec<Rc<Attack>>),
}

/// The full description of an action.
#[derive(Debug,Clone)]
pub struct Action {
    pub name: String,
    pub kind: ActionKind,
}
