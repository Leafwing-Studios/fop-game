use std::marker::PhantomData;

/// The canonical label for an affix's behavior.
/// Gear affixes take priority over innate affixes
/// Stores character-agnostic metadata about the affix
/// To avoid data duplication
pub trait AffixLabel: 'static + Send + Sync {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn max_replicates(&self) -> u8;

    fn gear_points(&self) -> Option<i8>;

    fn affix_category(&self) -> AffixCategory;
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AffixCategory {
    Arms,
    Armor,
    Trinket,
    Innate,
}

/// Affixes are added to entities as components to modify their behavior.
/// Each affix will get its own system to perform its logic.
/// This is a sane alternative to a scripting system, as queries are cached.
/// All affixes must be collected, combined and verified
/// against the AffixLabel's max_replicates before being added as a component.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Affix<T: AffixLabel> {
    phantom: PhantomData<T>,
    pub replicates: u8,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FeatureCategory {
    Class,
    Species,
    CombatTalent,
    SkillTalent,
}

/// Features are special affix-like abilities that are granted by non-gear sources
/// They are converted into Affix<T> before being used by the game.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Feature<T: AffixLabel> {
    phantom: PhantomData<T>,
    pub feature_name: String,
    pub feature_category: FeatureCategory,
}

impl<T: AffixLabel> From<Feature<T>> for Affix<T> {
    fn from(_: Feature<T>) -> Self {
        Affix::<T> {
            phantom: PhantomData::<T>::default(),
            replicates: 1,
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AffixTag {
    Air,
    Earth,
    Fire,
    Water,
    Eldritch,
    Arcane,
    Radiant,
    Umbral,
    Primal,
    Decay,
    Electric,
    Corrosive,
}
