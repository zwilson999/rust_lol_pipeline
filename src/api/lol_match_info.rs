use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MatchResponse {
    #[serde(default)]
    pub metadata: MatchMetadata,
    #[serde(default)]
    pub info: MatchInfo,
    #[serde(default)]
    pub account_name: String,
    #[serde(default)]
    pub match_id: String,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MatchMetadata {
    #[serde(default)]
    pub data_version: String,
    #[serde(default)]
    pub match_id: String,
    #[serde(default)]
    pub participants: Vec<String>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MatchInfo {
    #[serde(default)]
    pub end_of_game_result: Option<String>,
    #[serde(default)]
    pub game_creation: Option<u64>,
    #[serde(default)]
    pub game_duration: Option<u32>,
    #[serde(default)]
    pub game_end_timestamp: Option<u64>,
    #[serde(default)]
    pub game_id: Option<u64>,
    #[serde(default)]
    pub game_mode: Option<String>,
    #[serde(default)]
    pub game_name: Option<String>,
    #[serde(default)]
    pub game_start_timestamp: Option<u64>,
    #[serde(default)]
    pub game_type: String,
    #[serde(default)]
    pub game_version: Option<String>,
    #[serde(default)]
    pub participants: Option<Vec<MatchParticipant>>,
    #[serde(default)]
    pub platform_id: Option<String>,
    #[serde(default)]
    pub queue_id: i64,
    #[serde(default)]
    pub teams: Option<Vec<Team>>,
    #[serde(default)]
    pub tournament_code: Option<String>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[allow(non_snake_case)]
pub struct MatchParticipant {
    #[serde(default)]
    pub all_in_pings: Option<u32>,
    #[serde(default)]
    pub assist_me_pings: Option<u32>,
    #[serde(default)]
    pub assists: Option<u32>,
    #[serde(default)]
    pub baron_kills: Option<u32>,
    #[serde(default)]
    pub bounty_level: Option<u32>,
    #[serde(default)]
    pub champ_experience: Option<u32>,
    #[serde(default)]
    pub champ_level: Option<u32>,
    #[serde(default)]
    pub champion_id: Option<u32>,
    #[serde(default)]
    pub champion_name: Option<String>,
    #[serde(default)]
    pub champion_transform: Option<u32>,
    #[serde(default)]
    pub consumables_purchased: Option<u32>,
    #[serde(default)]
    pub challenges: Option<Challenges>,
    #[serde(default)]
    pub damage_dealt_to_buildings: Option<u32>,
    #[serde(default)]
    pub damage_dealt_to_objectives: Option<u32>,
    #[serde(default)]
    pub damage_dealt_to_turrets: Option<u32>,
    #[serde(default)]
    pub danger_pings: Option<u32>,
    #[serde(default)]
    pub damage_self_mitigated: Option<u32>,
    #[serde(default)]
    pub deaths: Option<u32>,
    #[serde(default)]
    pub detector_wards_placed: Option<u32>,
    #[serde(default)]
    pub double_kills: Option<u32>,
    #[serde(default)]
    pub dragon_kills: Option<u32>,
    #[serde(default)]
    pub eligible_for_progression: Option<bool>,
    #[serde(default)]
    pub enemy_missing_pings: Option<u32>,
    #[serde(default)]
    pub enemy_vision_pings: Option<u32>,
    #[serde(default)]
    pub first_blood_assist: Option<bool>,
    #[serde(default)]
    pub first_blood_kill: Option<bool>,
    #[serde(default)]
    pub first_tower_assist: Option<bool>,
    #[serde(default)]
    pub first_tower_kill: Option<bool>,
    #[serde(default)]
    pub game_ended_in_early_surrender: Option<bool>,
    #[serde(default)]
    pub game_ended_in_surrender: Option<bool>,
    #[serde(default)]
    pub hold_pings: Option<u32>,
    #[serde(default)]
    pub get_back_pings: Option<u32>,
    #[serde(default)]
    pub gold_earned: Option<u32>,
    #[serde(default)]
    pub gold_spent: Option<u32>,
    #[serde(default)]
    pub individual_position: Option<String>,
    #[serde(default)]
    pub inhibitor_kills: Option<u32>,
    #[serde(default)]
    pub inhibitor_takedowns: Option<u32>,
    #[serde(default)]
    pub inhibitors_lost: Option<u32>,
    #[serde(default)]
    pub item0: Option<u32>,
    #[serde(default)]
    pub item1: Option<u32>,
    #[serde(default)]
    pub item2: Option<u32>,
    #[serde(default)]
    pub item3: Option<u32>,
    #[serde(default)]
    pub item4: Option<u32>,
    #[serde(default)]
    pub item5: Option<u32>,
    #[serde(default)]
    pub item6: Option<u32>,
    #[serde(default)]
    pub items_purchased: Option<u32>,
    #[serde(default)]
    pub killing_sprees: Option<u32>,
    #[serde(default)]
    pub kills: Option<u32>,
    #[serde(default)]
    pub lane: Option<String>,
    #[serde(default)]
    pub largest_critical_strike: Option<u32>,
    #[serde(default)]
    pub largest_killing_spree: Option<u32>,
    #[serde(default)]
    pub largest_multi_kill: Option<u32>,
    #[serde(default)]
    pub longest_time_spent_living: Option<u32>,
    #[serde(default)]
    pub magic_damage_dealt: Option<u32>,
    #[serde(default)]
    pub magic_damage_dealt_to_champions: Option<u32>,
    #[serde(default)]
    pub magic_damage_taken: Option<u32>,
    #[serde(default)]
    pub missions: Option<Missions>,
    #[serde(default)]
    pub neutral_minions_killed: Option<u32>,
    #[serde(default)]
    pub need_vision_pings: Option<u64>,
    #[serde(default)]
    pub nexus_kills: Option<u32>,
    #[serde(default)]
    pub nexus_takedowns: Option<u32>,
    #[serde(default)]
    pub nexus_lost: Option<u32>,
    #[serde(default)]
    pub objectives_stolen: Option<u32>,
    #[serde(default)]
    pub objectives_stolen_assists: Option<u32>,
    #[serde(default)]
    pub participant_id: Option<u32>,
    #[serde(default)]
    pub penta_kills: Option<u32>,
    #[serde(default)]
    pub perks: Option<Perks>,
    #[serde(default)]
    pub physical_damage_dealt: Option<u32>,
    #[serde(default)]
    pub physical_damage_dealt_to_champions: Option<u32>,
    #[serde(default)]
    pub physical_damage_taken: Option<u32>,
    #[serde(default)]
    pub placement: Option<u32>,
    #[serde(default)]
    pub player_augment_1: Option<u32>,
    #[serde(default)]
    pub player_augment_2: Option<u32>,
    #[serde(default)]
    pub player_augment_3: Option<u32>,
    #[serde(default)]
    pub player_augment_4: Option<u32>,
    #[serde(default)]
    pub player_subteam_id: Option<u32>,
    #[serde(default)]
    pub push_pings: Option<u32>,
    #[serde(default)]
    pub profile_icon: Option<u32>,
    #[serde(default)]
    pub puuid: Option<String>,
    #[serde(default)]
    pub quadra_kills: Option<u32>,
    #[serde(default)]
    pub riot_id_game_name: Option<String>,
    #[serde(default)]
    pub riot_id_name: Option<String>,
    #[serde(default)]
    pub riot_id_tagline: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub sight_wards_bought_in_game: Option<u32>,
    #[serde(default)]
    pub spell_1_casts: Option<u32>,
    #[serde(default)]
    pub spell_2_casts: Option<u32>,
    #[serde(default)]
    pub spell_3_casts: Option<u32>,
    #[serde(default)]
    pub spell_4_casts: Option<u32>,
    #[serde(default)]
    pub subteam_placement: Option<u32>,
    #[serde(default)]
    pub summoner_1_casts: Option<u32>,
    #[serde(default)]
    pub summoner_1_id: Option<u32>,
    #[serde(default)]
    pub summoner_2_casts: Option<u32>,
    #[serde(default)]
    pub summoner_2_id: Option<u32>,
    #[serde(default)]
    pub summoner_id: Option<String>,
    #[serde(default)]
    pub summoner_level: Option<u32>,
    #[serde(default)]
    pub summoner_name: Option<String>,
    #[serde(default)]
    pub team_early_surrendered: Option<bool>,
    #[serde(default)]
    pub team_id: Option<u32>,
    #[serde(default)]
    pub team_position: Option<String>,
    #[serde(default)]
    pub time_CCing_others: Option<u32>,
    #[serde(default)]
    pub time_played: Option<u32>,
    #[serde(default)]
    pub total_damage_dealt: Option<u32>,
    #[serde(default)]
    pub total_damage_dealt_to_champions: Option<u32>,
    #[serde(default)]
    pub total_damage_shielded_on_teammates: Option<u32>,
    #[serde(default)]
    pub total_damage_taken: Option<u32>,
    #[serde(default)]
    pub total_heal: Option<u32>,
    #[serde(default)]
    pub total_heals_on_teammates: Option<u32>,
    #[serde(default)]
    pub total_minions_killed: Option<u32>,
    #[serde(default)]
    pub total_time_CC_dealt: Option<u32>,
    #[serde(default)]
    pub total_time_spent_dead: Option<u32>,
    #[serde(default)]
    pub total_units_healed: Option<u32>,
    #[serde(default)]
    pub triple_kills: Option<u32>,
    #[serde(default)]
    pub true_damage_dealt: Option<u64>,
    #[serde(default)]
    pub true_damage_dealt_to_champions: Option<u64>,
    #[serde(default)]
    pub true_damage_taken: Option<u64>,
    #[serde(default)]
    pub turret_kills: Option<u32>,
    #[serde(default)]
    pub turret_takedowns: Option<u32>,
    #[serde(default)]
    pub turrets_lost: Option<u32>,
    #[serde(default)]
    pub unreal_kills: Option<u32>,
    #[serde(default)]
    pub vision_score: Option<u32>,
    #[serde(default)]
    pub vision_cleared_pings: Option<u32>,
    #[serde(default)]
    pub vision_wards_bought_in_game: Option<u32>,
    #[serde(default)]
    pub wards_killed: Option<u32>,
    #[serde(default)]
    pub wards_placed: Option<u32>,
    #[serde(default)]
    pub win: Option<bool>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Challenges {
    #[serde(rename(deserialize = "12AssistStreakCount"))]
    #[serde(default)]
    pub twelve_assist_streak_count: Option<u32>,
    #[serde(default)]
    pub ability_uses: Option<u32>,
    #[serde(default)]
    pub aces_before_15_minutes: Option<u32>,
    #[serde(default)]
    pub allied_jungle_monster_kills: Option<u32>,
    #[serde(default)]
    pub baron_takedowns: Option<u32>,
    #[serde(default)]
    pub blast_cone_opposite_opponent_count: Option<u32>,
    #[serde(default)]
    pub bounty_gold: Option<u32>,
    #[serde(default)]
    pub buffs_stolen: Option<u32>,
    #[serde(default)]
    pub complete_support_quest_in_time: Option<u32>,
    #[serde(default)]
    pub control_wards_placed: Option<u32>,
    #[serde(default)]
    pub damage_per_minute: Option<f64>,
    #[serde(default)]
    pub damage_taken_on_team_percentage: Option<f64>,
    #[serde(default)]
    pub danced_with_rift_herald: Option<u32>,
    #[serde(default)]
    pub deaths_by_enemy_champs: Option<u32>,
    #[serde(default)]
    pub dodge_skill_shots_small_window: Option<u32>,
    #[serde(default)]
    pub double_aces: Option<u32>,
    #[serde(default)]
    pub dragon_takedowns: Option<u32>,
    #[serde(default)]
    pub earliest_baron: Option<f64>,
    #[serde(default)]
    pub earliest_dragon_takedown: Option<f64>,
    #[serde(default)]
    pub legendary_item_used: Option<Vec<u32>>,
    #[serde(default)]
    pub effective_heal_and_shielding: Option<f64>,
    #[serde(default)]
    pub elder_dragon_kills_with_opposing_soul: Option<u32>,
    #[serde(default)]
    pub elder_dragon_multikills: Option<u32>,
    #[serde(default)]
    pub enemy_champion_immobilizations: Option<u32>,
    #[serde(default)]
    pub enemy_jungle_monster_kills: Option<u32>,
    #[serde(default)]
    pub epic_monster_kills_near_enemy_jungler: Option<u32>,
    #[serde(default)]
    pub epic_monster_steals: Option<u32>,
    #[serde(default)]
    pub epic_monster_stolen_without_smite: Option<u32>,
    #[serde(default)]
    pub first_turret_killed: Option<f64>,
    #[serde(default)]
    pub first_turret_killed_time: Option<f64>,
    #[serde(default)]
    pub flawless_aces: Option<u32>,
    #[serde(default)]
    pub full_team_takedown: Option<u32>,
    #[serde(default)]
    pub game_length: Option<f64>,
    #[serde(default)]
    pub get_takedowns_in_all_lanes_early_jungle_as_laner: Option<u32>,
    #[serde(default)]
    pub gold_per_minute: Option<f64>,
    #[serde(default)]
    pub had_open_nexus: Option<u32>,
    #[serde(default)]
    pub immobilize_and_kill_with_ally: Option<u32>,
    #[serde(default)]
    pub initial_buff_count: Option<u32>,
    #[serde(default)]
    pub initial_crab_count: Option<u32>,
    #[serde(default)]
    pub jungle_cs_before_10_minutes: Option<f64>,
    #[serde(default)]
    pub jungler_takedowns_near_damaged_epic_monster: Option<u32>,
    #[serde(default)]
    pub kda: Option<f64>,
    #[serde(default)]
    pub kill_after_hidden_with_ally: Option<u32>,
    #[serde(default)]
    pub killed_champ_took_full_team_damage_survived: Option<u32>,
    #[serde(default)]
    pub killing_sprees: Option<u32>,
    #[serde(default)]
    pub kill_participation: Option<f64>,
    #[serde(default)]
    pub kills_near_enemy_turret: Option<u32>,
    #[serde(default)]
    pub kills_on_other_lanes_early_jungle_as_laner: Option<u32>,
    #[serde(default)]
    pub kills_on_recently_healed_by_aram_pack: Option<u32>,
    #[serde(default)]
    pub kills_under_own_turret: Option<u32>,
    #[serde(default)]
    pub kills_with_help_from_epic_monster: Option<u32>,
    #[serde(default)]
    pub knock_enemy_into_team_and_kill: Option<u32>,
    #[serde(default)]
    pub k_turrets_destroyed_before_plates_fall: Option<u32>,
    #[serde(default)]
    pub land_skill_shots_early_game: Option<u32>,
    #[serde(default)]
    pub lane_minions_first_10_minutes: Option<u32>,
    #[serde(default)]
    pub lost_an_inhibitor: Option<u32>,
    #[serde(default)]
    pub max_kill_deficit: Option<u32>,
    #[serde(default)]
    pub mejais_full_stack_in_time: Option<u32>,
    #[serde(default)]
    pub more_enemy_jungle_than_opponent: Option<f64>,
    #[serde(default)]
    pub multi_kill_one_spell: Option<u32>,
    #[serde(default)]
    pub multikills: Option<u32>,
    #[serde(default)]
    pub multikills_after_aggressive_flash: Option<u32>,
    #[serde(default)]
    pub multi_turret_rift_herald_count: Option<u32>,
    #[serde(default)]
    pub outer_turret_executes_before_10_minutes: Option<u32>,
    #[serde(default)]
    pub outnumbered_kills: Option<u32>,
    #[serde(default)]
    pub outnumbered_nexus_kill: Option<u32>,
    #[serde(default)]
    pub perfect_dragon_souls_taken: Option<u32>,
    #[serde(default)]
    pub perfect_game: Option<u32>,
    #[serde(default)]
    pub pick_kill_with_ally: Option<u32>,
    #[serde(default)]
    pub poro_explosions: Option<u32>,
    #[serde(default)]
    pub quick_cleanse: Option<u32>,
    #[serde(default)]
    pub quick_first_turret: Option<u32>,
    #[serde(default)]
    pub quick_solo_kills: Option<u32>,
    #[serde(default)]
    pub rift_herald_takedowns: Option<u32>,
    #[serde(default)]
    pub save_ally_from_death: Option<u32>,
    #[serde(default)]
    pub scuttle_crab_kills: Option<u32>,
    #[serde(default)]
    pub shortest_time_to_ace_from_first_takedown: Option<f64>,
    #[serde(default)]
    pub skillshots_dodged: Option<u32>,
    #[serde(default)]
    pub skillshots_hit: Option<u32>,
    #[serde(default)]
    pub snowballs_hit: Option<u32>,
    #[serde(default)]
    pub solo_baron_kills: Option<u32>,
    #[serde(default)]
    pub solo_kills: Option<u32>,
    #[serde(default)]
    pub stealth_wards_placed: Option<u32>,
    #[serde(default)]
    pub survived_single_digit_hp_count: Option<u32>,
    #[serde(default)]
    pub survived_three_immobilizes_in_fight: Option<u32>,
    #[serde(default)]
    pub takedown_on_first_turret: Option<u32>,
    #[serde(default)]
    pub takedowns: Option<u32>,
    #[serde(default)]
    pub takedowns_after_gaining_level_advantage: Option<u32>,
    #[serde(default)]
    pub takedowns_before_jungle_minion_spawn: Option<u32>,
    #[serde(default)]
    pub takedowns_first_x_minutes: Option<u32>,
    #[serde(default)]
    pub takedowns_in_alcove: Option<u32>,
    #[serde(default)]
    pub takedowns_in_enemy_fountain: Option<u32>,
    #[serde(default)]
    pub team_baron_kills: Option<u32>,
    #[serde(default)]
    pub team_damage_percentage: Option<f64>,
    #[serde(default)]
    pub team_elder_dragon_kills: Option<f64>,
    #[serde(default)]
    pub team_rift_herald_kills: Option<u32>,
    #[serde(default)]
    pub third_inhibitor_destroyed_time: Option<f64>,
    #[serde(default)]
    pub took_large_damage_survived: Option<u32>,
    #[serde(default)]
    pub turret_plates_taken: Option<u32>,
    #[serde(default)]
    pub turrets_taken_with_rift_herald: Option<u32>,
    #[serde(default)]
    pub turret_takedowns: Option<u32>,
    #[serde(default)]
    pub twenty_minions_in_3_seconds_count: Option<u32>,
    #[serde(default)]
    pub two_wards_one_sweeper_count: Option<u32>,
    #[serde(default)]
    pub unseen_recalls: Option<u32>,
    #[serde(default)]
    pub vision_score_advantage_lane_opponent: Option<f64>,
    #[serde(default)]
    pub vision_score_per_minute: Option<f64>,
    #[serde(default)]
    pub wards_guarded: Option<u32>,
    #[serde(default)]
    pub ward_takedowns: Option<u32>,
    #[serde(default)]
    pub ward_takedowns_before_20_m: Option<u32>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Missions {
    #[serde(default)]
    pub player_score_0: Option<u32>,
    #[serde(default)]
    pub player_score_1: Option<u32>,
    #[serde(default)]
    pub player_score_2: Option<u32>,
    #[serde(default)]
    pub player_score_3: Option<u32>,
    #[serde(default)]
    pub player_score_4: Option<u32>,
    #[serde(default)]
    pub player_score_5: Option<u32>,
    #[serde(default)]
    pub player_score_6: Option<u32>,
    #[serde(default)]
    pub player_score_7: Option<u32>,
    #[serde(default)]
    pub player_score_8: Option<u32>,
    #[serde(default)]
    pub player_score_9: Option<u32>,
    #[serde(default)]
    pub player_score_10: Option<u32>,
    #[serde(default)]
    pub player_score_11: Option<u32>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Perks {
    #[serde(default)]
    pub stat_perks: Option<PerkStats>,
    #[serde(default)]
    pub styles: Option<Vec<PerkStyle>>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PerkStats {
    #[serde(default)]
    pub defense: Option<u32>,
    #[serde(default)]
    pub flex: Option<u32>,
    #[serde(default)]
    pub offense: Option<u32>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PerkStyle {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub selections: Option<Vec<PerkStyleSelection>>,
    #[serde(default)]
    pub style: Option<u32>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PerkStyleSelection {
    #[serde(default)]
    pub perk: Option<u32>,
    #[serde(default)]
    pub var_1: Option<u32>,
    #[serde(default)]
    pub var_2: Option<u32>,
    #[serde(default)]
    pub var_3: Option<u32>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Team {
    #[serde(default)]
    pub bans: Option<Vec<Ban>>,
    #[serde(default)]
    pub objectives: Option<Objectives>,
    #[serde(default)]
    pub team_id: Option<u32>,
    #[serde(default)]
    pub win: Option<bool>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Ban {
    #[serde(default)]
    pub champion_id: Option<i32>,
    #[serde(default)]
    pub pick_turn: Option<u32>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Objectives {
    #[serde(default)]
    pub baron: Option<Objective>,
    #[serde(default)]
    pub champion: Option<Objective>,
    #[serde(default)]
    pub dragon: Option<Objective>,
    #[serde(default)]
    pub inhibitor: Option<Objective>,
    #[serde(default)]
    pub rift_herald: Option<Objective>,
    #[serde(default)]
    pub tower: Option<Objective>,
}

#[derive(Deserialize, Default, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Objective {
    #[serde(default)]
    pub first: Option<bool>,
    #[serde(default)]
    pub kills: Option<u32>,
}
