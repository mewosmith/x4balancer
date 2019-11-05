#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportWeapon {
    pub r#macro: ImportWeaponMacro,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportWeaponMacro {
    pub name: String,
    pub class: String,
    pub alias: Option<String>,
    pub component: ImportCompRef,
    pub properties: ImportWeaponProperties,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportCompRef {
    pub r#ref: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportWeaponProperties {
    pub identification: Option<Identification>,
    pub bullet: Option<Bullet>,
    pub rotationspeed: Option<RotationSpeed>,
    pub rotationacceleration: Option<RotationAcceleration>,
    pub reload: Option<Reload>,
    pub hull: Option<Hull>,
    pub heat: Option<Heat>,
    pub effects: Option<Effects>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Identification {
    pub name: Option<String>,
    pub basename: Option<String>,
    pub shortname: Option<String>,
    pub makerrace: Option<String>,
    pub description: Option<String>,
    pub mk: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Bullet {
    pub class: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RotationSpeed {
    pub max: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct RotationAcceleration {
    pub max: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Reload {
    pub rate: Option<String>,
    pub time: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Hull {
    pub max: Option<String>,
    pub threshold: Option<String>,
    pub integrated: Option<String>,
    pub hittable: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Heat {
    pub overheat: Option<String>,
    pub cooldelay: Option<String>,
    pub coolrate: Option<String>,
    pub reenable: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Effects {
    pub firing: Option<Firing>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Firing {
    pub start: Option<String>,
    pub stop: Option<String>,
}