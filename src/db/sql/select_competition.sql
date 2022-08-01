SELECT position, priority, total_scores, exam_scores
FROM results
WHERE tg_chat_id = :tg_chat_id AND case_number = :case_number AND program_id = :program_id
