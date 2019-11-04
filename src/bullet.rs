

// #[xml_element("macros")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct BulletXML {
//     #[sxs_type_element(rename = "macro")]
//     r#macro: BulletMacro,
// }
// #[xml_element("macro")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct BulletMacro {
//     #[sxs_type_attr]
//     name: String,
//     #[sxs_type_attr]
//     class: String,
//     #[sxs_type_element(rename = "component")]
//     component: ComponentRef,
//     #[sxs_type_element(rename = "properties")]
//     properties: BulletProperties,
// }
// #[xml_element("component")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct ComponentRef {
//     #[sxs_type_attr(rename = "ref")]
//     r#ref: String,
// }
// #[xml_element("properties")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct BulletProperties {
//     #[sxs_type_element(rename = "ammunition")]
//     ammunition: Ammunition,
//     #[sxs_type_element(rename = "bullet")]
//     bullet: Bullet,
//     #[sxs_type_element(rename = "heat")]
//     heat: Heat,
//     #[sxs_type_element(rename = "reload")]
//     reload: Reload,
//     #[sxs_type_element(rename = "damage")]
//     damage: Damage,
//     #[sxs_type_element(rename = "effects")]
//     effects: Effects,
//     #[sxs_type_element(rename = "weapon")]
//     weapon: Weapon,
// }
// #[xml_element("ammunition")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Ammunition {
//     #[sxs_type_attr]
//     value: String,
//     #[sxs_type_attr]
//     reload: String,
// }
// #[xml_element("bullet")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Bullet {
//     #[sxs_type_attr]
//     speed: String,
//     #[sxs_type_attr]
//     lifetime: String,
//     #[sxs_type_attr]
//     amount: String,
//     #[sxs_type_attr]
//     barrelamount: String,
//     #[sxs_type_attr]
//     icon: String,
//     #[sxs_type_attr]
//     timediff: String,
//     #[sxs_type_attr]
//     angle: String,
//     #[sxs_type_attr]
//     maxhits: String,
//     #[sxs_type_attr]
//     ricochet: String,
//     #[sxs_type_attr]
//     scale: String,
//     #[sxs_type_attr]
//     attach: String,
// }
// #[xml_element("heat")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Heat {
//     #[sxs_type_attr]
//     value: String,
// }
// #[xml_element("reload")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Reload {
//     #[sxs_type_attr]
//     rate: String,
// }
// #[xml_element("damage")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Damage {
//     #[sxs_type_attr]
//     value: String,
//     #[sxs_type_attr]
//     shield: String,
//     #[sxs_type_attr]
//     repair: String,
// }
// #[xml_element("effects")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Effects {
//     #[sxs_type_element(rename = "impact")]
//     impact: Impact,
//     #[sxs_type_element(rename = "launch")]
//     launch: Launch,
// }
// #[xml_element("impact")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Impact {
//     #[sxs_type_attr(rename = "ref")]
//     r#ref: String,
// }
// #[xml_element("launch")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Launch {
//     #[sxs_type_attr(rename = "ref")]
//     r#ref: String,
// }
// #[xml_element("weapon")]
// #[derive(Deserialize, Serialize, Debug, Default, Clone)]
// struct Weapon {
//     #[sxs_type_attr]
//     system: String,
// }