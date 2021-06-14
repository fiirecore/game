use deps::str::TinyStr16;

pub mod world;

pub type ScriptId = TinyStr16;

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MatchCondition {

    NotAny,
    None,
    Any,
    All,

}