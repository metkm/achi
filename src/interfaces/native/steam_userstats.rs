use crate::interfaces::{interface::VTable, native::CallableDefaultNativeFunction};

use std::ffi::{c_char, c_int};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ISteamUserStats013Functions {
    pub get_stat_float: CallableDefaultNativeFunction,
    pub get_stat_integer: CallableDefaultNativeFunction,
    pub set_stat_float: CallableDefaultNativeFunction,
    pub set_stat_integer: CallableDefaultNativeFunction,
    pub update_avg_rate_stat: CallableDefaultNativeFunction,
    pub get_achievement: CallableDefaultNativeFunction,

    pub set_achievement: unsafe extern "C" fn(this: *mut c_int, info_id: *const c_char) -> bool,
    pub clear_achievement: unsafe extern "C" fn(this: *mut c_int, info_id: *const c_char) -> bool,

    pub get_achievement_and_unlock_time: CallableDefaultNativeFunction,
    pub store_stats: CallableDefaultNativeFunction,
    pub get_achievement_icon: CallableDefaultNativeFunction,
    pub get_achievement_display_attribute: CallableDefaultNativeFunction,
    pub indicate_achievement_progress: CallableDefaultNativeFunction,
    pub get_num_achievements: CallableDefaultNativeFunction,
    pub get_achievement_name: CallableDefaultNativeFunction,
    pub request_user_stats: CallableDefaultNativeFunction,
    pub get_user_stat_float: CallableDefaultNativeFunction,
    pub get_user_stat_int: CallableDefaultNativeFunction,
    pub get_user_achievement: CallableDefaultNativeFunction,
    pub get_user_achievement_and_unlock_time: CallableDefaultNativeFunction,
    pub reset_all_stats: CallableDefaultNativeFunction,
    pub find_or_create_leaderboard: CallableDefaultNativeFunction,
    pub find_leaderboard: CallableDefaultNativeFunction,
    pub get_leaderboard_name: CallableDefaultNativeFunction,
    pub get_leaderboard_entry_count: CallableDefaultNativeFunction,
    pub get_leaderboard_sort_method: CallableDefaultNativeFunction,
    pub get_leaderboard_display_type: CallableDefaultNativeFunction,
    pub download_leaderboard_entries: CallableDefaultNativeFunction,
    pub download_leaderboard_entries_for_users: CallableDefaultNativeFunction,
    pub get_downloaded_leaderboard_entry: CallableDefaultNativeFunction,
    pub upload_leaderboard_score: CallableDefaultNativeFunction,
    pub attach_leaderboard_ugc: CallableDefaultNativeFunction,
    pub get_number_of_current_players: CallableDefaultNativeFunction,
    pub request_global_achievement_percentages: CallableDefaultNativeFunction,
    pub get_most_achieved_achievement_info: CallableDefaultNativeFunction,
    pub get_next_most_achieved_achievement_info: CallableDefaultNativeFunction,
    pub get_achievement_achieved_percent: CallableDefaultNativeFunction,
    pub request_global_stats: CallableDefaultNativeFunction,
    pub get_global_stat_float: CallableDefaultNativeFunction,
    pub get_global_stat_integer: CallableDefaultNativeFunction,
    pub get_global_stat_history_float: CallableDefaultNativeFunction,
    pub get_global_stat_history_integer: CallableDefaultNativeFunction,
    pub get_achievement_progress_limits_float: CallableDefaultNativeFunction,
    pub get_achievement_progress_limits_integer: CallableDefaultNativeFunction,
}

#[repr(C)]
pub struct ISteamUserStats013 {
    pub vtable: *const ISteamUserStats013Functions,
}

impl VTable for ISteamUserStats013 {
    type Functions = ISteamUserStats013Functions;

    fn vtable(&self) -> *const Self::Functions {
        self.vtable
    }
}
