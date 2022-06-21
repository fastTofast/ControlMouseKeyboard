use std::{thread, time};

use enigo::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Deserialize, Serialize, Debug)]
struct OpItem {
    position: Pos,
    count: i32,
    time: u64,
    word: String,
    optype: String,
}
#[tauri::command]
pub async fn handle_mouse_keyboard(pos_str: String) -> Result<String, String> {
    match serde_json::from_str::<Vec<OpItem>>(pos_str.as_str()) {
        Ok(list) => process(list),
        Err(e) => println!("{:?}-----{}", e, pos_str),
    }
    // enigo.mouse_click(MouseButton::Left);
    // enigo.mouse_click(MouseButton::Left);
    println!("22222222222222");
    // enigo.mouse_click(MouseButton::Right);
    // enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}hello world{-SHIFT}");
    return Ok("OK".into());
}

fn process(list: Vec<OpItem>) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 200);
    for item in list {
        for n in 0..item.count {
            if item.optype == "CLICKRIGHT" {
                enigo.mouse_click(MouseButton::Right);
            } else if item.optype == "CLICKLEFT" {
                enigo.mouse_click(MouseButton::Left);
            } else if item.optype == "MOVE" {
                println!("iMOVE");
                enigo.mouse_move_to(item.position.x, item.position.y);
            } else if item.optype == "MOVERELATE" {
                println!("MOVERELATE");
                enigo.mouse_move_relative(item.position.x, item.position.y);
            } else if item.optype == "KEYCLICK" {
                enigo.mouse_move_relative(item.position.x, item.position.y);
            } else if item.optype == "KEYUP" || item.optype == "KEYDOWN" {
                if let Some(char_first) = item.word.chars().next() {
                    enigo.key_click(Key::Layout(char_first));
                };
            } else if item.optype == "KEYINPUT" {
                enigo.key_sequence(&item.word)
            } else if item.optype == "DELAY" {
                std::thread::sleep(std::time::Duration::from_millis(item.time))
            }
        }
    }
}
