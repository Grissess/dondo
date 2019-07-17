use std::ops::{Index, IndexMut};
use std::borrow::Borrow;

use crate::dice::Die;
use crate::util;

/// All six ability scores of 5e (5e PHB, p. 173)
#[derive(Debug,Clone)]
pub struct Abilities {
    pub str: isize,
    pub dex: isize,
    pub con: isize,
    pub int: isize,
    pub wis: isize,
    pub cha: isize,
}

/// The six abilities themselves (5e PHB, p. 173)
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum Ability {
    Str, Dex, Con, Int, Wis, Cha,
}

impl Abilities {
    /// Map all abilities through a function (possibly more recognizable as `fmap`).
    pub fn map<F>(&self, mut func: F) -> Abilities 
        where F: FnMut(isize) -> isize
    {
        Abilities {
            str: func(self.str),
            dex: func(self.dex),
            con: func(self.con),
            int: func(self.int),
            wis: func(self.wis),
            cha: func(self.cha),
        }
    }
}

impl Index<Ability> for Abilities {
    type Output = isize;
    
    fn index(&self, idx: Ability) -> &isize {
        match idx {
            Ability::Str => &self.str,
            Ability::Dex => &self.dex,
            Ability::Con => &self.con,
            Ability::Int => &self.int,
            Ability::Wis => &self.wis,
            Ability::Cha => &self.cha,
        }
    }
}

impl IndexMut<Ability> for Abilities {
    fn index_mut(&mut self, idx: Ability) -> &mut isize {
        match idx {
            Ability::Str => &mut self.str,
            Ability::Dex => &mut self.dex,
            Ability::Con => &mut self.con,
            Ability::Int => &mut self.int,
            Ability::Wis => &mut self.wis,
            Ability::Cha => &mut self.cha,
        }
    }
}

/// Ability _scores_ (see 5e PHB, p. 173); just a wrapper around Abilities to avoid confusing
/// units.
#[derive(Debug,Clone)]
pub struct AScores(pub Abilities);

impl Default for AScores {
    fn default() -> AScores {
        AScores(Abilities {
            str: 10, dex: 10, con: 10, int: 10, wis: 10, cha: 10,
        })
    }
}

/// Ability _modifiers_ (see 5e PHB, p. 173); just a wrapper around Abilities to avoid confusing
/// units.
#[derive(Debug,Clone)]
pub struct AMods(pub Abilities);

impl<T> From<T> for AMods where T: Borrow<AScores> {
    fn from(scores: T) -> AMods {
        AMods(scores.borrow().0.map(|x| (x - 10) / 2))
    }
}

impl Default for AMods {
    fn default() -> AMods {
        (&AScores::default()).into()
    }
}

/// Creature size (5e PHB, p. 191)
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Size {
    Tiny, Small, Medium, Large, Huge, Gargantuan
}

impl Size {
    /// 5e DMG, p. 276
    pub fn hit_die(&self) -> Die {
        match self {
            Size::Tiny => Die(4),
            Size::Small => Die(6),
            Size::Medium => Die(8),
            Size::Large => Die(10),
            Size::Huge => Die(12),
            Size::Gargantuan => Die(20),
        }
    }
}

/// Challenge rating (5e DMG, p. 82 and others)
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum CR {
    CR0, CROneEighth, CROneQuarter, CROneHalf,
    CR1, CR2, CR3, CR4, CR5, CR6, CR7, CR8, CR9, CR10,
    CR11, CR12, CR13, CR14, CR15, CR16, CR17, CR18, CR19, CR20,
    CR21, CR22, CR23, CR24, CR25, CR26, CR27, CR28, CR29, CR30,
}

impl Into<f64> for CR {
    fn into(self) -> f64 {
        use CR::*;
        match self {
            CR0 => 0.0, CROneEighth => 0.125, CROneQuarter => 0.25, CROneHalf => 0.5,
            CR1 => 1.0, CR2 => 2.0, CR3 => 3.0, CR4 => 4.0, CR5 => 5.0,
            CR6 => 6.0, CR7 => 7.0, CR8 => 8.0, CR9 => 9.0, CR10 => 10.0,
            CR11 => 11.0, CR12 => 12.0, CR13 => 13.0, CR14 => 14.0, CR15 => 15.0,
            CR16 => 16.0, CR17 => 17.0, CR18 => 18.0, CR19 => 19.0, CR20 => 20.0,
            CR21 => 21.0, CR22 => 22.0, CR23 => 23.0, CR24 => 24.0, CR25 => 25.0,
            CR26 => 26.0, CR27 => 27.0, CR28 => 28.0, CR29 => 29.0, CR30 => 30.0,
        }
    }
}

impl From<f64> for CR {
    fn from(f: f64) -> CR {
        use CR::*;
        match f {
            x if x <= 0.0 => CR0,
            x if x <= 0.125 => CROneEighth,
            x if x <= 0.25 => CROneQuarter,
            x if x <= 0.5 => CROneHalf,
            x if x <= 1.0 => CR1,
            x if x <= 2.0 => CR2,
            x if x <= 3.0 => CR3,
            x if x <= 4.0 => CR4,
            x if x <= 5.0 => CR5,
            x if x <= 6.0 => CR6,
            x if x <= 7.0 => CR7,
            x if x <= 8.0 => CR8,
            x if x <= 9.0 => CR9,
            x if x <= 10.0 => CR10,
            x if x <= 11.0 => CR11,
            x if x <= 12.0 => CR12,
            x if x <= 13.0 => CR13,
            x if x <= 14.0 => CR14,
            x if x <= 15.0 => CR15,
            x if x <= 16.0 => CR16,
            x if x <= 17.0 => CR17,
            x if x <= 18.0 => CR18,
            x if x <= 19.0 => CR19,
            x if x <= 20.0 => CR20,
            x if x <= 21.0 => CR21,
            x if x <= 22.0 => CR22,
            x if x <= 23.0 => CR23,
            x if x <= 24.0 => CR24,
            x if x <= 25.0 => CR25,
            x if x <= 26.0 => CR26,
            x if x <= 27.0 => CR27,
            x if x <= 28.0 => CR28,
            x if x <= 29.0 => CR29,
            _ => CR30,
        }
    }
}

/// A proficiency bonus (5e PHB, p. 12)
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct ProfBonus(pub isize);

/// Hit points (5e PHB, p. 12)
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct HP(pub usize);

/// Armor class (5e PHB, p. 14)
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct AC(pub usize);

/// 5e DMG, p. 274
impl From<CR> for AC {
    fn from(cr: CR) -> AC {
        let crf: f64 = cr.into();
        AC(match crf {
            x if x < 4.0 => 13,
            x if x < 5.0 => 14,
            x if x < 8.0 => 15,
            x if x < 10.0 => 16,
            x if x < 13.0 => 17,
            x if x < 17.0 => 18,
            _ => 19,
        })
    }
}

/// 5e DMG, p. 274
impl From<CR> for ProfBonus {
    fn from(cr: CR) -> ProfBonus {
        let crf: f64 = cr.into();
        ProfBonus(match crf {
            x if x < 5.0 => 2,
            x if x < 9.0 => 3,
            x if x < 13.0 => 4,
            x if x < 17.0 => 5,
            x if x < 21.0 => 6,
            x if x < 25.0 => 7,
            x if x < 29.0 => 8,
            _ => 9,
        })
    }
}

/// 5e DMG, p. 274
impl From<HP> for CR {
    fn from(hp: HP) -> CR {
        use CR::*;
        match hp.0 {
            x if x <= 6 => CR0,
            x if x <= 35 => CROneEighth,
            x if x <= 49 => CROneQuarter,
            x if x <= 70 => CROneHalf,
            x if x <= 85 => CR1,
            x if x <= 100 => CR2,
            x if x <= 115 => CR3,
            x if x <= 130 => CR4,
            x if x <= 145 => CR5,
            x if x <= 160 => CR6,
            x if x <= 175 => CR7,
            x if x <= 190 => CR8,
            x if x <= 205 => CR9,
            x if x <= 220 => CR10,
            x if x <= 235 => CR11,
            x if x <= 250 => CR12,
            x if x <= 265 => CR13,
            x if x <= 280 => CR14,
            x if x <= 295 => CR15,
            x if x <= 310 => CR16,
            x if x <= 325 => CR17,
            x if x <= 340 => CR18,
            x if x <= 355 => CR19,
            x if x <= 400 => CR20,
            x if x <= 445 => CR21,
            x if x <= 490 => CR22,
            x if x <= 535 => CR23,
            x if x <= 580 => CR24,
            x if x <= 625 => CR25,
            x if x <= 670 => CR26,
            x if x <= 715 => CR27,
            x if x <= 760 => CR28,
            x if x <= 805 => CR29,
            _ => CR30,  // Technically capped at 850
        }
    }
}

impl CR {
    /// 5e DMG, p. 274
    fn for_expected_damage(dmg: usize) -> CR {
        use CR::*;
        match dmg {
            x if x <= 1 => CR0,
            x if x <= 3 => CROneEighth,
            x if x <= 5 => CROneQuarter,
            x if x <= 8 => CROneHalf,
            x if x <= 14 => CR1,
            x if x <= 20 => CR2,
            x if x <= 26 => CR3,
            x if x <= 32 => CR4,
            x if x <= 38 => CR5,
            x if x <= 44 => CR6,
            x if x <= 50 => CR7,
            x if x <= 56 => CR8,
            x if x <= 62 => CR9,
            x if x <= 68 => CR10,
            x if x <= 74 => CR11,
            x if x <= 80 => CR12,
            x if x <= 86 => CR13,
            x if x <= 92 => CR14,
            x if x <= 98 => CR15,
            x if x <= 104 => CR16,
            x if x <= 110 => CR17,
            x if x <= 116 => CR18,
            x if x <= 122 => CR19,
            x if x <= 140 => CR20,
            x if x <= 158 => CR21,
            x if x <= 176 => CR22,
            x if x <= 194 => CR23,
            x if x <= 212 => CR24,
            x if x <= 230 => CR25,
            x if x <= 248 => CR26,
            x if x <= 266 => CR27,
            x if x <= 284 => CR28,
            x if x <= 302 => CR29,
            _ => CR30,  // Technically capped at 320
        }
    }

    /// 5e PHB, p. 274; the "to hit bonus" is across any attack with any modifier (Str plus
    /// presumed proficiency for melee, Dex plus prof for ranged, Granting mod for special, etc.)
    pub fn to_hit_bonus(&self) -> isize {
        let crf: f64 = (*self).into();
        match crf {
            x if x < 3.0 => 3,
            x if x < 4.0 => 4,
            x if x < 5.0 => 5,
            x if x < 8.0 => 6,
            x if x < 11.0 => 7,
            x if x < 16.0 => 8,
            x if x < 17.0 => 9,
            x if x < 21.0 => 10,
            x if x < 24.0 => 11,
            x if x < 27.0 => 12,
            x if x < 30.0 => 13,
            _ => 14,
        }
    }

    /// 5e PHB, p. 274; this is save DCs specifically within attacks.
    pub fn save_dc(&self) -> isize {
        let crf: f64 = (*self).into();
        match crf {
            x if x < 4.0 => 13,
            x if x < 5.0 => 14,
            x if x < 8.0 => 15,
            x if x < 11.0 => 16,
            x if x < 13.0 => 17,
            x if x < 17.0 => 18,
            x if x < 21.0 => 19,
            x if x < 24.0 => 20,
            x if x < 27.0 => 21,
            x if x < 30.0 => 22,
            _ => 23,
        }
    }
}

/// (source TODO! Scraped from 5e MM)
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ACKind {
    Normal,
    UnarmoredDefense,
    Armor(usize),
    ArmorDex(usize),
    Natural(usize),
}

impl ACKind {
    pub fn armor_class(&self, mods: &AMods) -> AC {
        AC(match self {
            ACKind::Normal => util::clamp_isize(10 + mods.0.dex),
            ACKind::UnarmoredDefense => util::clamp_isize(10 + mods.0.dex + mods.0.con),
            ACKind::Armor(x) | ACKind::Natural(x) => *x,
            ACKind::ArmorDex(x) => util::clamp_isize((*x as isize) + mods.0.dex),
        })
    }
}
