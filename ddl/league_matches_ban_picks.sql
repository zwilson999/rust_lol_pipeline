-- Create table to analyze ban picks and ban order per player on each team within a match
-- Granularity is participant level (10 per normal draft match)
DROP TABLE IF EXISTS league_of_legends.flat.league_matches_ban_picks;
CREATE TABLE league_of_legends.flat.league_matches_ban_picks (
	account_name VARCHAR(50),
	match_id VARCHAR(25),
	team_id INT,
	ban_pick_turn INT,
	ban_pick_champion_id INT,
	load_id CHAR(36),
	time_inserted_utc TIMESTAMP DEFAULT TIMEZONE('UTC'::TEXT, NOW())
);

-- Example query
SELECT
	a.account_name,
	a.match_id,
	(teams_arr->>'teamId')::INT AS team_id,
	(bans->>'pickTurn')::INT AS ban_pick_turn,
	(bans->>'championId')::INT AS ban_pick_champion_id,
	a.load_id
FROM
	league_of_legends.raw.league_matches AS a,
	jsonb_path_query(a.response_json, '$.info.teams') AS teams,
	jsonb_array_elements(teams) AS teams_arr,
	jsonb_array_elements(teams_arr->'bans') AS bans
WHERE
	match_id = 'NA1_5000712282';