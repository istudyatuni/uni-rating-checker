SELECT tg_chat_id, case_number, degree, program_id, position, priority, total_scores, exam_scores
FROM results
WHERE tg_chat_id = :tg_chat_id
