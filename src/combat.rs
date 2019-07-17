use crate::types::*;
use crate::creature::*;
use crate::action::*;
use crate::dice::*;
use crate::damage::*;
use crate::basetraits::*;
use crate::util;

/// Expresses how many targets will be affected by an action that targets an area (`Target::Area`).
/// Exactly indicates that only the exact number will be affected; Density indicates how many
/// targets exist per area unit (usually feet). Density is usually fairly low: 0.04 would be one
/// target per 5' square (25 sq. ft.). The default is Exactly(2) (or half of a party of four, it
/// seems) as implicit in 5e DMG, p. 278.
#[derive(Debug,Clone)]
pub enum AreaEffectDensity {
    Exactly(usize),
    Density(f64),
}

impl Default for AreaEffectDensity {
    fn default() -> AreaEffectDensity {
        AreaEffectDensity::Exactly(2)
    }
}

/// The model used for recharges in combat simulation; in the very typical case of "Recharge 5-6",
/// the DMG insists that it's unlikely the exemplar white dragon gets any more than one breath
/// attack over three rounds (5e DMG, p. 278); the odds of getting a recharge after two turns is,
/// in fact, 5 to 4 (greater than one half). To sate the book's calculations, the default is Never,
/// but AfterPassProbability(0.5) is probably reasonable under less artificial circumstances.
#[derive(Debug,Clone)]
pub enum RechargeModel {
    Never,
    AfterPassProbability(f64),
}

impl Default for RechargeModel {
    fn default() -> RechargeModel {
        RechargeModel::Never
    }
}

/// Contains some common settings used for combat calculations
#[derive(Debug,Clone)]
pub struct CombatSettings {
    pub effect_density: AreaEffectDensity,
    pub recharge_model: RechargeModel,
    /// Number of rounds for CR damage calculation; default is 3 (5e DMG, p. 278)
    pub rounds: usize,
}

impl Default for CombatSettings {
    fn default() -> CombatSettings {
        CombatSettings {
            effect_density: Default::default(),
            recharge_model: Default::default(),
            rounds: 3,
        }
    }
}

/// Represents a 1-to-n pair of creatures which are in combat. This structure should be created and
/// used ephemerally; it's merely a convenience for calling methods on it.
#[derive(Debug)]
pub struct CombatPair<'a, 'd ,'s> {
    attacker: &'a Creature,
    defenders: &'d Creature,
    settings: &'s CombatSettings,
}

impl<'a, 'd, 's> CombatPair<'a, 'd, 's> {
    pub fn expected_targets(&self, atk: &Attack) -> usize {
        match &atk.target {
            Target::Exactly(n) => *n,
            Target::Area(a) => match self.settings.effect_density {
                AreaEffectDensity::Exactly(n) => n,
                AreaEffectDensity::Density(f) => (f * a.floor_area()) as usize,
            }
        }
    }

    pub fn expected_single_damage_rolls(&self, atk: &Attack) -> Vec<Damage> {
        atk.dmg_rolls.iter().enumerate().map(|(idx, DamageRoll(ex, k))| {
            Damage(
                util::clamp_isize(0.0f64.max(ex.expected() * self.defenders.damage_factor(*k)) as isize
                                  + if idx == 0 { atk.dmg_bonus } else { 0 }
                ),
                *k
            )
        }).collect()
    }

    pub fn expected_single_damage_sum(&self, atk: &Attack) -> usize {
        self.expected_single_damage_rolls(atk).iter().map(|Damage(u, _)| u).sum()
    }

    pub fn expected_single_damage(&self, atk: &Attack) -> usize {
        let mut dmg = self.expected_single_damage_sum(atk) as isize + atk.dmg_bonus;
        if let Some(Save(sk, sdc, sef)) = &atk.save {
            let dc = sdc.def_class(&self.attacker.mods(), self.attacker.prof_bonus());
            let sm = sk.modifier(&self.defenders.mods());
            match sef {
                SaveEffect::ReducesDamage(amt) => {
                    let p_pass = DiceExpr::Die(Die(20)).prob_pass((dc as isize) - sm);
                    dmg = (p_pass * ((dmg as f64) * amt) + (1.0 - p_pass) * (dmg as f64)) as isize;
                },
            };
        }
        util::clamp_isize(dmg)
    }

    pub fn expected_damage(&self, atk: &Attack) -> usize {
        self.expected_single_damage(atk) * self.expected_targets(atk)
    }

    pub fn attack_modifier(&self, atk: &Attack) -> isize {
        atk.modifier(&self.attacker.mods(), self.attacker.prof_bonus())
    }

    pub fn expected_hit_ac(&self, atk: &Attack) -> AC {
        AC(util::clamp_isize(
            (DiceExpr::Die(Die(20)).expected() + self.attack_modifier(atk) as f64) as isize
        ))
    }
}
