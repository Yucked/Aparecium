use chrono::{DateTime, Utc};
use umbriel::{RecordId, model};

#[model(node, table = "weapon")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Weapon {
    pub id: RecordId<Self>,
    pub name: String,
    #[orm(ty = "float")]
    pub strength: Strength,
    pub created: DateTime<Utc>,
    #[orm(nest_object = "Rocket")]
    pub rocket: Rocket,
}

pub type Strength = f64;

/// An earlier shape of `Weapon`, stored in the same table: the records it writes have fields
/// `Weapon` no longer declares.
#[model(node, table = "weapon")]
#[orm(relax_table)]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct WeaponOld {
    pub id: RecordId<Self>,
    pub name: String,
    #[orm(ty = "float")]
    pub strength: Strength,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
    #[orm(nest_object = "Rocket")]
    pub rocket: Rocket,
}

#[model(object)]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Rocket {
    pub name: String,
    pub strength: u64,
    pub nice: bool,
    pub bunch_of_other_fields: i32,
    pub created: DateTime<Utc>,
}

#[model(node, table = "weapon_stats")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct WeaponStats {
    pub id: RecordId<Self>,
    pub average_strength: f64,
}

#[model(node, table = "account")]
#[derive(Debug, Clone)]
pub struct Account {
    pub id: RecordId<Self>,
    pub balance: f64,
}

#[model(node, table = "balance")]
#[derive(Debug, Clone)]
pub struct Balance {
    pub id: RecordId<Self>,
    pub amount: f64,
}
