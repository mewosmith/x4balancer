#[macro_use]
extern crate serde;
extern crate csv;
extern crate serde_xml_rs;
extern crate yaserde;
// use yaserde::;
use serde_xml_rs::from_str;
use serde_xml_rs::ser::to_writer;
use std::env;
// use std::fmt;
// use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
// use std::process;
#[macro_use]
extern crate simple_xml_serialize;
extern crate simple_xml_serialize_macro;
use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;
use yaserde::ser::to_string;
use yaserde::*;
#[macro_use]
extern crate log;
extern crate xml;
#[macro_use]
extern crate yaserde_derive;

use std::io::prelude::*;
// use std::fs::File;
use std::io::BufWriter;

// use std::io::Write;
// use yaserde::ser::to_string;
use yaserde::YaSerialize;

// csv structs per type
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct BulletCSV {
    macro_name: String,
    class: String,
    component: String,
    ammo_value: String,
    ammo_reload: String,
    bullet_speed: String,
    bullet_lifetime: String,
    bullet_amount: String,
    bullet_barrelamount: String,
    bullet_icon: String,
    bullet_timediff: String,
    bullet_angle: String,
    bullet_maxhits: String,
    bullet_ricochet: String,
    bullet_scale: String,
    bullet_attach: String,
    heat_value: String,
    reload_rate: String,
    damage_value: String,
    damage_shield: String,
    damage_repair: String,
    impact: String,
    launch: String,
    weapon_system: String,
}

#[derive(YaSerialize, PartialEq, Debug)]
#[yaserde(root = "base")]
pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
}

#[derive(YaSerialize, PartialEq, Debug)]
#[yaserde(root = "sub")]
pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String,
}

impl Default for SubStruct {
    fn default() -> SubStruct {
        SubStruct {
            subitem: "".to_string(),
            text: "".to_string(),
        }
    }
}
// impl YaSerialize for XmlStruct {
//   fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {

//   }
// }
mod export_bullet;
mod test_bullet;
fn main() {
    // assert_eq!(
    //     SubStruct::default(),
    //     SubStruct {
    //         subitem: "".to_string(),
    //         text: "".to_string(),
    //     }
    // );
    // let model = XmlStruct {
    //     item: "something".to_string(),
    //     sub_struct: SubStruct {
    //         subitem: "sub_something".to_string(),
    //         text: "text_content".to_string(),
    //     },
    // };

    //  let mut buffer = vec![];

    // buffer.write_all(model.item.as_bytes()).unwrap();
    // let mut buffer = File::create("foo.txt").unwrap();
    // buffer.write(b"hello").unwrap();
    // let mut serializer = ser::Serializer::new_from_writer(buffer);
    // ser::serialize_with_writer(&buffer, buffer);
    // yaserde::YaSerialize::serialize(&serializer, model);

    // let text = ser::Serializer::write(&mut serializer, model).unwrap();
    // let text = write_all(&buffer).unwrap();

    // Writes some prefix of the byte string, not necessarily all of it.
    // yaserde::YaSerialize::serialize(writer: &mut ser::Serializer<W>)
    // yaserde::YaSerialize::serialize(buffer, &mut ser::Serializer<W>).;
    // let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
    // println!("{:?}", text);

    // option<macro> to struct
    // struct to csv
    // csv with placeholders to xml

    let args: Vec<String> = env::args().collect();
    // D:\x4_extract_2.6\assets\fx\weaponFx\macros
    // C:\Users\alby\Desktop\tpwar_targets\x4btarget
    let in_path = Path::new(&args[1]);
    let out_path = Path::new(&args[2]);
    let mut wtr = csv::Writer::from_path(out_path.join("bullet.txt")).expect("make writer");

    if in_path.exists() && in_path.is_dir() {
        for entry in fs::read_dir(&in_path).expect("ERROR! - read_dir") {
            let file = entry.expect("ERROR! - unwrapping entry");
            let file_path = file.path();

            // println!("{:?}", file_path);

            let mut xml_parsed: test_bullet::TestBullet = serde_xml_rs::from_str(
                &fs::read_to_string(&file_path).expect("ERROR! - xml_parsed read_to_string"),
            )
            .unwrap_or_default();

            // let my_point_xml = XMLElement::from(&xml_parsed);
            // println!("{:?}", xml_parsed);
            // println!("{:#?}", my_point_xml);
            // println!("{:#?}", &xml_parsed);
            // let mut csv_struct: BulletCSV = BulletCSV {
            //     macro_name: xml_parsed.r#macro.name.clone(),
            //     class: xml_parsed.r#macro.class.clone(),
            //     // cheese: xml_parsed
            //     //     .r#macro
            //     //     .class
            //     //     .unwrap_or(pizza).clone(),
            //     component: xml_parsed.r#macro.component.r#ref.clone(),
            //     // bullet_speed: xml_parsed.r#macro.properties.bullet.speed.clone(),
            //     // bullet_lifetime: xml_parsed.r#macro.properties.bullet.lifetime.clone(),
            //     // bullet_amount: xml_parsed.r#macro.properties.bullet.amount.clone(),
            //     // bullet_barrelamount: xml_parsed.r#macro.properties.bullet.barrelamount.clone(),
            //     // bullet_icon: xml_parsed.r#macro.properties.bullet.icon.clone(),
            //     // bullet_timediff: xml_parsed.r#macro.properties.bullet.timediff.clone(),
            //     // bullet_angle: xml_parsed.r#macro.properties.bullet.angle.clone(),
            //     // bullet_maxhits: xml_parsed.r#macro.properties.bullet.maxhits.clone(),
            //     // bullet_ricochet: xml_parsed.r#macro.properties.bullet.ricochet.clone(),
            //     // bullet_scale: xml_parsed.r#macro.properties.bullet.scale.clone(),
            //     // bullet_attach: xml_parsed.r#macro.properties.bullet.attach.clone(),
            //     // heat_value: xml_parsed.r#macro.properties.heat.value.clone(),
            //     // reload_rate: xml_parsed.r#macro.properties.reload.rate.clone(),
            //     // damage_value: xml_parsed.r#macro.properties.damage.value.clone(),
            //     // damage_shield: xml_parsed.r#macro.properties.damage.shield.clone(),
            //     // damage_repair: xml_parsed.r#macro.properties.damage.repair.clone(),
            //     // impact: xml_parsed.r#macro.properties.effects.impact.r#ref.clone(),
            //     // launch: xml_parsed.r#macro.properties.effects.launch.r#ref.clone(),
            //     // weapon_system: xml_parsed.r#macro.properties.weapon.system.clone(),
            // };
            wtr.serialize(BulletCSV {
                macro_name: xml_parsed.clone().r#macro.name,
                class: xml_parsed.clone().r#macro.class,
                component: none_default(xml_parsed.clone().r#macro.component.r#ref),
                ammo_value: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .ammunition
                        .unwrap_or_default()
                        .value,
                ),
                ammo_reload: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .ammunition
                        .unwrap_or_default()
                        .reload,
                ),
                bullet_speed: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .speed,
                ),
                bullet_lifetime: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .lifetime,
                ),
                bullet_amount: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .amount,
                ),
                bullet_barrelamount: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .barrelamount,
                ),
                bullet_icon: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .icon,
                ),
                bullet_timediff: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .timediff,
                ),
                bullet_angle: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .angle,
                ),
                bullet_maxhits: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .maxhits,
                ),
                bullet_ricochet: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .ricochet,
                ),
                bullet_scale: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .scale,
                ),
                bullet_attach: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .bullet
                        .unwrap_or_default()
                        .attach,
                ),
                heat_value: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .heat
                        .unwrap_or_default()
                        .value,
                ),
                reload_rate: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .reload
                        .unwrap_or_default()
                        .rate,
                ),
                damage_value: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .damage
                        .unwrap_or_default()
                        .value,
                ),
                damage_shield: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .damage
                        .unwrap_or_default()
                        .shield,
                ),
                damage_repair: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .damage
                        .unwrap_or_default()
                        .repair,
                ),
                impact: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .effects
                        .unwrap_or_default()
                        .impact
                        .unwrap_or_default()
                        .r#ref,
                ),
                launch: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .effects
                        .unwrap_or_default()
                        .launch
                        .unwrap_or_default()
                        .r#ref,
                ),
                weapon_system: none_default(
                    xml_parsed
                        .clone()
                        .r#macro
                        .properties
                        .weapon
                        .unwrap_or_default()
                        .system,
                ),
            })
            .unwrap();
            // &mut csv_struct.class.unwrap_or(pizza);
            // wtr.serialize(&csv_struct).expect("write writer");
        }
    };
}
fn none_default(string: Option<String>) -> String {
    let mut string = string;
    if string == None {
        string = serde::export::Some("placeholder".to_string());
    }
    string.unwrap()
}

// WRITE CSV
// let mut wtr = csv::Writer::from_path(out_path.join("bullet.csv")).expect("make writer");
// wtr.serialize(csv_struct).expect("write writer");

// READ CSV
// let mut rdr = csv::Reader::from_path(out_path.join("bullet.csv")).expect("make reader");
// for result in rdr.deserialize() {
//     let record: BulletCSV = result.unwrap();
// println!("{:?}", record);

// let xml_try: BulletXML = BulletXML {
//     r#macro: BulletMacro {
//         name: record.macro_name,
//         class: record.class,
//         component: ComponentRef {
//             r#ref: record.component,
//         },
//         properties: BulletProperties {
//             bullet: Bullet {
//                 speed: record.bullet_speed,
//                 lifetime: record.bullet_lifetime,
//                 amount: record.bullet_amount,
//                 barrelamount: record.bullet_barrelamount,
//                 icon: record.bullet_icon,
//                 timediff: record.bullet_timediff,
//                 angle: record.bullet_angle,
//                 maxhits: record.bullet_maxhits,
//                 ricochet: record.bullet_ricochet,
//                 scale: record.bullet_scale,
//                 attach: record.bullet_attach,
//             },
//             heat: Heat {
//                 value: record.heat_value,
//             },
//             reload: Reload {
//                 rate: record.reload_rate,
//             },
//             damage: Damage {
//                 value: record.damage_value,
//                 shield: record.damage_shield,
//                 repair: record.damage_repair,
//             },
//             effects: Effects {
//                 impact: Impact {
//                     r#ref: record.impact,
//                 },
//                 launch: Launch {
//                     r#ref: record.launch,
//                 },
//             },
//             weapon: Weapon {
//                 system: record.weapon_system,
//             },
//         },
//     },
// };
// println!("{:?}", xml_try);
// let my_point_xml = XMLElement::from(xml_try);
// println!("{}", my_point_xml);

// }

// WRITE XML
// let my_point_xml = XMLElement::from(xml_parsed);
// println!("{}", my_point_xml);

// fn parse_xml () {

// }
