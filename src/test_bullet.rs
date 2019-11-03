#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestBullet {
    pub r#macro: Option<TestBulletMacro>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestBulletMacro {
    pub name: Option<String>,
    pub class: Option<String>,
    pub component: Option<TestCompRef>,
}
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct TestCompRef {
    pub r#ref: Option<String>,
}
