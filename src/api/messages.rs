#![allow(non_upper_case_globals)]

use const_format::formatcp;

pub const about: &str = "Это бот для отслеживания рейтинга поступления в вуз. В данный момент поддерживается только магистратура ИТМО. Рейтинг проверяется каждые 10 минут";
pub const help: &str = r#"Доступные команды:
/watch - начать отслеживать рейтинг
/help - показать справку"#;
// wait for https://github.com/rodrimati1992/const_format_crates/issues/44 to rename to 'start'
pub const start_message: &str = formatcp!("Привет\\! {about}\n{help}");
pub const unknown_message: &str = "Даже не знаю что сказать. Попробуй /help";
pub const incorrect_command_header: &str = "Ожидаемая команда:";
pub const error_occurred: &str = "Произошла ошибка. О ней уже сообщено";
pub const rating_not_found: &str = "По этим данным ничего не найдено";

// commands description

pub const watch_command: &str = r#"`/watch [uni] [degree] [program] [case number]`

`[uni]` - в данный момент значение игнорируется и используется `itmo`
`[degree]` - в данный момент поддерживается только `master`
`[program]` - номер программы, находится в ссылке: `https://abit.itmo.ru/program/[номер]`
`[case number]` - номер дела, находится в личном кабинете
Например: `/watch itmo master 15000 xx-xx-xx`"#;
