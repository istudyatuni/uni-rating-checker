UPDATE results
SET position = ?4,
	priority = ?5,
	total_scores = ?6,
	exam_scores = ?7
WHERE tg_chat_id = ?1 AND case_number = ?2 AND program_id = ?3
