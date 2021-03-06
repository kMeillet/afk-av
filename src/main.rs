use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use rand::prelude::*;
use std::thread;
use std::time::Duration;

static WOW_ICON_WIDTH: i32 = 32;
static WOW_ICON_HEIGHT: i32 = 32;

static JOIN_BG_POS_X: i32 = 140;
static JOIN_BG_POS_Y: i32 = 325;

static INCLUSIVE_MIN_SLEEP_MS: u64 = 2000;
static EXCLUSIVE_MIN_SLEEP_MS: u64 = 10000;

static INTERACT_KEYBIND: Key = Key::F10;

fn sleep_random_timer() {
    let mut rng = thread_rng();
    let rng_wait_time = rng.gen_range(INCLUSIVE_MIN_SLEEP_MS, EXCLUSIVE_MIN_SLEEP_MS);
    let wait_time_duration = Duration::from_millis(rng_wait_time);

    println!("Sleep timer set to {}ms", rng_wait_time);

    thread::sleep(wait_time_duration);
}

fn main() -> ! {
    let mut enigo = Enigo::new();

    loop {
        // Total of 9 delay, best case is 18s, worst case is 90s (1mn30).
        // Move to macro A.
        enigo.mouse_move_to(WOW_ICON_WIDTH, WOW_ICON_HEIGHT);
        // Delay.
        sleep_random_timer();
        // Click.
        enigo.mouse_click(MouseButton::Left);
        // Delay.
        sleep_random_timer();
        // Use interact keybind.
        enigo.key_click(INTERACT_KEYBIND);
        // Delay.
        sleep_random_timer();
        // Click to show "Join BG".
        enigo.mouse_move_to(JOIN_BG_POS_X, JOIN_BG_POS_Y);
        // Delay.
        sleep_random_timer();
        // Click.
        enigo.mouse_click(MouseButton::Left);
        // Delay.
        sleep_random_timer();
        // Move to macro B.
        enigo.mouse_move_to(WOW_ICON_WIDTH * 2, WOW_ICON_HEIGHT);
        // Delay.
        sleep_random_timer();
        // Click.
        enigo.mouse_click(MouseButton::Left);
        // Delay.
        sleep_random_timer();
        // Anti-AFK macro.
        enigo.mouse_move_to(WOW_ICON_WIDTH * 4, WOW_ICON_HEIGHT);
        // Delay.
        sleep_random_timer();
        // Click.
        enigo.mouse_click(MouseButton::Left);
        // Delay.
        sleep_random_timer();
    }
}
