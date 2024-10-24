-- Create table to analyze ban picks and ban order per player on each team within a match
-- Granularity is participant level (10 per normal draft match)
DROP TABLE IF EXISTS league_of_legends.flat.league_matches_match_outcomes;
CREATE TABLE league_of_legends.flat.league_matches_match_outcomes (
	account_name VARCHAR(50),
	match_id VARCHAR(25),
	team_id INT,
	team_victory BOOLEAN,
	load_id CHAR(36),
	time_inserted_utc TIMESTAMP DEFAULT TIMEZONE('UTC'::TEXT, NOW())
);

-- Example query
/*
SELECT
	a.account_name,
	a.match_id,
	(teams_arr->>'teamId')::INT AS team_id,
	(teams_arr->>'win')::BOOLEAN AS team_victory,
	a.load_id
FROM
	league_of_legends.raw.league_matches AS a,
	jsonb_path_query(a.response_json, '$.info.teams') AS teams,
	jsonb_array_elements(teams) AS teams_arr
WHERE
	match_id = '';
*/
