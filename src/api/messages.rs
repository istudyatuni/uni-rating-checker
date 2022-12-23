#![allow(non_upper_case_globals)]

use lazy_static::lazy_static;

const CARGO_PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const GIT_HASH: &str = env!("GIT_HASH");

pub const description: &str = "Это бот для отслеживания изменений рейтинга поступления в вуз. \
В данный момент поддерживается только магистратура ИТМО. Рейтинг проверяется каждые 10 минут";

pub const help: &str = r#"Доступные команды:

/watch - начать отслеживать рейтинг
/unwatch - прекратить отслеживать рейтинг
/all - показать все отслеживаемые рейтинги
/help - показать справку
/about - показать информацию о боте"#;

lazy_static! {
    pub static ref about: String = format!(
        "{description}\n\nИсходный код: {CARGO_PKG_REPOSITORY}\n\nКоммит \\(версия\\): `{GIT_HASH}`"
    );
    pub static ref start: String = format!("Привет\\! {description}\n\n{help}");
}

pub const unknown_message: &str = "Даже не знаю что сказать. Попробуй /help";

pub const incorrect_command_header: &str = "Ожидаемая команда:";

pub const error_occurred: &str = "Произошла ошибка. О ней уже сообщено";

pub const rating_not_found: &str = "По этим данным ничего не найдено";

pub const done: &str = "Сделано";

pub const easter_egg: &str = "О, ты нашел пасхалку. Мои поздравления";

// commands description

pub const watch_command: &str = r#"`/watch [uni] [degree] [program] [case number]`

`[uni]` - в данный момент значение игнорируется и используется `itmo`
`[degree]` - в данный момент поддерживается только `master`
`[program]` - номер программы, находится в ссылке: `https://abit.itmo.ru/program/[номер]`
`[case number]` - номер дела, находится в личном кабинете

Например: `/watch itmo master 15000 xx-xx-xx`"#;

pub const unwatch_command: &str = r#"1. `/unwatch [uni] [degree] [program] [case number]`
или
2. `/unwatch all` - отписаться от всех уведомлений

Можно повторить ранее отправленную команду `/watch`, изменив `watch` на `unwatch`

`[uni]` - в данный момент значение игнорируется и используется `itmo`
`[degree]` - в данный момент поддерживается только `master`
`[program]` - номер программы, находится в ссылке: `https://abit.itmo.ru/program/[номер]`
`[case number]` - номер дела, находится в личном кабинете

Например: `/unwatch itmo master 15000 xx-xx-xx`"#;
