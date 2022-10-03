use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MatchMetadata 
{
    data_version: String,
    match_id: String,
    participants: Vec<String>,
    info: MatchInfo
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MatchInfo 
{
    game_creation: u64,
    game_duration: u32,
    game_id: u64,
    game_mode: String,
    game_name: String,
    game_start_timestamp: u64,
    game_type: String,
    game_version: String,
    participants: Vec<MatchParticipant>,
    platform_id: String,
    queue_id: u16,
    teams: Vec<Team>,
    tournament_code: String
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MatchParticipant
{
    assists: u8,
    baron_kills: u8,
    bounty_level: u8,
    champ_experience: u32,
    champ_level: u32,
    champion_id: u32,
    champion_name: String,
    champion_transform u8,
    consumables_purchased: u16,
    damage_dealt_to_buildings: u64,
    damage_dealt_to_objectives: u64,
    damage_dealt_to_turrets: u64,
    damage_self_mitigated: u64,
    deaths: u8,
    detector_wards_placed: u8,
    double_kills: u8,
    dragon_kills: u8,
    first_blood_assist: bool,
    first_blood_kill: bool,
    first_tower_assist: bool,
    first_tower_kill: bool,
    game_ended_in_early_surrender: bool,
    game_ended_in_surrender: bool,
    gold_earned: u16,
    gold_spent: u16,
    individual_position: String,
    inhibitor_kills: u8,
    inhibitor_takedowns: u8,
    inhibitors_lost: u8,
    item0: u16,
    item1: u16,
    item2: u16,
    item3: u16,
    item4: u16,
    item5: u16,
    item6: u16,
    items_purchased: u8,
    killing_sprees: u8,
    kills: u8,
    lane: String,
    largest_critical_strike: u16,
    largest_killing_spree: u8,
    largest_multi_kill: u8,
    longest_time_spent_living: u16,
    magic_damage_dealt: u64,
    magic_damage_dealt_to_champions: u64,
    magic_damage_taken: u64,
    neutral_minions_killed: u16,
    nexus_kills: u8,
    nexus_lost: u8,
    nexus_takedowns: u8,
    objectives_stolen: u8,
    objectives_stolen_assists: u8,
    participant_id: u8,
    penta_kills: u8,
    perks: Perk,
    physical_damage_dealt: u64,
    physical_damage_dealt_to_champions: u64,
    physical_damage_taken: u64,
    profile_icon: u32,
    puuid: String,
    quadra_kills: u8,
    riot_id_name: String,
    riot_id_tagline: String,
    role: String,
    sight_wards_bought_in_game: u32,
    spell1_casts: u32,
    spell2_casts: u32,
    spell3_casts: u32,
    spell4_casts: u32,
    summoner1_casts: u32,
    summoner1_id: u8,
    summoner2_id: u8,
    summoner_id: String,
    summoner_level: u16,
    summoner_name: String,
    team_early_surrendered: bool,
    team_id: u32,
    team_position: String,
    time_CCing_others: u64,
    total_damage_dealt: u64,
    total_damage_dealt_to_champions: u64,
    total_damage_shielded_on_teammates: u64,
    total_damage_taken: u64,
    total_heal: u64,
    total_heals_on_teammates: u64,
    total_minions_killed: u16,
    total_time_CC_dealt: u32,
    total_time_spent_dead: u32,
    total_units_healed: u32,
    triple_kills: u8,
    true_damage_dealt: u64,
    true_damage_dealt_to_champions: u64,
    true_damage_taken: u64,
    turret_kills: u8,
    turret_takedowns: u8,
    turrets_lost: u8,
    unreal_kills: u8,
    vision_score: u16,
    vision_wards_bought_in_game: u16,
    wards_killed: u16,
    wards_placed: u16,
    win: bool

}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Perk
{
    stat_perks: StatPerk,
    styles: Vec<Style>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StatPerk
{
    defense: u16,
    flex: u16,
    offense: u16
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Style
{
    description: String,
    selections: Vec<Selection>,
    style: u16
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Selection
{
    perk: u32,
    var1: u16,
    var2: u16,
    var3: u16
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Team
{
    bans: Vec<Ban>,
    objectives: Objective,
    team_id: u16,
    win: bool
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Ban
{
    champion_id: u16,
    pick_turn: u8
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Objective
{
    baron: Baron,
    champion: Champion,
    dragon: Dragon,
    inhibitor: Inhibitor,
    rift_herald: RiftHerald,
    tower: Tower
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Baron
{
    first: bool,
    kills: u8
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Champion
{
    first: bool,
    kills: u16
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Dragon
{
    first: bool,
    kills: u8
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Inhibitor
{
    first: bool,
    kills: u8
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RiftHerald
{
    first: bool,
    kills: u8
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tower
{
    first: bool,
    kills: u8
}
