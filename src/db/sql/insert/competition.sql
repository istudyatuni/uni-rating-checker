INSERT OR IGNORE INTO results (
	tg_chat_id,
	case_number,
	degree,
	program_id,
	position,
	priority,
	total_scores,
	exam_scores
)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
