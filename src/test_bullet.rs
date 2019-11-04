#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestBullet {
    pub r#macro: TestBulletMacro,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestBulletMacro {
    pub name: String,
    pub class: String,
    pub component: TestCompRef,
    pub properties: TestBulletProperties,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestCompRef {
    pub r#ref: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestBulletProperties {
    pub ammunition: Option<Ammunition>,
    pub bullet: Option<Bullet>,
    pub heat: Option<Heat>,
    pub reload: Option<Reload>,
    pub damage: Option<Damage>,
    pub effects: Option<Effects>,
    pub weapon: Option<Weapon>,
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
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Heat {
    pub value: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Reload {
    pub rate: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Damage {
    pub value: Option<String>,
    pub shield: Option<String>,
    pub repair: Option<String>,
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
pub struct Weapon {
    pub system: Option<String>,
}
