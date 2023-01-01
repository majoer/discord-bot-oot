use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::error;

#[derive(Deserialize, Serialize, Debug)]
struct Settings {}

#[derive(Deserialize, Serialize, Debug)]
struct RandomizedWorldSettings {
    starting_age: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ItemPool {}

#[derive(Deserialize, Serialize, Debug)]
struct Dungeon {}

#[derive(Deserialize, Serialize, Debug)]
struct EmptyDungeon {}

#[derive(Deserialize, Serialize, Debug)]
struct Trial {}

#[derive(Deserialize, Serialize, Debug)]
struct Entrance {
    region: String,
    from: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Item {
    item: String,
    player: u32,
    price: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct GossipStone {
    text: String,
    colors: Vec<String>,
    hinted_locations: Option<Vec<String>>,
    hinted_items: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Log {
    #[serde(rename = ":version")]
    version: String,

    file_hash: Vec<String>,

    #[serde(rename = ":seed")]
    seed: String,

    #[serde(rename = ":settings_string")]
    settings_string: String,

    #[serde(rename = ":enable_distribution_file")]
    enable_distribution_file: bool,

    settings: Settings,

    randomized_settings: HashMap<String, RandomizedWorldSettings>,

    item_pool: HashMap<String, ItemPool>,

    dungeons: HashMap<String, Dungeon>,

    empty_dungeons: HashMap<String, EmptyDungeon>,

    trials: HashMap<String, Trial>,

    // entrances: HashMap<String, HashMap<String, Entrance>>,
    locations: HashMap<String, HashMap<String, Item>>,

    #[serde(rename = ":skipped_locations")]
    skipped_locations: HashMap<String, HashMap<String, Item>>,

    #[serde(rename = ":woth_locations")]
    woth_locations: HashMap<String, HashMap<String, Item>>,

    #[serde(rename = ":goal_locations")]
    goal_locations:
        HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<String, Item>>>>>,

    #[serde(rename = ":barren_regions")]
    barren_regions: HashMap<String, Vec<String>>,

    gossip_stones: HashMap<String, HashMap<String, GossipStone>>,

    #[serde(rename = ":playthrough")]
    playthrough: HashMap<String, HashMap<String, Item>>,
}

struct ItemLocation {
    location: String,
    player: u32,
    world: String,
}

pub fn parse_spoiler_log(spoiler_log_str: String) -> String {
    let spoiler_log: Log = match serde_json::from_str(&spoiler_log_str) {
        Ok(content) => content,
        Err(why) => {
            error!("{:?}", why);
            panic!("Unable to parse file")
        }
    };

    let mut item_locations: HashMap<String, Vec<ItemLocation>> = HashMap::new();

    for (world_name, world) in &spoiler_log.locations {
        for (location, item) in world {
            if !item_locations.contains_key(&item.item) {
                item_locations.insert(item.item.to_string(), Vec::new());
            }

            item_locations
                .get_mut(&item.item)
                .unwrap()
                .push(ItemLocation {
                    location: (location.to_string()),
                    player: (item.player),
                    world: (world_name.to_string()),
                });
        }
    }

    let ignored_keys = vec![
        "Arrows (10)",
        "Arrows (30)",
        "Arrows (5)",
        "Bombs (10)",
        "Bombs (20)",
        "Bombs (5)",
        "Buy Arrows (10)",
        "Buy Arrows (30)",
        "Buy Arrows (50)",
        "Buy Blue Fire",
        "Buy Bombs (5) for 25 Rupees",
        "Buy Bombs (5) for 35 Rupees",
        "Buy Bombchu (10)",
        "Buy Bombchu (20)",
        "Buy Bombchu (5)",
        "Buy Bombs (10)",
        "Buy Bombs (20)",
        "Buy Bombs (30)",
        "Buy Bombs (5)",
        "Buy Bottle Bug",
        "Buy Deku Nut (5)",
        "Buy Deku Seeds (30)",
        "Buy Deku Stick (1)",
        "Buy Fairy's Spirit",
        "Buy Fish",
        "Buy Green Potion",
        "Buy Red Potion for 30 Rupees",
        "Ice Trap",
        "Piece of Heart",
        "Piece of Heart (Treasure Chest Game)",
        "Recovery Heart",
        "Rupee (1)",
        "Rupees (20)",
        "Rupees (200)",
        "Rupees (5)",
        "Rupees (50)",
    ];
    let mut item_names: Vec<&String> = Vec::from_iter(
        item_locations
            .keys()
            .filter(|key| !ignored_keys.contains(&key.as_str())),
    );
    let mut result = Vec::new();

    item_names.sort();

    for item_name in &item_names {
        let locations = item_locations.get(*item_name).unwrap();
        let mut locations_str = Vec::new();
        result.push(item_name.to_string());

        for location in locations.iter() {
            let str = format!(
                "{} Player {} -> {}: {}",
                "    ",
                location.player,
                location.world,
                location.location.to_string(),
            );

            locations_str.push(str);
        }

        locations_str.sort();
        result.extend(locations_str);
    }

    return result.join("\n");
}
