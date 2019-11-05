#[macro_use]
extern crate serde;
extern crate csv;
extern crate serde_xml_rs;
use regex::Regex;
use std::env;
use std::any::{Any, TypeId};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::process;
extern crate simple_xml_serialize;
extern crate simple_xml_serialize_macro;
use simple_xml_serialize::XMLElement;
// use simple_xml_serialize_macro::xml_element;
extern crate log;
extern crate xml;

#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    csv_file: String,
    output_csv: String,
    input_xml: String,
    input_bullet: String,
    export_xml: String,
    wares_file: String,
}
mod wares_import;
mod wares_export;
mod bullet_export;
mod bullet_import;
mod weapon_export;
mod weapon_import;
mod csv_struct;
fn main() {
    // option<macro> to struct
    // struct to csv
    // csv with placeholders to xml

    let args: Vec<String> = env::args().collect();
    let config: Config = toml::from_str(include_str!("config.toml")).unwrap();

    match args[1].as_str() {
        "to_xml" => csv_to_xml(Path::new(&config.csv_file), Path::new(&config.export_xml)),
        "to_csv" => xml_to_csv(Path::new(&config.input_xml), Path::new(&config.output_csv), Path::new(&config.input_bullet), Path::new(&config.wares_file)),
        _ => println!("invalid args \"to_xml\" or \"to_csv\""),
    }
}
fn none_default(string: Option<String>) -> String {
    let mut string = string;
    if string == None {
        string = serde::export::Some("placeholder".to_string());
    }
    string.unwrap()
}
fn clean_string(string: String) -> String {
    let mut clean = string;
    if clean.contains("<diff>") {
        let re = Regex::new("<.*diff.*>").unwrap();
        clean = re.replace(&clean, "").to_string();
        let re = Regex::new("<.*replace.*>").unwrap();
        clean = re.replace(&clean, "").to_string();
        let re = Regex::new("<.*add.*>").unwrap();
        clean = re.replace(&clean, "").to_string();
    }
    if clean.contains("=\"placeholder\"") {
        let re = Regex::new("\\s[a-z]*=\"placeholder\"").unwrap();
        for _ in re.captures_iter(&clean.clone()) {
            clean = re.replace(&clean, "").to_string();
        }
    }
    clean
}

fn xml_to_csv(in_path: &Path, out_path: &Path, bullet_path: &Path, ware_path: &Path) {
    use csv_struct::*;
    let mut csv_writer = csv::Writer::from_path(out_path.join("x4_balancer.csv")).expect("make writer");

    // open every file in each folder (presumably all weapons)
    // parse weapons into struct
    // open corresponding bullet file
    // parse bullet into struct
    // find ware entry
    // parse ware entry
    // add all to csvstruct

    if in_path.exists() && in_path.is_dir() {
        for entry in fs::read_dir(&in_path).expect("ERROR! - read_dir") {
            // read and parse each weapon file
            let file_path = entry.expect("ERROR! - unwrapping entry").path();
            let read_string = clean_string(fs::read_to_string(&file_path).expect("ERROR! - weapon_parsed read_to_string"));
            let weapon_parsed: weapon_import::ImportWeapon = serde_xml_rs::from_str(&read_string).unwrap_or_default();
            // find the bullet filename and path from the bullet class listed on the weapon
            let bullet_path = bullet_path.join(format!("{}.xml", none_default(weapon_parsed.clone().r#macro.properties.bullet.unwrap_or_default().class)));
            // read and parse bullet
            let bullet_string = clean_string(fs::read_to_string(&bullet_path).expect("ERROR! - bullet_parsed read_to_string"));
            let bullet_parsed: bullet_import::ImportBullet = serde_xml_rs::from_str(&bullet_string).unwrap_or_default();

            // find matching ware from the weapon macro name itself
            let ware_read = fs::read_to_string(&ware_path).unwrap();
            let mut ware_new = "".to_string();
            for ware in ware_read.split_terminator("</ware>") {
                if ware.contains(&weapon_parsed.clone().r#macro.name) == true {
                    ware_new.push_str(&ware);
                    ware_new.push_str("\n</ware>");
                }
            }
            let ware_parsed: wares_import::ImportWareEntry = serde_xml_rs::from_str(&ware_new).unwrap_or_default();
            // println!("{:?}", ware_parsed);
            let mut owners = "".to_string();
            let owners_vec = ware_parsed.clone().owner.unwrap_or_default();
                for owner in owners_vec {
                    let faction = none_default(owner.faction);
                    owners.push_str(&faction);
                    owners.push_str(" ");
                }
            // println!("{:?}", owners);
            let mut production = "".to_string();
            if ware_new.contains("production") {
                let re = Regex::new(r"(?s)<production.*</production>").expect("setting up regex");
                
                let prod = re.captures(&ware_new).expect("finding production");
                production.push_str(&prod[0]);
            }            

            // add this weapon/bullet/ware combo to the csv writer
            csv_writer.serialize(CSVStruct {
                // weapon values
                macro_name: weapon_parsed.clone().r#macro.name,
                class: weapon_parsed.clone().r#macro.class,
                alias: none_default(weapon_parsed.clone().r#macro.alias),
                component: none_default(weapon_parsed.clone().r#macro.component.r#ref),
                id_name: none_default(weapon_parsed.clone().r#macro.properties.identification.unwrap_or_default().name),
                id_basename: none_default(weapon_parsed.clone().r#macro.properties.identification.unwrap_or_default().basename),
                id_shortname: none_default(weapon_parsed.clone().r#macro.properties.identification.unwrap_or_default().shortname),
                makerrace: none_default(weapon_parsed.clone().r#macro.properties.identification.unwrap_or_default().makerrace),
                description: none_default(weapon_parsed.clone().r#macro.properties.identification.unwrap_or_default().description),
                mk: none_default(weapon_parsed.clone().r#macro.properties.identification.unwrap_or_default().mk),
                bullets_used: none_default(weapon_parsed.clone().r#macro.properties.bullet.unwrap_or_default().class),
                wreload_rate: none_default(weapon_parsed.clone().r#macro.properties.reload.unwrap_or_default().rate),
                wreload_time: none_default(weapon_parsed.clone().r#macro.properties.reload.unwrap_or_default().time),
                rot_speed_max: none_default(weapon_parsed.clone().r#macro.properties.rotationspeed.unwrap_or_default().max),
                rot_accel_max: none_default(weapon_parsed.clone().r#macro.properties.rotationacceleration.unwrap_or_default().max),
                hull_max: none_default(weapon_parsed.clone().r#macro.properties.hull.unwrap_or_default().max),
                hull_threshold: none_default(weapon_parsed.clone().r#macro.properties.hull.unwrap_or_default().threshold),
                hull_integrated: none_default(weapon_parsed.clone().r#macro.properties.hull.unwrap_or_default().integrated),
                hull_hittable: none_default(weapon_parsed.clone().r#macro.properties.hull.unwrap_or_default().hittable),
                heat_overheat: none_default(weapon_parsed.clone().r#macro.properties.heat.unwrap_or_default().overheat),
                heat_cooldelay: none_default(weapon_parsed.clone().r#macro.properties.heat.unwrap_or_default().cooldelay),
                heat_coolrate: none_default(weapon_parsed.clone().r#macro.properties.heat.unwrap_or_default().coolrate),
                heat_reenable: none_default(weapon_parsed.clone().r#macro.properties.heat.unwrap_or_default().reenable),
                firing_start_fx: none_default(weapon_parsed.clone().r#macro.properties.effects.unwrap_or_default().firing.unwrap_or_default().start),
                firing_stop_fx: none_default(weapon_parsed.clone().r#macro.properties.effects.unwrap_or_default().firing.unwrap_or_default().stop),
                // bullet values
                bullet_macro_name: bullet_parsed.clone().r#macro.name,
                bullet_class: bullet_parsed.clone().r#macro.class,
                bullet_component: none_default(bullet_parsed.clone().r#macro.component.r#ref),
                ammo_value: none_default(bullet_parsed.clone().r#macro.properties.ammunition.unwrap_or_default().value),
                ammo_reload: none_default(bullet_parsed.clone().r#macro.properties.ammunition.unwrap_or_default().reload),
                bullet_speed: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().speed),
                bullet_lifetime: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().lifetime),
                bullet_amount: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().amount),
                bullet_barrelamount: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().barrelamount),
                bullet_icon: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().icon),
                bullet_timediff: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().timediff),
                bullet_angle: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().angle),
                bullet_maxhits: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().maxhits),
                bullet_ricochet: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().ricochet),
                bullet_scale: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().scale),
                bullet_attach: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().attach),
                bullet_range: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().range),
                bullet_restitution: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().restitution),
                bullet_selfdestruct: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().selfdestruct),
                bullet_delay: none_default(bullet_parsed.clone().r#macro.properties.bullet.unwrap_or_default().delay),
                heat_value: none_default(bullet_parsed.clone().r#macro.properties.heat.unwrap_or_default().value),
                heat_initial: none_default(bullet_parsed.clone().r#macro.properties.heat.unwrap_or_default().initial),
                reload_rate: none_default(bullet_parsed.clone().r#macro.properties.reload.unwrap_or_default().rate),
                reload_time: none_default(bullet_parsed.clone().r#macro.properties.reload.unwrap_or_default().time),
                damage_value: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().value),
                damage_shield: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().shield),
                damage_repair: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().repair),
                damage_min: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().min),
                damage_max: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().max),
                damage_time: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().time),
                damage_hull: none_default(bullet_parsed.clone().r#macro.properties.damage.unwrap_or_default().hull),
                impact: none_default(bullet_parsed.clone().r#macro.properties.effects.unwrap_or_default().impact.unwrap_or_default().r#ref),
                launch: none_default(bullet_parsed.clone().r#macro.properties.effects.unwrap_or_default().launch.unwrap_or_default().r#ref),
                weapon_system: none_default(bullet_parsed.clone().r#macro.properties.weapon.unwrap_or_default().system),
                area_damage: none_default(bullet_parsed.clone().r#macro.properties.areadamage.unwrap_or_default().value),
                sound_ambient: none_default(bullet_parsed.clone().r#macro.properties.sounds.unwrap_or_default().ambient.unwrap_or_default().r#ref),
                ware_id: none_default(ware_parsed.clone().id),
                ware_name: none_default(ware_parsed.clone().name),
                ware_desc: none_default(ware_parsed.clone().description),
                ware_group: none_default(ware_parsed.clone().group),
                ware_transport: none_default(ware_parsed.clone().transport),
                ware_volume: none_default(ware_parsed.clone().volume),
                ware_tags: none_default(ware_parsed.clone().tags),
                ware_price_min: none_default(ware_parsed.clone().price.unwrap_or_default().min),
                ware_price_avg: none_default(ware_parsed.clone().price.unwrap_or_default().average),
                ware_price_max: none_default(ware_parsed.clone().price.unwrap_or_default().max),
                ware_production: production,
                // ware_prod_time: 
                // ware_prod_amount: 
                // ware_prod_method: 
                // ware_prod_name: 
                // ware_prod_wares: 
                // ware_prod_amts: 
                ware_comp_ref: none_default(ware_parsed.clone().component.unwrap_or_default().r#ref),
                ware_comp_amt: none_default(ware_parsed.clone().component.unwrap_or_default().amount),
                ware_license: none_default(ware_parsed.clone().restriction.unwrap_or_default().licence),
                ware_use_threshold: none_default(ware_parsed.clone().r#use.unwrap_or_default().threshold),
                ware_owners: owners,
            })
            .unwrap();
        }
    };
}

fn csv_to_xml(in_path: &Path, out_path: &Path) {
    use csv_struct::*;
    use bullet_export::*;
    use weapon_export::*;
    use wares_export::*;
    let mut rdr = csv::Reader::from_path(in_path).expect("make reader");
    let mut ware_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<wares>\n".to_string();
    for result in rdr.deserialize() {
        // pull out one record at a time
        let record: CSVStruct = result.unwrap();
        // put weapon values into export struct
        let weapon_record: ExportWeaponFile = ExportWeaponFile {
            r#macro: ExportWeaponMacro {
                name: record.clone().macro_name,
                class: record.clone().class,
                alias: record.clone().alias,
                component: ExportWeaponCompRef { r#ref: record.clone().component },
                properties: ExportWeaponProperties {
                    identification: ExportIdentification {
                        name: record.clone().id_name,
                        basename: record.clone().id_basename,
                        shortname: record.clone().id_shortname,
                        makerrace: record.clone().makerrace,
                        description: record.clone().description,
                        mk: record.clone().mk
                    },
                    bullet: ExportBullet { class: record.clone().bullets_used },
                    rotationspeed: ExportRotSpeed { max: record.clone().rot_speed_max },
                    rotationacceleration: ExportRotAccel { max: record.clone().rot_accel_max },
                    reload: ExportReload {
                        rate: record.clone().wreload_rate,
                        time: record.clone().wreload_time
                    },
                    hull: ExportHull {
                        max: record.clone().hull_max,
                        threshold: record.clone().hull_threshold,
                        integrated: record.clone().hull_integrated,
                        hittable: record.clone().hull_hittable
                    },
                    heat: ExportHeat {
                        overheat: record.clone().heat_overheat,
                        cooldelay: record.clone().heat_cooldelay,
                        coolrate: record.clone().heat_coolrate,
                        reenable: record.clone().heat_reenable
                    },
                    effects: ExportEffects { 
                        firing: Firing {
                            start: record.clone().firing_start_fx,
                            stop: record.clone().firing_stop_fx
                        }
                    }
                }
            }
        };
        // put bullet values into export struct
        let bullet_record: ExportBulletFile = ExportBulletFile {
            r#macro: ExportBulletMacro {
                name: record.clone().bullet_macro_name,
                class: record.clone().bullet_class,
                component: ExportBulletCompRef { r#ref: record.clone().bullet_component },
                properties: ExportBulletProperties {
                    ammunition: ExportBulletAmmunition {
                        value: record.clone().ammo_value,
                        reload: record.clone().ammo_reload,
                    },
                    bullet: ExportBulletLine {
                        speed: record.clone().bullet_speed,
                        lifetime: record.clone().bullet_lifetime,
                        amount: record.clone().bullet_amount,
                        barrelamount: record.clone().bullet_barrelamount,
                        icon: record.clone().bullet_icon,
                        timediff: record.clone().bullet_timediff,
                        angle: record.clone().bullet_angle,
                        maxhits: record.clone().bullet_maxhits,
                        ricochet: record.clone().bullet_ricochet,
                        scale: record.clone().bullet_scale,
                        attach: record.clone().bullet_attach,
                        range: record.clone().bullet_range,
                        restitution: record.clone().bullet_restitution,
                        selfdestruct: record.clone().bullet_selfdestruct,
                        delay: record.clone().bullet_delay,
                    },
                    heat: ExportBulletHeat {
                        value: record.clone().heat_value,
                        initial: record.clone().heat_initial,
                    },
                    reload: ExportBulletReload {
                        rate: record.clone().reload_rate,
                        time: record.clone().reload_time,
                    },
                    damage: ExportBulletDamage {
                        value: record.clone().damage_value,
                        shield: record.clone().damage_shield,
                        repair: record.clone().damage_repair,
                        hull: record.clone().damage_hull,
                        min: record.clone().damage_min,
                        max: record.clone().damage_max,
                        time: record.clone().damage_time,
                    },
                    effects: ExportBulletEffects {
                        impact: ExportImpact { r#ref: record.clone().impact },
                        launch: ExportLaunch { r#ref: record.clone().launch },
                    },
                    weapon: ExportBulletWeapon { system: record.clone().weapon_system },
                    areadamage: ExportAreaDamage { value: record.clone().area_damage },
                    sounds: ExportSounds {
                        ambient: ExportAmbient { r#ref: record.clone().sound_ambient },
                    },
                },
            },
        };
        // put ware values into export struct
        let ware_record: ExportWareEntry = ExportWareEntry {
            id: record.clone().ware_id,
            name: record.clone().ware_name,
            description: record.clone().ware_desc,
            group: record.clone().ware_group,
            transport: record.clone().ware_transport,
            volume: record.clone().ware_volume,
            tags: record.clone().ware_tags,
            price: ExportPrice {
                min: record.clone().ware_price_min,
                average: record.clone().ware_price_avg,
                max: record.clone().ware_price_max,
            },
            component: ExportComponent {
                r#ref: record.clone().ware_comp_ref,
                amount: record.clone().ware_comp_amt,
            },
            restriction: ExportRestriction { licence: record.clone().ware_license },
            r#use: ExportUse { threshold: record.clone().ware_use_threshold }
        };
        // get owner and productions values in there
        // let export_owners = vec![];
        let mut owner_string = "".to_string();
        for ownerfaction in record.clone().ware_owners.split_whitespace() {
            // let owner: ExportOwner = ExportOwner {faction: ownerfaction.to_string()};
            // export_owners.push(owner);
            owner_string.push_str(format!("<owner faction=\"{}\" />\n", ownerfaction).as_str());
        };
        // let add_ware_string = XMLElement::from(&ware_record).to_string_pretty_prolog("\n", "    ");
        let add_ware_string = XMLElement::from(&ware_record).to_string_pretty("\n", "    ");
        let add_ware_string = add_ware_string.replace("<component", format!("{}\n<component", record.clone().ware_production).as_str());
        let add_ware_string = add_ware_string.replace("</ware>", format!("{}</ware>\n", owner_string).as_str());

        // push to ware file
        ware_string.push_str(&add_ware_string);



        // write weapon file
        let weapon_string = clean_string(XMLElement::from(weapon_record).to_string_pretty_prolog("\n", "    "));
        let mut weapon_buffer = File::create(out_path.join(format!("{}.xml", &record.macro_name))).unwrap();
        weapon_buffer.write_all(weapon_string.as_bytes()).unwrap();
        // write bullet file
        let bullet_string = clean_string(XMLElement::from(bullet_record).to_string_pretty_prolog("\n", "    "));
        let mut bullet_buffer = File::create(out_path.join(format!("{}.xml", &record.bullet_macro_name))).unwrap();
        bullet_buffer.write_all(bullet_string.as_bytes()).unwrap();
    }
    // write ware file
    ware_string.push_str("\n</wares>");
    let mut ware_buffer = File::create(out_path.join("wares.xml")).unwrap();
    ware_buffer.write_all(ware_string.as_bytes()).unwrap();
}

// WRITE CSV
// let mut wtr = csv::Writer::from_path(out_path.join("bullet.csv")).expect("make writer");
// wtr.serialize(csv_struct).expect("write writer");

// READ CSV
// let mut rdr = csv::Reader::from_path(out_path.join("bullet.csv")).expect("make reader");
// for result in rdr.deserialize() {
//     let record: CSVStruct = result.unwrap();
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
