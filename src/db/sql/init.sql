CREATE TABLE IF NOT EXISTS results (
	tg_chat_id TEXT NOT NULL,
	case_number TEXT NOT NULL,
	position INTEGER,
	priority INTEGER,
	total_scores INTEGER,
	exam_scores INTEGER,
	PRIMARY KEY (tg_chat_id, case_number)
)
