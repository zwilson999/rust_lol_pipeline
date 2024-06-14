-- Team objectives within each match
DROP TABLE IF EXISTS league_of_legends.flat.league_matches_team_objectives;
CREATE TABLE league_of_legends.flat.league_matches_team_objectives (
	account_name VARCHAR(50),
	match_id VARCHAR(25),
	team_id INT,
	baron_objective_first BOOLEAN,
	baron_objective_kills INT,
	tower_objective_first BOOLEAN,
	tower_objective_kills INT,
	dragon_objective_first BOOLEAN,
	dragon_objective_kills INT,
	champion_objective_first BOOLEAN,
	champion_objective_kills INT,
	inhibitor_objective_first BOOLEAN,
	inhibitor_objective_kills INT,
	rift_herald_objective_first BOOLEAN,
	rift_herald_objective_kills INT,
	load_id CHAR(36),
	time_inserted_utc TIMESTAMP DEFAULT TIMEZONE('UTC'::TEXT, NOW())
);

-- Example flattening query
SELECT
	a.account_name,
	a.match_id,
	(teams_arr->>'teamId')::INT AS team_id,
	-- Extract objective information at the team level
	(teams_arr->'objectives'->'baron'->'first')::BOOLEAN AS baron_objective_first,
	(teams_arr->'objectives'->'baron'->'kills')::INT AS baron_objective_kills,
	(teams_arr->'objectives'->'tower'->'first')::BOOLEAN AS tower_objective_first,
	(teams_arr->'objectives'->'tower'->'kills')::INT AS tower_objective_kills,
	(teams_arr->'objectives'->'dragon'->'first')::BOOLEAN AS dragon_objective_first,
	(teams_arr->'objectives'->'dragon'->'kills')::INT AS dragon_objective_kills,
	(teams_arr->'objectives'->'champion'->'first')::BOOLEAN AS champion_objective_first,
	(teams_arr->'objectives'->'champion'->'kills')::INT AS champion_objective_kills,
	(teams_arr->'objectives'->'inhibitor'->'first')::BOOLEAN AS inhibitor_objective_first,
	(teams_arr->'objectives'->'inhibitor'->'kills')::INT AS inhibitor_objective_kills,
	(teams_arr->'objectives'->'riftHerald'->'first')::BOOLEAN AS rift_herald_objective_first,
	(teams_arr->'objectives'->'riftHerald'->'kills')::INT AS rift_herald_objective_kills,
	a.load_id
FROM
	league_of_legends.raw.league_matches AS a,
	jsonb_path_query(a.response_json, '$.info.teams') AS teams,
	jsonb_array_elements(teams) AS teams_arr
WHERE
	match_id = 'NA1_5000712282';