use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("macros")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportWeaponFile {
    #[sxs_type_element(rename = "macro")]
    pub r#macro: ExportWeaponMacro,
}
#[xml_element("macro")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportWeaponMacro {
    #[sxs_type_attr]
    pub name: String,
    #[sxs_type_attr]
    pub class: String,
    #[sxs_type_attr]
    pub alias: String,
    #[sxs_type_element(rename = "component")]
    pub component: ExportWeaponCompRef,
    #[sxs_type_element(rename = "properties")]
    pub properties: ExportWeaponProperties,
}
#[xml_element("component")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportWeaponCompRef {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
}
#[xml_element("properties")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportWeaponProperties {
    #[sxs_type_element(rename = "identification")]
    pub identification: ExportIdentification,
    #[sxs_type_element(rename = "bullet")]
    pub bullet: ExportBullet,
    #[sxs_type_element(rename = "rotationspeed")]
    pub rotationspeed: ExportRotSpeed,
    #[sxs_type_element(rename = "rotationacceleration")]
    pub rotationacceleration: ExportRotAccel,
    #[sxs_type_element(rename = "reload")]
    pub reload: ExportReload,
    #[sxs_type_element(rename = "hull")]
    pub hull: ExportHull,
    #[sxs_type_element(rename = "heat")]
    pub heat: ExportHeat,
    #[sxs_type_element(rename = "effects")]
    pub effects: ExportEffects,
}
#[xml_element("identification")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportIdentification {
    #[sxs_type_attr]
    pub name: String,
    #[sxs_type_attr]
    pub basename: String,
    #[sxs_type_attr]
    pub shortname: String,
    #[sxs_type_attr]
    pub makerrace: String,
    #[sxs_type_attr]
    pub description: String,
    #[sxs_type_attr]
    pub mk: String,
}
#[xml_element("bullet")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportBullet {
    #[sxs_type_attr]
    pub class: String,
}
#[xml_element("rotationspeed")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportRotSpeed {
    #[sxs_type_attr]
    pub max: String,
}
#[xml_element("rotationacceleration")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportRotAccel {
    #[sxs_type_attr]
    pub max: String,
}
#[xml_element("reload")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportReload {
    #[sxs_type_attr]
    pub rate: String,
    #[sxs_type_attr]
    pub time: String,
}
#[xml_element("hull")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportHull {
    #[sxs_type_attr]
    pub max: String,
    #[sxs_type_attr]
    pub threshold: String,
    #[sxs_type_attr]
    pub integrated: String,
    #[sxs_type_attr]
    pub hittable: String,
}
#[xml_element("heat")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportHeat {
    #[sxs_type_attr]
    pub overheat: String,
    #[sxs_type_attr]
    pub cooldelay: String,
    #[sxs_type_attr]
    pub coolrate: String,
    #[sxs_type_attr]
    pub reenable: String,
}
#[xml_element("effects")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ExportEffects {
    #[sxs_type_element(rename = "firing")]
    pub firing: Firing,
}
#[xml_element("firing")]
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Firing {
    #[sxs_type_attr]
    pub start: String,
    #[sxs_type_attr]
    pub stop: String,
}