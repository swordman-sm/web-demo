mod tm_user;
mod admin_roles;
mod admins;
mod ads;
mod menus;
mod user_levels;
mod users;
mod video_categories;
mod videos;
mod video_tags;
mod video_replies;
mod watch_records;

pub use tm_user::TmUser;
use regex::Regex;


lazy_static! {
    pub static ref RE_USERNAME: Regex = { Regex::new(r"^[a-zA-Z]+[a-zA-Z_0-9]{4,19}$").unwrap() };
}
