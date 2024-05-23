use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchResponse {
    pub metadata: Option<MatchMetadata>,
    pub info: Option<MatchInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchMetadata {
    pub data_version: Option<String>,
    pub match_id: Option<String>,
    pub participants: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub end_of_game_result: Option<String>,
    pub game_creation: Option<u64>,
    pub game_duration: Option<u32>,
    pub game_end_timestamp: Option<u64>,
    pub game_id: Option<u64>,
    pub game_mode: Option<String>,
    pub game_name: Option<String>,
    pub game_start_timestamp: Option<u64>,
    pub game_type: Option<String>,
    pub game_version: Option<String>,
    pub participants: Option<Vec<MatchParticipant>>,
    pub platform_id: Option<String>,
    pub queue_id: Option<u32>,
    pub teams: Option<Vec<Team>>,
    pub tournament_code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct MatchParticipant {
    pub all_in_pings: Option<u32>,
    pub assist_me_pings: Option<u32>,
    pub assists: Option<u32>,
    pub baron_kills: Option<u32>,
    pub bounty_level: Option<u32>,
    pub champ_experience: Option<u32>,
    pub champ_level: Option<u32>,
    pub champion_id: Option<u32>,
    pub champion_name: Option<String>,
    pub champion_transform: Option<u32>,
    pub consumables_purchased: Option<u32>,
    pub challenges: Option<Challenges>,
    pub damage_dealt_to_buildings: Option<u32>,
    pub damage_dealt_to_objectives: Option<u32>,
    pub damage_dealt_to_turrets: Option<u32>,
    pub danger_pings: Option<u32>,
    pub damage_self_mitigated: Option<u32>,
    pub deaths: Option<u32>,
    pub detector_wards_placed: Option<u32>,
    pub double_kills: Option<u32>,
    pub dragon_kills: Option<u32>,
    pub eligible_for_progression: Option<bool>,
    pub enemy_missing_pings: Option<u32>,
    pub enemy_vision_pings: Option<u32>,
    pub first_blood_assist: Option<bool>,
    pub first_blood_kill: Option<bool>,
    pub first_tower_assist: Option<bool>,
    pub first_tower_kill: Option<bool>,
    pub game_ended_in_early_surrender: Option<bool>,
    pub game_ended_in_surrender: Option<bool>,
    pub hold_pings: Option<u32>,
    pub get_back_pings: Option<u32>,
    pub gold_earned: Option<u32>,
    pub gold_spent: Option<u32>,
    pub individual_position: Option<String>,
    pub inhibitor_kills: Option<u32>,
    pub inhibitor_takedowns: Option<u32>,
    pub inhibitors_lost: Option<u32>,
    pub item0: Option<u32>,
    pub item1: Option<u32>,
    pub item2: Option<u32>,
    pub item3: Option<u32>,
    pub item4: Option<u32>,
    pub item5: Option<u32>,
    pub item6: Option<u32>,
    pub items_purchased: Option<u32>,
    pub killing_sprees: Option<u32>,
    pub kills: Option<u32>,
    pub lane: Option<String>,
    pub largest_critical_strike: Option<u32>,
    pub largest_killing_spree: Option<u32>,
    pub largest_multi_kill: Option<u32>,
    pub longest_time_spent_living: Option<u32>,
    pub magic_damage_dealt: Option<u32>,
    pub magic_damage_dealt_to_champions: Option<u32>,
    pub magic_damage_taken: Option<u32>,
    pub missions: Option<Missions>,
    pub neutral_minions_killed: Option<u32>,
    pub need_vision_pings: Option<u64>,
    pub nexus_kills: Option<u32>,
    pub nexus_takedowns: Option<u32>,
    pub nexus_lost: Option<u32>,
    pub objectives_stolen: Option<u32>,
    pub objectives_stolen_assists: Option<u32>,
    pub participant_id: Option<u32>,
    pub penta_kills: Option<u32>,
    pub perks: Option<Perks>,
    pub physical_damage_dealt: Option<u32>,
    pub physical_damage_dealt_to_champions: Option<u32>,
    pub physical_damage_taken: Option<u32>,
    pub placement: Option<u32>,
    pub player_augment_1: Option<u32>,
    pub player_augment_2: Option<u32>,
    pub player_augment_3: Option<u32>,
    pub player_augment_4: Option<u32>,
    pub player_subteam_id: Option<u32>,
    pub push_pings: Option<u32>,
    pub profile_icon: Option<u32>,
    pub puuid: Option<String>,
    pub quadra_kills: Option<u32>,
    pub riot_id_game_name: Option<String>,
    pub riot_id_name: Option<String>,
    pub riot_id_tagline: Option<String>,
    pub role: Option<String>,
    pub sight_wards_bought_in_game: Option<u32>,
    pub spell_1_casts: Option<u32>,
    pub spell_2_casts: Option<u32>,
    pub spell_3_casts: Option<u32>,
    pub spell_4_casts: Option<u32>,
    pub subteam_placement: Option<u32>,
    pub summoner_1_casts: Option<u32>,
    pub summoner_1_id: Option<u32>,
    pub summoner_2_casts: Option<u32>,
    pub summoner_2_id: Option<u32>,
    pub summoner_id: Option<String>,
    pub summoner_level: Option<u32>,
    pub summoner_name: Option<String>,
    pub team_early_surrendered: Option<bool>,
    pub team_id: Option<u32>,
    pub team_position: Option<String>,
    pub time_CCing_others: Option<u32>,
    pub time_played: Option<u32>,
    pub total_damage_dealt: Option<u32>,
    pub total_damage_dealt_to_champions: Option<u32>,
    pub total_damage_shielded_on_teammates: Option<u32>,
    pub total_damage_taken: Option<u32>,
    pub total_heal: Option<u32>,
    pub total_heals_on_teammates: Option<u32>,
    pub total_minions_killed: Option<u32>,
    pub total_time_CC_dealt: Option<u32>,
    pub total_time_spent_dead: Option<u32>,
    pub total_units_healed: Option<u32>,
    pub triple_kills: Option<u32>,
    pub true_damage_dealt: Option<u64>,
    pub true_damage_dealt_to_champions: Option<u64>,
    pub true_damage_taken: Option<u64>,
    pub turret_kills: Option<u32>,
    pub turret_takedowns: Option<u32>,
    pub turrets_lost: Option<u32>,
    pub unreal_kills: Option<u32>,
    pub vision_score: Option<u32>,
    pub vision_cleared_pings: Option<u32>,
    pub vision_wards_bought_in_game: Option<u32>,
    pub wards_killed: Option<u32>,
    pub wards_placed: Option<u32>,
    pub win: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenges {
    #[serde(rename(deserialize = "12AssistStreakCount"))]
    pub twelve_assist_streak_count: Option<u32>,
    pub ability_uses: Option<u32>,
    pub aces_before_15_minutes: Option<u32>,
    pub allied_jungle_monster_kills: Option<u32>,
    pub baron_takedowns: Option<u32>,
    pub blast_cone_opposite_opponent_count: Option<u32>,
    pub bounty_gold: Option<u32>,
    pub buffs_stolen: Option<u32>,
    pub complete_support_quest_in_time: Option<u32>,
    pub control_wards_placed: Option<u32>,
    pub damage_per_minute: Option<u32>,
    pub damage_taken_on_team_percentage: Option<u32>,
    pub danced_with_rift_herald: Option<u32>,
    pub deaths_by_enemy_champs: Option<u32>,
    pub dodge_skill_shots_small_window: Option<u32>,
    pub double_aces: Option<u32>,
    pub dragon_takedowns: Option<u32>,
    pub legendary_item_used: Option<Vec<u32>>,
    pub effective_heal_and_shielding: Option<f64>,
    pub elder_dragon_kills_with_opposing_soul: Option<u32>,
    pub elder_dragon_multikills: Option<u32>,
    pub enemy_champion_immobilizations: Option<u32>,
    pub enemy_jungle_monster_kills: Option<u32>,
    pub epic_monster_kills_near_enemy_jungler: Option<u32>,
    pub epic_monster_steals: Option<u32>,
    pub epic_monster_stolen_without_smite: Option<u32>,
    pub first_turret_killed: Option<u32>,
    pub first_turret_killed_time: Option<f64>,
    pub flawless_aces: Option<u32>,
    pub full_team_takedown: Option<u32>,
    pub game_length: Option<f64>,
    pub get_takedowns_in_all_lanes_early_jungle_as_laner: Option<u32>,
    pub gold_per_minute: Option<f64>,
    pub had_open_nexus: Option<u32>,
    pub immobilize_and_kill_with_ally: Option<u32>,
    pub initial_buff_count: Option<u32>,
    pub initial_crab_count: Option<u32>,
    pub jungle_cs_before_10_minutes: Option<u32>,
    pub jungler_takedowns_near_damaged_epic_monster: Option<u32>,
    pub kda: Option<f64>,
    pub kill_after_hidden_with_ally: Option<u32>,
    pub killed_champ_took_full_team_damage_survived: Option<u32>,
    pub killing_sprees: Option<u32>,
    pub kill_participation: Option<f64>,
    pub kills_near_enemy_turret: Option<u32>,
    pub kills_on_other_lanes_early_jungle_as_laner: Option<u32>,
    pub kills_on_recently_healed_by_aram_pack: Option<u32>,
    pub kills_under_own_turret: Option<u32>,
    pub kills_with_help_from_epic_monster: Option<u32>,
    pub knock_enemy_into_team_and_kill: Option<u32>,
    pub k_turrets_destroyed_before_plates_fall: Option<u32>,
    pub land_skill_shots_early_game: Option<u32>,
    pub lane_minions_first_10_minutes: Option<u32>,
    pub lost_an_inhibitor: Option<u32>,
    pub max_kill_deficit: Option<u32>,
    pub mejais_full_stack_in_time: Option<u32>,
    pub more_enemy_jungle_than_opponent: Option<u32>,
    pub multi_kill_one_spell: Option<u32>,
    pub multikills: Option<u32>,
    pub multikills_after_aggressive_flash: Option<u32>,
    pub multi_turret_rift_herald_count: Option<u32>,
    pub outer_turret_executes_before_10_minutes: Option<u32>,
    pub outnumbered_kills: Option<u32>,
    pub outnumbered_nexus_kill: Option<u32>,
    pub perfect_dragon_souls_taken: Option<u32>,
    pub perfect_game: Option<u32>,
    pub pick_kill_with_ally: Option<u32>,
    pub poro_explosions: Option<u32>,
    pub quick_cleanse: Option<u32>,
    pub quick_first_turret: Option<u32>,
    pub quick_solo_kills: Option<u32>,
    pub rift_herald_takedowns: Option<u32>,
    pub save_ally_from_death: Option<u32>,
    pub scuttle_crab_kills: Option<u32>,
    pub shortest_time_to_ace_from_first_takedown: Option<f64>,
    pub skillshots_dodged: Option<u32>,
    pub skillshots_hit: Option<u32>,
    pub snowballs_hit: Option<u32>,
    pub solo_baron_kills: Option<u32>,
    pub solo_kills: Option<u32>,
    pub stealth_wards_placed: Option<u32>,
    pub survived_single_digit_hp_count: Option<u32>,
    pub survived_three_immobilizes_in_fight: Option<u32>,
    pub takedown_on_first_turret: Option<u32>,
    pub takedowns: Option<u32>,
    pub takedowns_after_gaining_level_advantage: Option<u32>,
    pub takedowns_before_jungle_minion_spawn: Option<u32>,
    pub takedowns_first_x_minutes: Option<u32>,
    pub takedowns_in_alcove: Option<u32>,
    pub takedowns_in_enemy_fountain: Option<u32>,
    pub team_baron_kills: Option<u32>,
    pub team_damage_percentage: Option<f64>,
    pub team_elder_dragon_kills: Option<f64>,
    pub team_rift_herald_kills: Option<u32>,
    pub took_large_damage_survived: Option<u32>,
    pub turret_plates_taken: Option<u32>,
    pub turrets_taken_with_rift_herald: Option<u32>,
    pub turret_takedowns: Option<u32>,
    pub twenty_minions_in_3_seconds_count: Option<u32>,
    pub two_wards_one_sweeper_count: Option<u32>,
    pub unseen_recalls: Option<u32>,
    pub vision_score_per_minute: Option<u32>,
    pub wards_guarded: Option<u32>,
    pub ward_takedowns: Option<u32>,
    pub ward_takedowns_before_20_m: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Missions {
    pub player_score_0: Option<u32>,
    pub player_score_1: Option<u32>,
    pub player_score_2: Option<u32>,
    pub player_score_3: Option<u32>,
    pub player_score_4: Option<u32>,
    pub player_score_5: Option<u32>,
    pub player_score_6: Option<u32>,
    pub player_score_7: Option<u32>,
    pub player_score_8: Option<u32>,
    pub player_score_9: Option<u32>,
    pub player_score_10: Option<u32>,
    pub player_score_11: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub stat_perks: Option<PerkStats>,
    pub styles: Option<Vec<PerkStyle>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerkStats {
    pub defense: Option<u32>,
    pub flex: Option<u32>,
    pub offense: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyle {
    pub description: Option<String>,
    pub selections: Option<Vec<PerkStyleSelection>>,
    pub style: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleSelection {
    pub perk: Option<u32>,
    pub var_1: Option<u32>,
    pub var_2: Option<u32>,
    pub var_3: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Option<Vec<Ban>>,
    pub objectives: Option<Objectives>,
    pub team_id: Option<u32>,
    pub win: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub champion_id: Option<u32>,
    pub pick_turn: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Objectives {
    pub baron: Option<Objective>,
    pub champion: Option<Objective>,
    pub dragon: Option<Objective>,
    pub inhibitor: Option<Objective>,
    pub rift_herald: Option<Objective>,
    pub tower: Option<Objective>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    pub first: Option<bool>,
    pub kills: Option<u32>,
}
