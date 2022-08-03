SELECT COUNT(*) FROM (SELECT COUNT(tg_chat_id) FROM results GROUP BY tg_chat_id)
