use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("macros")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletFile {
    #[sxs_type_element(rename = "macro")]
    pub r#macro: ExportBulletMacro,
}
#[xml_element("macro")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletMacro {
    #[sxs_type_attr]
    pub name: String,
    #[sxs_type_attr]
    pub class: String,
    #[sxs_type_element(rename = "component")]
    pub component: ExportBulletCompRef,
    #[sxs_type_element(rename = "properties")]
    pub properties: ExportBulletProperties,
}
#[xml_element("component")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletCompRef {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
}
#[xml_element("properties")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletProperties {
    #[sxs_type_element(rename = "ammunition")]
    pub ammunition: ExportBulletAmmunition,
    #[sxs_type_element(rename = "bullet")]
    pub bullet: ExportBulletLine,
    #[sxs_type_element(rename = "heat")]
    pub heat: ExportBulletHeat,
    #[sxs_type_element(rename = "reload")]
    pub reload: ExportBulletReload,
    #[sxs_type_element(rename = "damage")]
    pub damage: ExportBulletDamage,
    #[sxs_type_element(rename = "effects")]
    pub effects: ExportBulletEffects,
    #[sxs_type_element(rename = "weapon")]
    pub weapon: ExportBulletWeapon,
    #[sxs_type_element(rename = "areadamage")]
    pub areadamage: ExportAreaDamage,
    #[sxs_type_element(rename = "sounds")]
    pub sounds: ExportSounds,
}
#[xml_element("ammunition")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletAmmunition {
    #[sxs_type_attr]
    pub value: String,
    #[sxs_type_attr]
    pub reload: String,
}
#[xml_element("bullet")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletLine {
    #[sxs_type_attr]
    pub speed: String,
    #[sxs_type_attr]
    pub lifetime: String,
    #[sxs_type_attr]
    pub amount: String,
    #[sxs_type_attr]
    pub barrelamount: String,
    #[sxs_type_attr]
    pub icon: String,
    #[sxs_type_attr]
    pub timediff: String,
    #[sxs_type_attr]
    pub angle: String,
    #[sxs_type_attr]
    pub maxhits: String,
    #[sxs_type_attr]
    pub ricochet: String,
    #[sxs_type_attr]
    pub scale: String,
    #[sxs_type_attr]
    pub attach: String,
    #[sxs_type_attr]
    pub range: String,
    #[sxs_type_attr]
    pub restitution: String,
    #[sxs_type_attr]
    pub selfdestruct: String,
    #[sxs_type_attr]
    pub delay: String,
}
#[xml_element("heat")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletHeat {
    #[sxs_type_attr]
    pub value: String,
    #[sxs_type_attr]
    pub initial: String,
}
#[xml_element("reload")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletReload {
    #[sxs_type_attr]
    pub rate: String,
    #[sxs_type_attr]
    pub time: String,
}
#[xml_element("damage")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletDamage {
    #[sxs_type_attr]
    pub value: String,
    #[sxs_type_attr]
    pub shield: String,
    #[sxs_type_attr]
    pub repair: String,
    #[sxs_type_attr]
    pub hull: String,
    #[sxs_type_attr]
    pub min: String,
    #[sxs_type_attr]
    pub max: String,
    #[sxs_type_attr]
    pub time: String,
}
#[xml_element("effects")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletEffects {
    #[sxs_type_element(rename = "impact")]
    pub impact: ExportImpact,
    #[sxs_type_element(rename = "launch")]
    pub launch: ExportLaunch,
}
#[xml_element("impact")]
#[derive(Debug, Default, Clone)]
pub struct ExportImpact {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
}
#[xml_element("launch")]
#[derive(Debug, Default, Clone)]
pub struct ExportLaunch {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
}
#[xml_element("weapon")]
#[derive(Debug, Default, Clone)]
pub struct ExportBulletWeapon {
    #[sxs_type_attr]
    pub system: String,
}
#[xml_element("areadamage")]
#[derive(Debug, Default, Clone)]
pub struct ExportAreaDamage {
    #[sxs_type_attr]
    pub value: String,
}
#[xml_element("sounds")]
#[derive(Debug, Default, Clone)]
pub struct ExportSounds {
    #[sxs_type_element(rename = "ambient")]
    pub ambient: ExportAmbient,
}
#[xml_element("ambient")]
#[derive(Debug, Default, Clone)]
pub struct ExportAmbient {
    #[sxs_type_attr(rename = "ref")]
    pub r#ref: String,
}
