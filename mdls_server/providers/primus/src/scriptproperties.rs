use serde::Deserialize;

#[derive(Debug, Default, Clone)]
pub struct ScriptProperties {
    pub data: Vec<ApplicableProperties>,
}

impl ScriptProperties {
    pub fn new(string: &str) -> Self {
        let doc = roxmltree::Document::parse(&string).expect("malformed xml i guess");
        let mut sp = ScriptProperties { data: vec![] };
        let mut ap = ApplicableProperties {
            datatype: Datatypes::Unknown,
            properties: vec![],
        };
        for node in doc.descendants() {
            match node.tag_name().name() {
                "datatype" | "keyword" => {
                    // last keyword will never be pushed
                    sp.data.push(ap.clone());
                    ap.properties.clear();
                    ap.datatype = match_datatype(node.attribute("name"));
                }
                "property" => ap.properties.push(Property {
                    datatype: ap.datatype.clone(),
                    prop_name: if let Some(stir) = node.attribute("name") {
                        Some(stir.to_string())
                    } else {
                        None
                    },
                    prop_result: if let Some(stir) = node.attribute("result") {
                        Some(stir.to_string())
                    } else {
                        None
                    },
                    prop_type: if let Some(stir) = node.attribute("type") {
                        Some(stir.to_string())
                    } else {
                        None
                    }, // keywords have description instead of result (among others)
                }),
                _ => (),
            }
        }
        sp
    }
    pub fn search(&self, string: &str) -> Vec<Property> {
        let properties = self.data.iter().filter_map(|x| x.search(string)).collect();
        // for datatype in self.data.iter() {
        //     if let Some(prop) = datatype.search(string) {

        //     }
        // }
        properties
    }
}

pub fn match_datatype(string: Option<&str>) -> Datatypes {
    match string.expect("datatype or keyword didn't have a name field") {
        "activity" => Datatypes::Activity,
        "adsign" => Datatypes::Adsign,
        "alertlevel" => Datatypes::Alertlevel,
        "angle" => Datatypes::Angle,
        "assignment" => Datatypes::Assignment,
        "attention" => Datatypes::Attention,
        "blacklistgroup" => Datatypes::Blacklistgroup,
        "blacklisttype" => Datatypes::Blacklisttype,
        "boardingbehaviour" => Datatypes::Boardingbehaviour,
        "boardingphase" => Datatypes::Boardingphase,
        "boolean" => Datatypes::Boolean,
        "build" => Datatypes::Build,
        "buildmodule" => Datatypes::Buildmodule,
        "buildprocessor" => Datatypes::Buildprocessor,
        "buildstorage" => Datatypes::Buildstorage,
        "cargolist" => Datatypes::Cargolist,
        "class" => Datatypes::Class,
        "cluster" => Datatypes::Cluster,
        "collectable" => Datatypes::Collectable,
        "command" => Datatypes::Command,
        "commandaction" => Datatypes::Commandaction,
        "component" => Datatypes::Component,
        "componentslot" => Datatypes::Componentslot,
        "componentstate" => Datatypes::Componentstate,
        "constructionplanentrydata" => Datatypes::Constructionplanentrydata,
        "constructionplanentryid" => Datatypes::Constructionplanentryid,
        "constructionsequence" => Datatypes::Constructionsequence,
        "container" => Datatypes::Container,
        "controllable" => Datatypes::Controllable,
        "controlpaneltype" => Datatypes::Controlpaneltype,
        "controlposition" => Datatypes::Controlposition,
        "controlpost" => Datatypes::Controlpost,
        "cue" => Datatypes::Cue,
        "cuestate" => Datatypes::Cuestate,
        "datatype" => Datatypes::Datatype,
        "dbdata" => Datatypes::Dbdata,
        "defensible" => Datatypes::Defensible,
        "deployablecategory" => Datatypes::Deployablecategory,
        "destructible" => Datatypes::Destructible,
        "dockarea" => Datatypes::Dockarea,
        "dockingbay" => Datatypes::Dockingbay,
        "dockstate" => Datatypes::Dockstate,
        "dronemode" => Datatypes::Dronemode,
        "drop" => Datatypes::Drop,
        "engine" => Datatypes::Engine,
        "entity" => Datatypes::Entity,
        "entityrole" => Datatypes::Entityrole,
        "entitytype" => Datatypes::Entitytype,
        "enum" => Datatypes::Enum,
        "explosive" => Datatypes::Explosive,
        "faction" => Datatypes::Faction,
        "flightbehaviour" => Datatypes::Flightbehaviour,
        "flightcontrolmodel" => Datatypes::Flightcontrolmodel,
        "float" => Datatypes::Float,
        "formationshape" => Datatypes::Formationshape,
        "galaxy" => Datatypes::Galaxy,
        "gate" => Datatypes::Gate,
        "group" => Datatypes::Group,
        "highway" => Datatypes::Highway,
        "highwayentrygate" => Datatypes::Highwayentrygate,
        "highwayexitgate" => Datatypes::Highwayexitgate,
        "hitpoints" => Datatypes::Hitpoints,
        "integer" => Datatypes::Integer,
        "killmethod" => Datatypes::Killmethod,
        "kwenum" => Datatypes::Kwenum,
        "largefloat" => Datatypes::Largefloat,
        "largeint" => Datatypes::Largeint,
        "length" => Datatypes::Length,
        "level" => Datatypes::Level,
        "licence" => Datatypes::Licence,
        "list" => Datatypes::List,
        "loadout" => Datatypes::Loadout,
        "lock" => Datatypes::Lock,
        "lookuplist" => Datatypes::Lookuplist,
        "macro" => Datatypes::Macro,
        "missiongroup" => Datatypes::Missiongroup,
        "missiontype" => Datatypes::Missiontype,
        "module" => Datatypes::Module,
        "money" => Datatypes::Money,
        "moodlevel" => Datatypes::Moodlevel,
        "navcontext" => Datatypes::Navcontext,
        "nonplayer" => Datatypes::Nonplayer,
        "notification" => Datatypes::Notification,
        "npc" => Datatypes::NPC,
        "npctemplate" => Datatypes::Npctemplate,
        "npctemplateentry" => Datatypes::Npctemplateentry,
        "numeric" => Datatypes::Numeric,
        "object" => Datatypes::Object,
        "objective" => Datatypes::Objective,
        "operation" => Datatypes::Operation,
        "order" => Datatypes::Order,
        "orderstate" => Datatypes::Orderstate,
        "pier" => Datatypes::Pier,
        "position" => Datatypes::Position,
        "purpose" => Datatypes::Purpose,
        "quadrant" => Datatypes::Quadrant,
        "race" => Datatypes::Race,
        "region" => Datatypes::Region,
        "relationchangereason" => Datatypes::Relationchangereason,
        "room" => Datatypes::Room,
        "roompopulationtype" => Datatypes::Roompopulationtype,
        "roomtype" => Datatypes::Roomtype,
        "rotation" => Datatypes::Rotation,
        "sector" => Datatypes::Sector,
        "shieldgenerator" => Datatypes::Shieldgenerator,
        "ship" => Datatypes::Ship,
        "shiptype" => Datatypes::Shiptype,
        "signalleak" => Datatypes::Signalleak,
        "signalleaktype" => Datatypes::Signalleaktype,
        "skilltype" => Datatypes::Skilltype,
        "space" => Datatypes::Space,
        "spacesuit" => Datatypes::Spacesuit,
        "station" => Datatypes::Station,
        "string" => Datatypes::String,
        "table" => Datatypes::Table,
        "tag" => Datatypes::Tag,
        "time" => Datatypes::Time,
        "trade" => Datatypes::Trade,
        "turret" => Datatypes::Turret,
        "unitcategory" => Datatypes::Unitcategory,
        "unlock" => Datatypes::Unlock,
        "vector" => Datatypes::Vector,
        "ventureplatform" => Datatypes::Ventureplatform,
        "walkablemodule" => Datatypes::Walkablemodule,
        "ware" => Datatypes::Ware,
        "wareamountlist" => Datatypes::Wareamountlist,
        "warelist" => Datatypes::Warelist,
        "waretransport" => Datatypes::Waretransport,
        "weapon" => Datatypes::Weapon,
        "weaponmode" => Datatypes::Weaponmode,
        "zone" => Datatypes::Zone,
        "player" => Datatypes::Player,
        "global" => Datatypes::Global,
        "true" => Datatypes::True,
        "false" => Datatypes::False,
        "pi" => Datatypes::Pi,
        "null" => Datatypes::Null,
        "readtext" => Datatypes::Readtext,
        "stat" => Datatypes::Stat,
        "userdata" => Datatypes::Userdata,
        "param" => Datatypes::Param,
        "loop" => Datatypes::Loop,
        "warebasket" => Datatypes::Warebasket,
        "waregroup" => Datatypes::Waregroup,
        "lookup" => Datatypes::Lookup,
        "this" => Datatypes::This,
        "event" => Datatypes::Event,
        "quota" => Datatypes::Quota,
        "md" => Datatypes::Md,
        "parent" => Datatypes::Parent,
        "static" => Datatypes::Static,
        "staticbase" => Datatypes::Staticbase,
        "namespace" => Datatypes::Namespace,
        _ => Datatypes::Unknown,
    }
}

#[derive(Debug, Clone)]
pub struct ApplicableProperties {
    pub datatype: Datatypes,
    pub properties: Vec<Property>,
}
impl ApplicableProperties {
    fn search(&self, string: &str) -> Option<Property> {
        for prop in self.properties.iter() {
            if let Some(name) = prop.clone().prop_name {
                if string.to_string() == name {
                    return Some(prop.clone());
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub datatype: Datatypes,
    pub prop_name: Option<String>,
    pub prop_result: Option<String>,
    pub prop_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Datatypes {
    Activity,
    Adsign,
    Alertlevel,
    Angle,
    Assignment,
    Attention,
    Blacklistgroup,
    Blacklisttype,
    Boardingbehaviour,
    Boardingphase,
    Boolean,
    Build,
    Buildmodule,
    Buildprocessor,
    Buildstorage,
    Cargolist,
    Class,
    Cluster,
    Collectable,
    Command,
    Commandaction,
    Component,
    Componentslot,
    Componentstate,
    Constructionplanentrydata,
    Constructionplanentryid,
    Constructionsequence,
    Container,
    Controllable,
    Controlpaneltype,
    Controlposition,
    Controlpost,
    Cue,
    Cuestate,
    Datatype,
    Dbdata,
    Defensible,
    Deployablecategory,
    Destructible,
    Dockarea,
    Dockingbay,
    Dockstate,
    Dronemode,
    Drop,
    Engine,
    Entity,
    Entityrole,
    Entitytype,
    Enum,
    Explosive,
    Faction,
    Flightbehaviour,
    Flightcontrolmodel,
    Float,
    Formationshape,
    Galaxy,
    Gate,
    Group,
    Highway,
    Highwayentrygate,
    Highwayexitgate,
    Hitpoints,
    Integer,
    Killmethod,
    Kwenum,
    Largefloat,
    Largeint,
    Length,
    Level,
    Licence,
    List,
    Loadout,
    Lock,
    Lookuplist,
    Macro,
    Missiongroup,
    Missiontype,
    Module,
    Money,
    Moodlevel,
    Navcontext,
    NPC,
    Nonplayer,
    Notification,
    Npctemplate,
    Npctemplateentry,
    Numeric,
    Object,
    Objective,
    Operation,
    Order,
    Orderstate,
    Pier,
    Position,
    Purpose,
    Quadrant,
    Race,
    Region,
    Relationchangereason,
    Room,
    Roompopulationtype,
    Roomtype,
    Rotation,
    Sector,
    Shieldgenerator,
    Ship,
    Shiptype,
    Signalleak,
    Signalleaktype,
    Skilltype,
    Space,
    Spacesuit,
    Station,
    String,
    Table,
    Tag,
    Time,
    Trade,
    Turret,
    Unitcategory,
    Unlock,
    Vector,
    Ventureplatform,
    Walkablemodule,
    Ware,
    Wareamountlist,
    Warelist,
    Waretransport,
    Weapon,
    Weaponmode,
    Zone,
    Player,
    Global,
    True,
    False,
    Pi,
    Null,
    Readtext,
    Stat,
    Userdata,
    Param,
    Loop,
    Warebasket,
    Waregroup,
    Lookup,
    This,
    Event,
    Quota,
    Md,
    Parent,
    Static,
    Staticbase,
    Namespace,
    Unknown,
}

impl Default for Datatypes {
    fn default() -> Self {
        Datatypes::Unknown
    }
    
}