use std::time::Duration;

use chrono::{DateTime, Utc};
use umbriel::{LinkMany, LinkOne, Node, RecordId, model, types::Geometry};

use crate::{Alien, Planet, SpaceShip, Weapon};

/// An edge whose stored types are inferred, one field per geometry kind.
#[model(edge, table = "visits")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone)]
pub struct Visits<In: Node, Out>
where
    Out: Node,
{
    pub id: RecordId<Self>,
    #[orm(link_one = "In")]
    pub r#in: LinkOne<In>,

    #[orm(link_one = "Out")]
    pub out: LinkOne<Out>,
    pub hair_color: Option<String>,

    #[orm(ty = "duration")]
    pub time_visited: Duration,

    #[orm(link_one = "Planet<u64>")]
    pub mana: LinkOne<Planet<u64>>,
    pub name: String,
    pub age: u8,
    pub created: DateTime<Utc>,
    pub life_expectancy: Duration,
    pub line_string: geo::LineString,
    pub multi_line_string: geo::MultiLineString,
    pub polygon: geo::Polygon,
    pub multi_polygon: geo::MultiPolygon,
    pub point: geo::Point,
    pub multi_point: geo::MultiPoint,
    pub geometry_collection: Geometry,
    pub territory_area: geo::Polygon,
    pub home: geo::Point,

    #[orm(ty = "geometry<point>")]
    pub point_explicit: geo::Point,

    #[orm(ty = "geometry<multipoint>")]
    pub multi_point_explicit: geo::MultiPoint,

    #[orm(ty = "geometry<line>")]
    pub line_string_explicit: geo::LineString,

    #[orm(ty = "geometry<multiline>")]
    pub multi_line_string_explicit: geo::MultiLineString,

    #[orm(ty = "geometry<polygon>")]
    pub polygon_explicit: geo::Polygon,

    #[orm(ty = "geometry<multipolygon>")]
    pub multi_polygon_explicit: geo::MultiPolygon,

    #[orm(ty = "geometry<collection>")]
    pub geometry_collection_explicit: Geometry,

    pub tags: Vec<String>,
    #[orm(link_one = "Weapon")]
    pub weapon: LinkOne<Weapon>,
    #[orm(link_many = "SpaceShip")]
    pub space_ships: LinkMany<SpaceShip>,
}

/// Connects an alien to the planets it visits.
pub type AlienVisitsPlanet = Visits<Alien, Planet<u64>>;

/// An edge whose `in` and `out` have explicit types.
#[model(edge, table = "visits_explicit")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone)]
pub struct VisitsExplicit<In: Node, Out: Node> {
    #[orm(ty = "int")]
    pub id: RecordId<Self>,

    #[surreal(rename = "in")]
    #[orm(link_one = "In", ty = "record")]
    pub in_: LinkOne<In>,

    #[orm(link_one = "Out", ty = "record")]
    pub out: LinkOne<Out>,

    #[orm(ty = "duration")]
    pub time_visited: Duration,
}

pub type AlienVisitsPlanetExplicit = VisitsExplicit<Alien, Planet<u64>>;
