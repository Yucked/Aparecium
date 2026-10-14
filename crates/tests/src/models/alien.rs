use std::time::Duration;

use chrono::{DateTime, Utc};
use umbriel::{LinkMany, LinkOne, LinkSelf, RecordId, Relate, model};

use crate::{AlienVisitsPlanet, Planet, SpaceShip, Weapon};

/// Every stored type is inferred from the Rust type.
#[model(node, table = "alien")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone)]
pub struct Alien {
    pub id: RecordId<Self>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_string: geo::LineString,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,
    pub tags: Vec<String>,

    #[orm(link_self = "Alien")]
    pub ally: LinkSelf<Alien>,

    #[orm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,

    #[orm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,

    /// Read through the `visits` edge; never stored.
    #[orm(relate(model = "AlienVisitsPlanet", connection = "->visits->planet"))]
    pub planets_to_visit: Relate<Planet<u64>>,
}

/// Every stored type is given explicitly.
#[model(node, table = "alien_with_explicit_attributes")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone)]
pub struct AlienWithExplicitAttributes {
    #[orm(ty = "string")]
    pub id: RecordId<Self>,

    #[orm(ty = "string")]
    pub name: String,

    #[orm(ty = "int")]
    pub age: u8,

    #[orm(ty = "datetime")]
    pub created: DateTime<Utc>,

    #[orm(ty = "duration")]
    pub life_expectancy: Duration,

    #[orm(ty = "geometry<polygon>")]
    pub territory_area: geo::Polygon,

    #[orm(ty = "geometry<point>")]
    pub home: geo::Point,

    #[orm(ty = "array<string>")]
    pub tags: Vec<String>,

    #[orm(
        link_self = "AlienWithExplicitAttributes",
        ty = "record<alien_with_explicit_attributes>"
    )]
    pub ally: LinkSelf<AlienWithExplicitAttributes>,

    #[orm(link_one = "Weapon", ty = "record<weapon>")]
    pub weapon: LinkOne<Weapon>,

    #[orm(link_many = "SpaceShip", ty = "array<record<space_ship>>")]
    pub space_ships: LinkMany<SpaceShip>,
}
