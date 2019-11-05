#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportBullet {
    pub r#macro: ImportBulletMacro,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportBulletMacro {
    pub name: String,
    pub class: String,
    pub component: ImportCompRef,
    pub properties: ImportBulletProperties,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportCompRef {
    pub r#ref: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ImportBulletProperties {
    pub ammunition: Option<Ammunition>,
    pub bullet: Option<Bullet>,
    pub heat: Option<Heat>,
    pub reload: Option<Reload>,
    pub damage: Option<Damage>,
    pub effects: Option<Effects>,
    pub weapon: Option<Weapon>,
    pub areadamage: Option<AreaDamage>,
    pub sounds: Option<Sounds>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Ammunition {
    pub value: Option<String>,
    pub reload: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Bullet {
    pub speed: Option<String>,
    pub lifetime: Option<String>,
    pub amount: Option<String>,
    pub barrelamount: Option<String>,
    pub icon: Option<String>,
    pub timediff: Option<String>,
    pub angle: Option<String>,
    pub maxhits: Option<String>,
    pub ricochet: Option<String>,
    pub scale: Option<String>,
    pub attach: Option<String>,
    pub range: Option<String>,
    pub restitution: Option<String>,
    pub selfdestruct: Option<String>,
    pub delay: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Heat {
    pub value: Option<String>,
    pub initial: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Reload {
    pub rate: Option<String>,
    pub time: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Damage {
    pub value: Option<String>,
    pub shield: Option<String>,
    pub repair: Option<String>,
    pub hull: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub time: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct AreaDamage {
    pub value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Effects {
    pub impact: Option<Impact>,
    pub launch: Option<Launch>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Impact {
    pub r#ref: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Launch {
    pub r#ref: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Sounds {
    pub ambient: Option<Ambient>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Ambient {
    pub r#ref: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Weapon {
    pub system: Option<String>,
}
