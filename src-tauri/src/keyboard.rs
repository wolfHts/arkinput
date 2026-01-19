use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use rdev::{listen, Event, EventType, Key};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::database::Database;
use crate::models::InputRecord;
use crate::window::get_active_window;

static DB: OnceCell<Arc<Database>> = OnceCell::new();
static EXCLUDED_APPS: OnceCell<Mutex<Vec<String>>> = OnceCell::new();

struct InputBuffer {
    content: String,
    app_name: String,
    window_title: Option<String>,
    last_input_time: Instant,
    key_count: i32,
}

impl InputBuffer {
    fn new() -> Self {
        Self {
            content: String::new(),
            app_name: String::new(),
            window_title: None,
            last_input_time: Instant::now(),
            key_count: 0,
        }
    }

    fn reset(&mut self) {
        self.content.clear();
        self.app_name.clear();
        self.window_title = None;
        self.key_count = 0;
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

pub fn init_database(db: Arc<Database>) {
    let _ = DB.set(db);
    let _ = EXCLUDED_APPS.set(Mutex::new(Vec::new()));
}

pub fn set_excluded_apps(apps: Vec<String>) {
    if let Some(excluded) = EXCLUDED_APPS.get() {
        let mut guard = excluded.lock();
        *guard = apps;
    }
}

fn is_app_excluded(app_name: &str) -> bool {
    if let Some(excluded) = EXCLUDED_APPS.get() {
        let guard = excluded.lock();
        guard.iter().any(|a| a.eq_ignore_ascii_case(app_name))
    } else {
        false
    }
}

fn key_to_char(key: Key, shift: bool) -> Option<char> {
    match key {
        Key::KeyA => Some(if shift { 'A' } else { 'a' }),
        Key::KeyB => Some(if shift { 'B' } else { 'b' }),
        Key::KeyC => Some(if shift { 'C' } else { 'c' }),
        Key::KeyD => Some(if shift { 'D' } else { 'd' }),
        Key::KeyE => Some(if shift { 'E' } else { 'e' }),
        Key::KeyF => Some(if shift { 'F' } else { 'f' }),
        Key::KeyG => Some(if shift { 'G' } else { 'g' }),
        Key::KeyH => Some(if shift { 'H' } else { 'h' }),
        Key::KeyI => Some(if shift { 'I' } else { 'i' }),
        Key::KeyJ => Some(if shift { 'J' } else { 'j' }),
        Key::KeyK => Some(if shift { 'K' } else { 'k' }),
        Key::KeyL => Some(if shift { 'L' } else { 'l' }),
        Key::KeyM => Some(if shift { 'M' } else { 'm' }),
        Key::KeyN => Some(if shift { 'N' } else { 'n' }),
        Key::KeyO => Some(if shift { 'O' } else { 'o' }),
        Key::KeyP => Some(if shift { 'P' } else { 'p' }),
        Key::KeyQ => Some(if shift { 'Q' } else { 'q' }),
        Key::KeyR => Some(if shift { 'R' } else { 'r' }),
        Key::KeyS => Some(if shift { 'S' } else { 's' }),
        Key::KeyT => Some(if shift { 'T' } else { 't' }),
        Key::KeyU => Some(if shift { 'U' } else { 'u' }),
        Key::KeyV => Some(if shift { 'V' } else { 'v' }),
        Key::KeyW => Some(if shift { 'W' } else { 'w' }),
        Key::KeyX => Some(if shift { 'X' } else { 'x' }),
        Key::KeyY => Some(if shift { 'Y' } else { 'y' }),
        Key::KeyZ => Some(if shift { 'Z' } else { 'z' }),
        Key::Num0 => Some(if shift { ')' } else { '0' }),
        Key::Num1 => Some(if shift { '!' } else { '1' }),
        Key::Num2 => Some(if shift { '@' } else { '2' }),
        Key::Num3 => Some(if shift { '#' } else { '3' }),
        Key::Num4 => Some(if shift { '$' } else { '4' }),
        Key::Num5 => Some(if shift { '%' } else { '5' }),
        Key::Num6 => Some(if shift { '^' } else { '6' }),
        Key::Num7 => Some(if shift { '&' } else { '7' }),
        Key::Num8 => Some(if shift { '*' } else { '8' }),
        Key::Num9 => Some(if shift { '(' } else { '9' }),
        Key::Space => Some(' '),
        Key::Minus => Some(if shift { '_' } else { '-' }),
        Key::Equal => Some(if shift { '+' } else { '=' }),
        Key::LeftBracket => Some(if shift { '{' } else { '[' }),
        Key::RightBracket => Some(if shift { '}' } else { ']' }),
        Key::BackSlash => Some(if shift { '|' } else { '\\' }),
        Key::SemiColon => Some(if shift { ':' } else { ';' }),
        Key::Quote => Some(if shift { '"' } else { '\'' }),
        Key::Comma => Some(if shift { '<' } else { ',' }),
        Key::Dot => Some(if shift { '>' } else { '.' }),
        Key::Slash => Some(if shift { '?' } else { '/' }),
        Key::BackQuote => Some(if shift { '~' } else { '`' }),
        _ => None,
    }
}

fn save_buffer(buffer: &mut InputBuffer) {
    if buffer.is_empty() {
        return;
    }

    if let Some(db) = DB.get() {
        let record = InputRecord {
            id: None,
            timestamp: chrono::Utc::now(),
            app_name: buffer.app_name.clone(),
            window_title: buffer.window_title.clone(),
            content: buffer.content.clone(),
            key_count: buffer.key_count,
            created_at: None,
        };

        if let Err(e) = db.insert_record(&record) {
            eprintln!("Failed to save input record: {}", e);
        }
    }

    buffer.reset();
}

pub fn start_keyboard_listener() {
    thread::spawn(|| {
        let buffer = Arc::new(Mutex::new(InputBuffer::new()));
        let buffer_clone = buffer.clone();
        let shift_pressed = Arc::new(Mutex::new(false));
        let shift_clone = shift_pressed.clone();

        // Flush timer thread
        let buffer_for_timer = buffer.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(100));
                let mut buf = buffer_for_timer.lock();
                if !buf.is_empty() && buf.last_input_time.elapsed() > Duration::from_millis(500) {
                    save_buffer(&mut buf);
                }
            }
        });

        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    // Track shift state
                    if matches!(key, Key::ShiftLeft | Key::ShiftRight) {
                        *shift_clone.lock() = true;
                        return;
                    }

                    // Get current window info
                    let window_info = get_active_window();
                    if window_info.is_none() {
                        return;
                    }
                    let window_info = window_info.unwrap();

                    // Check if app is excluded
                    if is_app_excluded(&window_info.app_name) {
                        return;
                    }

                    let mut buf = buffer_clone.lock();
                    let now = Instant::now();

                    // Check if we need to start a new buffer
                    let app_changed = !buf.is_empty() && buf.app_name != window_info.app_name;
                    let timeout = !buf.is_empty() && buf.last_input_time.elapsed() > Duration::from_millis(500);

                    if app_changed || timeout {
                        save_buffer(&mut buf);
                    }

                    // Handle special keys
                    match key {
                        Key::Return => {
                            buf.content.push_str("[Enter]");
                            buf.key_count += 1;
                        }
                        Key::Tab => {
                            buf.content.push_str("[Tab]");
                            buf.key_count += 1;
                        }
                        Key::Backspace => {
                            buf.content.push_str("[Backspace]");
                            buf.key_count += 1;
                        }
                        Key::Delete => {
                            buf.content.push_str("[Delete]");
                            buf.key_count += 1;
                        }
                        Key::Escape => {
                            buf.content.push_str("[Esc]");
                            buf.key_count += 1;
                        }
                        Key::UpArrow => {
                            buf.content.push_str("[Up]");
                            buf.key_count += 1;
                        }
                        Key::DownArrow => {
                            buf.content.push_str("[Down]");
                            buf.key_count += 1;
                        }
                        Key::LeftArrow => {
                            buf.content.push_str("[Left]");
                            buf.key_count += 1;
                        }
                        Key::RightArrow => {
                            buf.content.push_str("[Right]");
                            buf.key_count += 1;
                        }
                        _ => {
                            let shift = *shift_pressed.lock();
                            if let Some(c) = key_to_char(key, shift) {
                                buf.content.push(c);
                                buf.key_count += 1;
                            }
                        }
                    }

                    // Update buffer state
                    if buf.app_name.is_empty() {
                        buf.app_name = window_info.app_name;
                        buf.window_title = window_info.window_title;
                    }
                    buf.last_input_time = now;
                }
                EventType::KeyRelease(key) => {
                    if matches!(key, Key::ShiftLeft | Key::ShiftRight) {
                        *shift_clone.lock() = false;
                    }
                }
                _ => {}
            }
        };

        if let Err(e) = listen(callback) {
            eprintln!("Error listening to keyboard events: {:?}", e);
        }
    });
}
