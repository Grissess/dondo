/// 5e PHB, p. 196
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum DamageKind {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

pub struct Damage(pub usize, pub DamageKind);
