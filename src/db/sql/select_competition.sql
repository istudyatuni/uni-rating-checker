SELECT case_number, position, priority, total_scores, exam_scores
FROM results
WHERE tg_chat_id = :tg_chat_id