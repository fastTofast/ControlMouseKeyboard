use std::{thread, time};

use enigo::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Pos {
    x: i32,
    y: i32,
    count: i32,
}
#[tauri::command]
pub async fn move_mouse(pos_str: String) -> Result<(), String> {
    let pos: Pos = serde_json::from_str(pos_str.as_str()).unwrap();
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(pos.x, pos.y);
    // enigo.mouse_click(MouseButton::Right);
    // enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}hello world{-SHIFT}");
    let mut count = 0;
    while count < pos.count {
        count = count + 1;
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(500));
    }
    Ok(())
}

#[tauri::command]
pub fn move_mouse2() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 200);
    // enigo.mouse_click(MouseButton::Left);
    // enigo.mouse_click(MouseButton::Left);
    println!("22222222222222");
    // enigo.mouse_click(MouseButton::Right);
    // enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}hello world{-SHIFT}");
}
