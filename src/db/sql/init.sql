CREATE TABLE IF NOT EXISTS results (
	tg_chat_id TEXT NOT NULL,
	case_number TEXT NOT NULL,
	program_id TEXT NOT NULL,
	position INTEGER,
	priority INTEGER,
	total_scores INTEGER,
	exam_scores INTEGER,
	PRIMARY KEY (tg_chat_id, case_number, program_id)
);
CREATE TABLE IF NOT EXISTS programs (
	id NUMERIC NOT NULL,
	uni TEXT NOT NULL,
	name TEXT NOT NULL,
	PRIMARY KEY (id, uni)
)
