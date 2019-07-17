use crate::types::*;
use crate::basetraits::*;
use crate::action::*;
use crate::damage::DamageKind;
use crate::dice::DiceExpr;

use std::collections::HashSet;
use std::rc::Rc;

/// A basic creature, without CR or prof bonus, as that takes nontrivial effort to compute.
#[derive(Debug,Clone)]
pub struct BaseCreature {
    pub ascores: AScores,
    pub ac_kind: ACKind,
    pub actions: Vec<Action>,
    pub size: Size,
    pub hit_dice: usize,
    pub immunities: HashSet<DamageKind>,
    pub resistances: HashSet<DamageKind>,
    pub vulnerabilities: HashSet<DamageKind>,
}

impl BaseCreature {
    /// 5e PHB, p. 197 (resistance and vulnerability)
    pub fn damage_factor(&self, k: DamageKind) -> f64 {
        let mut fac = 1.0f64;
        if self.immunities.contains(&k) {
            return 0.0;
        }
        if self.resistances.contains(&k) {
            fac *= 0.5;
        }
        if self.vulnerabilities.contains(&k) {
            fac *= 2.0;
        }
        fac
    }

    pub fn mods(&self) -> AMods {
        (&self.ascores).into()
    }

    pub fn armor_class(&self) -> AC {
        self.ac_kind.armor_class(&self.mods())
    }

    pub fn expected_hit_points(&self) -> HP {
        use DiceExpr::*;
        HP(
            (Times(self.hit_dice, Rc::new(
                Plus(
                    Rc::new(Die(self.size.hit_die())),
                    Rc::new(Const(self.mods().0.con as isize)),
                )
            ))).expected() as usize
        )
    }

    /// Fictitiously make this BaseCreature into a Creature with the given CR. No guarantee is
    /// given as to that value's accuracy, which can have effect (through the proficiency bonus) on
    /// other calculations downstream.
    pub fn with_cr(self, cr: CR) -> Creature {
        Creature { base: self, cr: cr }
    }
}

/// A Creature is a BaseCreature which has a cached CR and proficiency
#[derive(Debug,Clone)]
pub struct Creature {
    base: BaseCreature,
    cr: CR,
}

impl Creature {
    pub fn damage_factor(&self, k: DamageKind) -> f64 {
        self.base.damage_factor(k)
    }

    pub fn mods(&self) -> AMods {
        self.base.mods()
    }

    pub fn prof_bonus(&self) -> ProfBonus {
        self.cr.into()
    }
}
