//! Tauri application entry point

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use leptos::prelude::*;
use smart_ingredients_app::App;

fn main() {
    mount_to_body(App);
}
