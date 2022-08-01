UPDATE results
SET position = ?3,
	priority = ?4,
	total_scores = ?5,
	exam_scores = ?6
WHERE tg_chat_id = ?1 AND case_number = ?2
