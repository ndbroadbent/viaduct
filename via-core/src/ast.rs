use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub model: Option<Model>,
    pub controller: Option<Controller>,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldAttributes {
    pub serialize: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: TypeRef,
    pub optional: bool,
    pub attributes: FieldAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRef {
    pub name: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Controller {
    pub params: Vec<ParamsProfile>,
    pub respond_with: Vec<String>,
    pub actions: ControllerActions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ControllerActions {
    #[default]
    AutoCrud,
    Manual(Vec<Action>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamsProfile {
    pub name: ParamsKind,
    pub entries: Vec<ParamEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParamsKind {
    Editable,
    Named(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamEntry {
    pub name: String,
    pub optional: bool,
}
