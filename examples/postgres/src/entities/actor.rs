use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "actor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::film_actor::Entity")]
    FilmActor,
}

impl Related<super::film_actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmActor.def()
    }
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        super::film_actor::Relation::Film.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::film_actor::Relation::Actor.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::film_actor::Entity")]
    FilmActor,
    #[sea_orm(entity = "super::film::Entity")]
    Film,
}
