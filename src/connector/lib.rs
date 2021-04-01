use chrono::TimeZone;
use chrono_tz::Tz;
use serenity::model::channel::Message;

pub fn format_message_to_telegram(msg: Message, guild_name: String, time_zone: &Tz) -> String {
	let converted_date = time_zone.from_utc_datetime(&msg.timestamp.naive_utc());

	format!(
		"
        Server Name: {} \n
        Author: {} \n
        Time and Date: {} \n
        Content: {}
        ",
		guild_name,
		msg.author.name,
		converted_date.to_rfc2822(),
		msg.content
	)
}
