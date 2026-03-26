use core::ffi::{c_char, c_int};
use std::ffi::CString;

#[link(name = "myteams")]
unsafe extern "C" {
    fn server_event_team_created(team_uuid: *const c_char, team_name: *const c_char, user_uuid: *const c_char) -> c_int;
    fn server_event_channel_created(team_uuid: *const c_char, channel_uuid: *const c_char, channel_name: *const c_char) -> c_int;
    fn server_event_thread_created(channel_uuid: *const c_char, thread_uuid: *const c_char, user_uuid: *const c_char, thread_title: *const c_char, thread_body: *const c_char) -> c_int;
    fn server_event_reply_created(thread_uuid: *const c_char, user_uuid: *const c_char, reply_body: *const c_char) -> c_int;
    fn server_event_user_subscribed(team_uuid: *const c_char, user_uuid: *const c_char) -> c_int;
    fn server_event_user_unsubscribed(team_uuid: *const c_char, user_uuid: *const c_char) -> c_int;
    fn server_event_user_created(user_uuid: *const c_char, user_name: *const c_char) -> c_int;
    fn server_event_user_loaded(user_uuid: *const c_char, user_name: *const c_char) -> c_int;
    fn server_event_user_logged_in(user_uuid: *const c_char) -> c_int;
    fn server_event_user_logged_out(user_uuid: *const c_char) -> c_int;
    fn server_event_private_message_sended(sender_uuid: *const c_char, receiver_uuid: *const c_char, message_body: *const c_char) -> c_int;
}

fn cstr(s: &str) -> CString {
    CString::new(s).expect("string contains null byte")
}

pub fn event_team_created(team_uuid: &str, team_name: &str, user_uuid: &str) -> c_int {
    unsafe {
        server_event_team_created(
            cstr(team_uuid).as_ptr(),
            cstr(team_name).as_ptr(),
            cstr(user_uuid).as_ptr(),
        )
    }
}

pub fn event_channel_created(team_uuid: &str, channel_uuid: &str, channel_name: &str) -> c_int {
    unsafe {
        server_event_channel_created(
            cstr(team_uuid).as_ptr(),
            cstr(channel_uuid).as_ptr(),
            cstr(channel_name).as_ptr(),
        )
    }
}

pub fn event_thread_created(
    channel_uuid: &str,
    thread_uuid: &str,
    user_uuid: &str,
    thread_title: &str,
    thread_body: &str,
) -> c_int {
    unsafe {
        server_event_thread_created(
            cstr(channel_uuid).as_ptr(),
            cstr(thread_uuid).as_ptr(),
            cstr(user_uuid).as_ptr(),
            cstr(thread_title).as_ptr(),
            cstr(thread_body).as_ptr(),
        )
    }
}

pub fn event_reply_created(thread_uuid: &str, user_uuid: &str, reply_body: &str) -> c_int {
    unsafe {
        server_event_reply_created(
            cstr(thread_uuid).as_ptr(),
            cstr(user_uuid).as_ptr(),
            cstr(reply_body).as_ptr(),
        )
    }
}

pub fn event_user_subscribed(team_uuid: &str, user_uuid: &str) -> c_int {
    unsafe {
        server_event_user_subscribed(
            cstr(team_uuid).as_ptr(),
            cstr(user_uuid).as_ptr(),
        )
    }
}

pub fn event_user_unsubscribed(team_uuid: &str, user_uuid: &str) -> c_int {
    unsafe {
        server_event_user_unsubscribed(
            cstr(team_uuid).as_ptr(),
            cstr(user_uuid).as_ptr(),
        )
    }
}

pub fn event_user_created(user_uuid: &str, user_name: &str) -> c_int {
    unsafe {
        server_event_user_created(
            cstr(user_uuid).as_ptr(),
            cstr(user_name).as_ptr(),
        )
    }
}

pub fn event_user_loaded(user_uuid: &str, user_name: &str) -> c_int {
    unsafe {
        server_event_user_loaded(
            cstr(user_uuid).as_ptr(),
            cstr(user_name).as_ptr(),
        )
    }
}

pub fn event_user_logged_in(user_uuid: &str) -> c_int {
    unsafe { server_event_user_logged_in(cstr(user_uuid).as_ptr()) }
}

pub fn event_user_logged_out(user_uuid: &str) -> c_int {
    unsafe { server_event_user_logged_out(cstr(user_uuid).as_ptr()) }
}

pub fn event_private_message_sent(sender_uuid: &str, receiver_uuid: &str, message_body: &str) -> c_int {
    unsafe {
        server_event_private_message_sended(
            cstr(sender_uuid).as_ptr(),
            cstr(receiver_uuid).as_ptr(),
            cstr(message_body).as_ptr(),
        )
    }
}

