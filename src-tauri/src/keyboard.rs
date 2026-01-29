use enigo::{Enigo, Key, Keyboard, Settings};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::config::Config;

static ENIGO: Lazy<Mutex<Enigo>> =
    Lazy::new(|| Mutex::new(Enigo::new(&Settings::default()).expect("Failed to create Enigo")));

fn string_to_key(key_str: &str) -> Option<Key> {
    match key_str.to_lowercase().as_str() {
        // Letters
        "a" => Some(Key::Unicode('a')),
        "b" => Some(Key::Unicode('b')),
        "c" => Some(Key::Unicode('c')),
        "d" => Some(Key::Unicode('d')),
        "e" => Some(Key::Unicode('e')),
        "f" => Some(Key::Unicode('f')),
        "g" => Some(Key::Unicode('g')),
        "h" => Some(Key::Unicode('h')),
        "i" => Some(Key::Unicode('i')),
        "j" => Some(Key::Unicode('j')),
        "k" => Some(Key::Unicode('k')),
        "l" => Some(Key::Unicode('l')),
        "m" => Some(Key::Unicode('m')),
        "n" => Some(Key::Unicode('n')),
        "o" => Some(Key::Unicode('o')),
        "p" => Some(Key::Unicode('p')),
        "q" => Some(Key::Unicode('q')),
        "r" => Some(Key::Unicode('r')),
        "s" => Some(Key::Unicode('s')),
        "t" => Some(Key::Unicode('t')),
        "u" => Some(Key::Unicode('u')),
        "v" => Some(Key::Unicode('v')),
        "w" => Some(Key::Unicode('w')),
        "x" => Some(Key::Unicode('x')),
        "y" => Some(Key::Unicode('y')),
        "z" => Some(Key::Unicode('z')),

        // Special keys
        "space" => Some(Key::Space),
        "enter" | "return" => Some(Key::Return),
        "escape" | "esc" => Some(Key::Escape),
        "tab" => Some(Key::Tab),
        "backspace" => Some(Key::Backspace),

        // Arrow keys
        "up" => Some(Key::UpArrow),
        "down" => Some(Key::DownArrow),
        "left" => Some(Key::LeftArrow),
        "right" => Some(Key::RightArrow),

        // Punctuation
        ";" | "semicolon" => Some(Key::Unicode(';')),
        "'" | "apostrophe" => Some(Key::Unicode('\'')),
        "," | "comma" => Some(Key::Unicode(',')),
        "." | "period" => Some(Key::Unicode('.')),
        "/" | "slash" => Some(Key::Unicode('/')),

        _ => {
            // Try single character
            if key_str.len() == 1 {
                Some(Key::Unicode(key_str.chars().next().unwrap()))
            } else {
                None
            }
        }
    }
}

pub fn press_key(key_name: &str, config: &Config) {
    // println!("[DEBUG] press_key called for: {}", key_name);
    let binding = match config.key_bindings.get(key_name) {
        Some(b) => b.clone(),
        None => {
            println!("[ERROR] No binding found for key: {}", key_name);
            return;
        }
    };

    let key = match string_to_key(&binding) {
        Some(k) => k,
        None => {
            println!("[ERROR] Failed to convert binding to Key: {}", binding);
            return;
        }
    };

    // Press the key directly without state checking
    match ENIGO.lock() {
        Ok(mut enigo) => {
            enigo.key(key, enigo::Direction::Press);
            // println!("[DOWN] {}: {} -> {:?}", key_name, binding, key);
        }
        Err(e) => println!("[ERROR] Failed to lock ENIGO: {}", e),
    }
}

pub fn release_key(key_name: &str, config: &Config) {
    let binding = match config.key_bindings.get(key_name) {
        Some(b) => b.clone(),
        None => return,
    };

    let key = match string_to_key(&binding) {
        Some(k) => k,
        None => return,
    };

    // Release the key directly
    match ENIGO.lock() {
        Ok(mut enigo) => {
            enigo.key(key, enigo::Direction::Release);
            // println!("[UP] {}: {} -> {:?}", key_name, binding, key);
        }
        Err(e) => println!("[ERROR] Failed to lock ENIGO for release: {}", e),
    }
}

pub fn release_all(_config: &Config) {
    // PRESSED_KEYS tracking removed for performance/reliability.
    // Client is responsible for releasing keys.
}
