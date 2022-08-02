UPDATE results
SET position = ?5,
	priority = ?6,
	total_scores = ?7,
	exam_scores = ?8
WHERE tg_chat_id = ?1 AND case_number = ?2 AND program_id = ?3 AND degree = ?4
