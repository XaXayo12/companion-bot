use azalea::inventory::ItemStack;
use azalea::prelude::*;
use azalea::registry::builtin::{EntityKind, ItemKind as Item};

pub fn is_food(item: Item) -> bool {
    matches!(
        item,
        Item::Apple
            | Item::Bread
            | Item::Carrot
            | Item::BakedPotato
            | Item::BeetrootSoup
            | Item::MushroomStew
            | Item::RabbitStew
            | Item::CookedBeef
            | Item::CookedPorkchop
            | Item::CookedChicken
            | Item::CookedMutton
            | Item::CookedRabbit
            | Item::CookedCod
            | Item::CookedSalmon
            | Item::MelonSlice
            | Item::SweetBerries
            | Item::GlowBerries
            | Item::Cookie
            | Item::PumpkinPie
            | Item::DriedKelp
            | Item::GoldenCarrot
            | Item::GoldenApple
            | Item::EnchantedGoldenApple
    )
}

pub fn sword_rank(item: Item) -> Option<u8> {
    Some(match item {
        Item::WoodenSword => 1,
        Item::GoldenSword => 2,
        Item::StoneSword => 3,
        Item::IronSword => 4,
        Item::DiamondSword => 5,
        Item::NetheriteSword => 6,
        _ => return None,
    })
}

pub fn pickaxe_rank(item: Item) -> Option<u8> {
    Some(match item {
        Item::WoodenPickaxe => 1,
        Item::GoldenPickaxe => 2,
        Item::StonePickaxe => 3,
        Item::IronPickaxe => 4,
        Item::DiamondPickaxe => 5,
        Item::NetheritePickaxe => 6,
        _ => return None,
    })
}

pub fn axe_rank(item: Item) -> Option<u8> {
    Some(match item {
        Item::WoodenAxe => 1,
        Item::GoldenAxe => 2,
        Item::StoneAxe => 3,
        Item::IronAxe => 4,
        Item::DiamondAxe => 5,
        Item::NetheriteAxe => 6,
        _ => return None,
    })
}

pub fn shovel_rank(item: Item) -> Option<u8> {
    Some(match item {
        Item::WoodenShovel => 1,
        Item::GoldenShovel => 2,
        Item::StoneShovel => 3,
        Item::IronShovel => 4,
        Item::DiamondShovel => 5,
        Item::NetheriteShovel => 6,
        _ => return None,
    })
}

pub fn is_shield(item: Item) -> bool {
    item_name(item) == "shield"
}

pub fn is_totem(item: Item) -> bool {
    matches!(item, Item::TotemOfUndying)
}

/// Which armor equipment slot an item belongs to, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorSlot {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

/// Returns the armor slot and a quality rank (higher = better) for an item.
pub fn armor_info(item: Item) -> Option<(ArmorSlot, u8)> {
    use ArmorSlot::*;
    let (slot, material) = match item {
        Item::LeatherHelmet => (Helmet, 1),
        Item::LeatherChestplate => (Chestplate, 1),
        Item::LeatherLeggings => (Leggings, 1),
        Item::LeatherBoots => (Boots, 1),
        Item::GoldenHelmet => (Helmet, 2),
        Item::GoldenChestplate => (Chestplate, 2),
        Item::GoldenLeggings => (Leggings, 2),
        Item::GoldenBoots => (Boots, 2),
        Item::ChainmailHelmet => (Helmet, 3),
        Item::ChainmailChestplate => (Chestplate, 3),
        Item::ChainmailLeggings => (Leggings, 3),
        Item::ChainmailBoots => (Boots, 3),
        Item::IronHelmet => (Helmet, 4),
        Item::IronChestplate => (Chestplate, 4),
        Item::IronLeggings => (Leggings, 4),
        Item::IronBoots => (Boots, 4),
        Item::DiamondHelmet => (Helmet, 5),
        Item::DiamondChestplate => (Chestplate, 5),
        Item::DiamondLeggings => (Leggings, 5),
        Item::DiamondBoots => (Boots, 5),
        Item::NetheriteHelmet => (Helmet, 6),
        Item::NetheriteChestplate => (Chestplate, 6),
        Item::NetheriteLeggings => (Leggings, 6),
        Item::NetheriteBoots => (Boots, 6),
        Item::TurtleHelmet => (Helmet, 4),
        _ => return None,
    };
    Some((slot, material))
}

/// Any held tool or weapon (sword, axe, pickaxe, shovel, hoe) plus the special
/// gear a player never throws away (bow, crossbow, trident, mace, shield,
/// elytra, fishing rod, shears, flint and steel, brush, spyglass).
pub fn is_tool(item: Item) -> bool {
    let name = item_name(item);
    name.ends_with("_sword")
        || name.ends_with("_axe")
        || name.ends_with("_pickaxe")
        || name.ends_with("_shovel")
        || name.ends_with("_hoe")
        || matches!(
            name.as_str(),
            "bow"
                | "crossbow"
                | "trident"
                | "mace"
                | "shield"
                | "elytra"
                | "fishing_rod"
                | "shears"
                | "flint_and_steel"
                | "brush"
                | "spyglass"
                | "bucket"
                | "water_bucket"
                | "lava_bucket"
                | "compass"
                | "clock"
                | "name_tag"
                | "lead"
        )
}

/// Materials and rares the bot must never throw away or deposit as "trash":
/// ingots and gems, netherite, enchanted books and smithing templates, pearls,
/// shulker boxes, beacons, nether stars, XP bottles, golden/enchanted apples.
pub fn is_valuable(item: Item) -> bool {
    let name = item_name(item);
    name.ends_with("_ingot")
        || name.ends_with("_shulker_box")
        || name.ends_with("_smithing_template")
        || name.ends_with("_upgrade_smithing_template")
        || matches!(
            name.as_str(),
            "diamond"
                | "emerald"
                | "lapis_lazuli"
                | "amethyst_shard"
                | "netherite_scrap"
                | "ancient_debris"
                | "raw_iron"
                | "raw_gold"
                | "raw_copper"
                | "enchanted_book"
                | "experience_bottle"
                | "ender_pearl"
                | "ender_eye"
                | "nether_star"
                | "beacon"
                | "totem_of_undying"
                | "golden_apple"
                | "enchanted_golden_apple"
                | "elytra"
                | "heart_of_the_sea"
                | "nautilus_shell"
                | "echo_shard"
                | "dragon_egg"
        )
}

/// A "keep" item is something the bot must never drop or deposit:
/// food, every tool/weapon, armor, totems, placeable blocks, and valuables.
/// Anything not covered here is treated as trash, so we err on the side of
/// keeping items rather than ever throwing away something useful.
pub fn is_keep(item: Item) -> bool {
    is_food(item)
        || is_tool(item)
        || armor_info(item).is_some()
        || is_totem(item)
        || is_block(item)
        || is_valuable(item)
}

/// The inverse of [`is_keep`]: trash that can be dropped/deposited. Because
/// [`is_keep`] is deliberately broad, only genuinely worthless loot (rotten
/// flesh, string, bones, gunpowder, spider eyes, ...) ever counts as trash.
pub fn is_trash(item: Item) -> bool {
    !is_keep(item)
}

/// Heuristic block detection: any item whose registry name ends in a common
/// block suffix, plus a handful of bare-name blocks. We can't enumerate every
/// block variant, so we match on the registry identifier string.
pub fn is_block(item: Item) -> bool {
    let name = item_name(item);
    name.ends_with("_block")
        || name.ends_with("_planks")
        || name.ends_with("_log")
        || name.ends_with("_stairs")
        || name.ends_with("_slab")
        || name.ends_with("_wall")
        || name.ends_with("_fence")
        || name.ends_with("_concrete")
        || name.ends_with("_terracotta")
        || name.ends_with("_wool")
        || name.ends_with("_glass")
        || matches!(
            name.as_str(),
            "stone"
                | "cobblestone"
                | "dirt"
                | "grass_block"
                | "sand"
                | "gravel"
                | "netherrack"
                | "obsidian"
                | "deepslate"
                | "bricks"
                | "glass"
                | "torch"
        )
}

/// The registry identifier (e.g. "diamond_sword") for an item.
fn item_name(item: Item) -> String {
    format!("{item:?}")
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i != 0 {
                vec!['_', c.to_ascii_lowercase()]
            } else {
                vec![c.to_ascii_lowercase()]
            }
        })
        .collect()
}

pub fn is_hostile(kind: EntityKind) -> bool {
    matches!(
        kind,
        EntityKind::Zombie
            | EntityKind::ZombieVillager
            | EntityKind::Husk
            | EntityKind::Drowned
            | EntityKind::Skeleton
            | EntityKind::Stray
            | EntityKind::Bogged
            | EntityKind::WitherSkeleton
            | EntityKind::Creeper
            | EntityKind::Spider
            | EntityKind::CaveSpider
            | EntityKind::Slime
            | EntityKind::MagmaCube
            | EntityKind::Witch
            | EntityKind::Pillager
            | EntityKind::Vindicator
            | EntityKind::Ravager
            | EntityKind::Evoker
            | EntityKind::Vex
            | EntityKind::Blaze
            | EntityKind::Phantom
            | EntityKind::Silverfish
            | EntityKind::Endermite
            | EntityKind::Hoglin
            | EntityKind::Zoglin
            | EntityKind::PiglinBrute
            | EntityKind::Guardian
            | EntityKind::ElderGuardian
            | EntityKind::Shulker
            | EntityKind::Breeze
    )
}

fn ranked_hotbar_slot(bot: &Client, rank: impl Fn(Item) -> Option<u8>) -> Option<u8> {
    let menu = bot.menu();
    let slots = menu.slots();
    let mut best: Option<(u8, u8)> = None;
    for hotbar_index in 0u8..9 {
        let slot_index = 36 + hotbar_index as usize;
        if let Some(ItemStack::Present(item)) = slots.get(slot_index)
            && let Some(value) = rank(item.kind)
                && best.is_none_or(|(best_value, _)| value > best_value) {
                    best = Some((value, hotbar_index));
                }
    }
    best.map(|(_, hotbar_index)| hotbar_index)
}

pub fn food_slot(bot: &Client) -> Option<u8> {
    hotbar_slot(bot, is_food)
}

/// The first hotbar slot (0..9) whose item satisfies `pred`, if any.
pub fn hotbar_slot(bot: &Client, pred: impl Fn(Item) -> bool) -> Option<u8> {
    let menu = bot.menu();
    let slots = menu.slots();
    for hotbar_index in 0u8..9 {
        let slot_index = 36 + hotbar_index as usize;
        if let Some(ItemStack::Present(item)) = slots.get(slot_index)
            && pred(item.kind)
        {
            return Some(hotbar_index);
        }
    }
    None
}

pub fn is_water_bucket(item: Item) -> bool {
    item_name(item) == "water_bucket"
}

pub fn is_empty_bucket(item: Item) -> bool {
    item_name(item) == "bucket"
}

pub fn best_sword_slot(bot: &Client) -> Option<u8> {
    ranked_hotbar_slot(bot, sword_rank)
}

pub fn best_pickaxe_slot(bot: &Client) -> Option<u8> {
    ranked_hotbar_slot(bot, pickaxe_rank)
}

/// Best melee weapon in the hotbar: swords and axes both count. For the same
/// material a sword wins (faster), but a higher-tier axe beats a lower sword.
pub fn best_weapon_slot(bot: &Client) -> Option<u8> {
    ranked_hotbar_slot(bot, |item| {
        sword_rank(item)
            .map(|r| r * 2 + 1)
            .or_else(|| axe_rank(item).map(|r| r * 2))
    })
}

/// Best tool in the hotbar for the block whose registry id is `block_id`:
/// axe for wood-like blocks, shovel for dirt/sand/gravel/snow, otherwise the
/// best pickaxe (with a pickaxe fallback so the bot always holds something).
pub fn best_tool_slot_for(bot: &Client, block_id: &str) -> Option<u8> {
    let wants_axe = block_id.ends_with("_log")
        || block_id.ends_with("_wood")
        || block_id.ends_with("_planks")
        || block_id.ends_with("_stem")
        || block_id.ends_with("_hyphae")
        || block_id.contains("chest")
        || matches!(block_id, "crafting_table" | "bookshelf" | "pumpkin" | "melon");
    let wants_shovel = matches!(
        block_id,
        "dirt"
            | "grass_block"
            | "podzol"
            | "mycelium"
            | "coarse_dirt"
            | "rooted_dirt"
            | "sand"
            | "red_sand"
            | "gravel"
            | "clay"
            | "soul_sand"
            | "soul_soil"
            | "snow"
            | "snow_block"
    ) || block_id.ends_with("_snow");
    if wants_axe {
        ranked_hotbar_slot(bot, axe_rank).or_else(|| best_pickaxe_slot(bot))
    } else if wants_shovel {
        ranked_hotbar_slot(bot, shovel_rank).or_else(|| best_pickaxe_slot(bot))
    } else {
        best_pickaxe_slot(bot)
    }
}

// --- Player menu absolute slot indices (Player menu layout) ---
pub const PLAYER_ARMOR_HELMET: usize = 5;
pub const PLAYER_ARMOR_CHESTPLATE: usize = 6;
pub const PLAYER_ARMOR_LEGGINGS: usize = 7;
pub const PLAYER_ARMOR_BOOTS: usize = 8;
pub const PLAYER_OFFHAND: usize = 45;
/// Inclusive range of the player's main inventory + hotbar storage slots.
pub const PLAYER_STORAGE_START: usize = 9;
pub const PLAYER_STORAGE_END: usize = 44;

pub fn armor_target_slot(slot: ArmorSlot) -> usize {
    match slot {
        ArmorSlot::Helmet => PLAYER_ARMOR_HELMET,
        ArmorSlot::Chestplate => PLAYER_ARMOR_CHESTPLATE,
        ArmorSlot::Leggings => PLAYER_ARMOR_LEGGINGS,
        ArmorSlot::Boots => PLAYER_ARMOR_BOOTS,
    }
}

/// Find an absolute slot index within the player's storage (9..=44) holding an
/// item that satisfies `pred`.
pub fn find_storage_slot(bot: &Client, pred: impl Fn(Item) -> bool) -> Option<usize> {
    let menu = bot.menu();
    let slots = menu.slots();
    for index in PLAYER_STORAGE_START..=PLAYER_STORAGE_END {
        if let Some(ItemStack::Present(item)) = slots.get(index)
            && pred(item.kind)
        {
            return Some(index);
        }
    }
    None
}

/// The item kind currently in the bot's offhand, if any.
pub fn offhand_item(bot: &Client) -> Option<Item> {
    let menu = bot.menu();
    let slots = menu.slots();
    match slots.get(PLAYER_OFFHAND) {
        Some(ItemStack::Present(item)) => Some(item.kind),
        _ => None,
    }
}

/// The currently-equipped armor item in the given slot, if any.
pub fn equipped_armor(bot: &Client, slot: ArmorSlot) -> Option<Item> {
    let menu = bot.menu();
    let slots = menu.slots();
    match slots.get(armor_target_slot(slot)) {
        Some(ItemStack::Present(item)) => Some(item.kind),
        _ => None,
    }
}

/// Find the best (highest-ranked) armor piece for `slot` sitting in storage.
/// Returns (absolute_slot_index, rank).
pub fn best_armor_in_storage(bot: &Client, want: ArmorSlot) -> Option<(usize, u8)> {
    let menu = bot.menu();
    let slots = menu.slots();
    let mut best: Option<(usize, u8)> = None;
    for index in PLAYER_STORAGE_START..=PLAYER_STORAGE_END {
        if let Some(ItemStack::Present(item)) = slots.get(index)
            && let Some((slot, rank)) = armor_info(item.kind)
            && slot == want
            && best.is_none_or(|(_, br)| rank > br)
        {
            best = Some((index, rank));
        }
    }
    best
}
