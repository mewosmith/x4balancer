#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportWares {
    pub ware: Option<ImportWareEntry>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportWareEntry {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub group: Option<String>,
    pub transport: Option<String>,
    pub volume: Option<String>,
    pub tags: Option<String>,
    pub price: Option<Price>,
    pub production: Option<Vec<Production>>,
    pub component: Option<Component>,
    pub restriction: Option<Restriction>,
    pub r#use: Option<Use>,
    pub owner: Option<Vec<Owner>>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Price {
    pub min: Option<String>,
    pub average: Option<String>,
    pub max: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Production {
    pub time: Option<String>,
    pub amount: Option<String>,
    pub method: Option<String>,
    pub name: Option<String>,
    pub primary: Option<Primary>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Primary {
    pub ware: Option<Vec<Ware>>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Ware {
    pub ware: Option<String>,
    pub amount: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Component {
    pub r#ref: Option<String>,
    pub amount: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Restriction {
    pub licence: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Use {
    pub threshold: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Owner {
    pub faction: Option<String>,
}