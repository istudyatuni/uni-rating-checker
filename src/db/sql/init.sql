CREATE TABLE IF NOT EXISTS results (
	tg_chat_id TEXT NOT NULL,
	case_number TEXT NOT NULL,
	degree TEXT NOT NULL,
	program_id TEXT NOT NULL,
	position INTEGER,
	priority INTEGER,
	total_scores NUMERIC,
	exam_scores NUMERIC,
	PRIMARY KEY (tg_chat_id, case_number, degree, program_id)
);
CREATE TABLE IF NOT EXISTS programs (
	id NUMERIC NOT NULL,
	uni TEXT NOT NULL,
	name TEXT NOT NULL,
	PRIMARY KEY (id, uni)
);
CREATE TABLE IF NOT EXISTS cache (
	key TEXT NOT NULL,
	value TEXT
);
CREATE TABLE IF NOT EXISTS deleted (
	tg_chat_id TEXT NOT NULL
);
