use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("ware")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportWareEntry {
    #[sxs_type_attr]
    pub id: String,
    #[sxs_type_attr]
    pub name: String,
    #[sxs_type_attr]
    pub description: String,
    #[sxs_type_attr]
    pub group: String,
    #[sxs_type_attr]
    pub transport: String,
    #[sxs_type_attr]
    pub volume: String,
    #[sxs_type_attr]
    pub tags: String,
    #[sxs_type_element(rename = "price")]
    pub price: ExportPrice,
    // #[sxs_type_element(rename = "production")]
    // pub production: String,
    #[sxs_type_element(rename = "component")]
    pub component: ExportComponent,
    #[sxs_type_element(rename = "restriction")]
    pub restriction: ExportRestriction,
    #[sxs_type_element(rename = "use")]
    pub r#use: ExportUse,
    // #[sxs_type_element(rename = "owner")]
    // pub owner: Vec<ExportOwner>,
    // pub owner: String,
}
#[xml_element("price")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportPrice {
    #[sxs_type_attr]
    pub min: String,
    #[sxs_type_attr]
    pub average: String,
    #[sxs_type_attr]
    pub max: String,
}
#[xml_element("production")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportProduction {
    #[sxs_type_attr]
    pub time: String,
    #[sxs_type_attr]
    pub amount: String,
    #[sxs_type_attr]
    pub method: String,
    #[sxs_type_attr]
    pub name: String,
    #[sxs_type_element(rename = "primary")]
    pub primary: ExportPrimary,
}
#[xml_element("primary")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportPrimary {
    pub ware: Vec<ExportWare>,
}
#[xml_element("ware")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportWare {
    #[sxs_type_attr]
    pub ware: String,
    #[sxs_type_attr]
    pub amount: String,
}
#[xml_element("component")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportComponent {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
    #[sxs_type_attr]
    pub amount: String,
}
#[xml_element("restriction")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportRestriction {
    #[sxs_type_attr]
    pub licence: String,
}
#[xml_element("use")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportUse {
    #[sxs_type_attr]
    pub threshold: String,
}
#[xml_element("owner")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportOwner {
    #[sxs_type_attr]
    pub faction: String,
}