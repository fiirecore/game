pub mod world;

pub type ScriptId = String;

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MatchCondition {

    NotAny,
    None,
    Any,
    All,

}