use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("macros")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportBullet {
    #[sxs_type_element(rename = "macro")]
    pub r#macro: ExportBulletMacro,
}
#[xml_element("macro")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportBulletMacro {
    #[sxs_type_attr]
    pub name: String,
    #[sxs_type_attr]
    pub class: String,
    #[sxs_type_element(rename = "component")]
    pub component: ExportCompRef,
}
#[xml_element("component")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportCompRef {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
}
