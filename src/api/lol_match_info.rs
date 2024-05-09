use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub metadata: MatchMetadata,
    pub info: MatchInfo
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchMetadata {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub game_creation: u64,
    pub game_duration: u32,
    pub game_id: u64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: u64,
    pub game_type: String,
    pub game_version: String,
    pub participants: Vec<MatchParticipant>,
    pub platform_id: String,
    pub queue_id: u16,
    pub teams: Vec<Team>,
    pub tournament_code: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct MatchParticipant {
    pub assists: u8,
    pub baron_kills: u8,
    pub bounty_level: u8,
    pub champ_experience: u32,
    pub champ_level: u32,
    pub champion_id: u32,
    pub champion_name: String,
    pub champion_transform: u8,
    pub consumables_purchased: u16,
    pub damage_dealt_to_buildings: u64,
    pub damage_dealt_to_objectives: u64,
    pub damage_dealt_to_turrets: u64,
    pub damage_self_mitigated: u64,
    pub deaths: u8,
    pub detector_wards_placed: u8,
    pub double_kills: u8,
    pub dragon_kills: u8,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub gold_earned: u16,
    pub gold_spent: u16,
    pub individual_position: String,
    pub inhibitor_kills: u8,
    pub inhibitor_takedowns: u8,
    pub inhibitors_lost: u8,
    pub item0: u16,
    pub item1: u16,
    pub item2: u16,
    pub item3: u16,
    pub item4: u16,
    pub item5: u16,
    pub item6: u16,
    pub items_purchased: u8,
    pub killing_sprees: u8,
    pub kills: u8,
    pub lane: String,
    pub largest_critical_strike: u16,
    pub largest_killing_spree: u8,
    pub largest_multi_kill: u8,
    pub longest_time_spent_living: u16,
    pub magic_damage_dealt: u64,
    pub magic_damage_dealt_to_champions: u64,
    pub magic_damage_taken: u64,
    pub neutral_minions_killed: u16,
    pub nexus_kills: u8,
    pub nexus_lost: u8,
    pub nexus_takedowns: u8,
    pub objectives_stolen: u8,
    pub objectives_stolen_assists: u8,
    pub participant_id: u8,
    pub penta_kills: u8,
    pub perks: Perk,
    pub physical_damage_dealt: u64,
    pub physical_damage_dealt_to_champions: u64,
    pub physical_damage_taken: u64,
    pub profile_icon: u32,
    pub puuid: String,
    pub quadra_kills: u8,
    pub riot_id_name: String,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: u32,
    pub spell1_casts: u32,
    pub spell2_casts: u32,
    pub spell3_casts: u32,
    pub spell4_casts: u32,
    pub summoner1_casts: u32,
    pub summoner1_id: u8,
    pub summoner2_id: u8,
    pub summoner_id: String,
    pub summoner_level: u16,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: u32,
    pub team_position: String,
    pub time_CCing_others: u64,
    pub total_damage_dealt: u64,
    pub total_damage_dealt_to_champions: u64,
    pub total_damage_shielded_on_teammates: u64,
    pub total_damage_taken: u64,
    pub total_heal: u64,
    pub total_heals_on_teammates: u64,
    pub total_minions_killed: u16,
    pub total_time_CC_dealt: u32,
    pub total_time_spent_dead: u32,
    pub total_units_healed: u32,
    pub triple_kills: u8,
    pub true_damage_dealt: u64,
    pub true_damage_dealt_to_champions: u64,
    pub true_damage_taken: u64,
    pub turret_kills: u8,
    pub turret_takedowns: u8,
    pub turrets_lost: u8,
    pub unreal_kills: u8,
    pub vision_score: u16,
    pub vision_wards_bought_in_game: u16,
    pub wards_killed: u16,
    pub wards_placed: u16,
    pub win: bool
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    pub stat_perks: StatPerk,
    pub styles: Vec<Style>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatPerk {
    pub defense: u16,
    pub flex: u16,
    pub offense: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub description: String,
    pub selections: Vec<Selection>,
    pub style: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
    pub perk: u32,
    pub var1: u16,
    pub var2: u16,
    pub var3: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Vec<Ban>,
    pub objectives: Objective,
    pub team_id: u16,
    pub win: bool
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub champion_id: u16,
    pub pick_turn: u8
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    pub baron: Baron,
    pub champion: Champion,
    pub dragon: Dragon,
    pub inhibitor: Inhibitor,
    pub rift_herald: RiftHerald,
    pub tower: Tower
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Baron {
    pub first: bool,
    pub kills: u8
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub first: bool,
    pub kills: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dragon {
    pub first: bool,
    pub kills: u8
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Inhibitor {
    pub first: bool,
    pub kills: u8
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RiftHerald {
    pub first: bool,
    pub kills: u8
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tower {
    pub first: bool,
    pub kills: u8
}
