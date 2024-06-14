-- Table for general match info
DROP TABLE IF EXISTS league_of_legends.flat.league_matches_match_outcomes;
CREATE TABLE league_of_legends.flat.league_matches_match_outcomes (
	account_name VARCHAR(50),
	match_id VARCHAR(25),
	game_id BIGINT,
	queue_id INT,
	game_mode VARCHAR(20),
	game_name VARCHAR(40),
	game_type VARCHAR(25),
	platform_id VARCHAR(4),
	game_version VARCHAR(20),
	end_of_game_result VARCHAR(35),
	game_creation_timestamp_utc TIMESTAMP,
	game_creation_unix_timestamp BIGINT,
	game_start_timestamp_utc TIMESTAMP,
	game_end_timestamp_utc TIMESTAMP,
	game_start_unix_timestamp BIGINT,
	game_end_unix_timestamp BIGINT,
	game_duration_seconds INT,
	tournament_code VARCHAR(40),
	load_id CHAR(36),
	time_inserted_utc TIMESTAMP DEFAULT TIMEZONE('UTC'::TEXT, NOW())
);


-- Example flattening query
SELECT
	a.account_name,
	a.match_id,
	(info->'gameId')::BIGINT AS game_id,
	(info->'queueId')::INT AS queue_id,
	(info->'gameMode')::VARCHAR(20) AS game_mode,
	(info->'gameName')::VARCHAR(40) AS game_name,
	(info->'gameType')::VARCHAR(25) AS game_type,
	(info->'platformId')::VARCHAR(4) AS platform_id,
	(info->'gameVersion')::VARCHAR(20) AS game_version,
	(info->'endOfGameResult')::VARCHAR(35) AS end_of_game_result,
	CAST(TO_TIMESTAMP((info->'gameCreation')::BIGINT/1000) AS TIMESTAMP) AS game_creation_timestamp_utc,
	(info->'gameCreation')::BIGINT AS game_creation_unix_timestamp,
	CAST(TO_TIMESTAMP((info->'gameStartTimestamp')::BIGINT/1000) AS TIMESTAMP) AS game_start_timestamp_utc,
	CAST(TO_TIMESTAMP((info->'gameEndTimestamp')::BIGINT/1000) AS TIMESTAMP) AS game_end_timestamp_utc,
	(info->'gameStartTimestamp')::BIGINT AS game_start_unix_timestamp,
	(info->'gameEndTimestamp')::BIGINT AS game_end_unix_timestamp,
	(info->'gameDuration')::BIGINT AS game_duration_seconds,
	(info->'tournamentCode')::VARCHAR(40) AS tournament_code,
	a.load_id
FROM
	league_of_legends.raw.league_matches AS a,
	jsonb_path_query(a.response_json, '$.info') AS info
WHERE
	match_id = 'NA1_5000712282';