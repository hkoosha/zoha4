use std::collections::HashSet;
use std::env;
use std::str::FromStr;
use std::sync::Mutex;

use dbus::{Error, Message};
use dbus::blocking::Connection;
use dbus::channel::Sender;
use device_query::{CallbackGuard, DeviceEvents, Keycode};
use eyre::eyre;

use crate::new_signal;

static mut LOCK: Mutex<bool> = Mutex::new(false);
static mut CONN: Option<Connection> = None;
static mut KEYCODES: Option<HashSet<Keycode>> = None;
static mut PUSHED: Option<HashSet<Keycode>> = None;

pub fn list_keycodes() -> Vec<&'static str> {
    return vec![
        "Key0",
        "Key1",
        "Key2",
        "Key3",
        "Key4",
        "Key5",
        "Key6",
        "Key7",
        "Key8",
        "Key9",
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
        "T",
        "U",
        "V",
        "W",
        "X",
        "Y",
        "Z",
        "F1",
        "F2",
        "F3",
        "F4",
        "F5",
        "F6",
        "F7",
        "F8",
        "F9",
        "F10",
        "F11",
        "F12",
        "Escape",
        "Space",
        "LControl",
        "RControl",
        "LShift",
        "RShift",
        "LAlt",
        "RAlt",
        "Meta",
        "Enter",
        "Up",
        "Down",
        "Left",
        "Right",
        "Backspace",
        "CapsLock",
        "Tab",
        "Home",
        "End",
        "PageUp",
        "PageDown",
        "Insert",
        "Delete",
        "Numpad0",
        "Numpad1",
        "Numpad2",
        "Numpad3",
        "Numpad4",
        "Numpad5",
        "Numpad6",
        "Numpad7",
        "Numpad8",
        "Numpad9",
        "NumpadSubtract",
        "NumpadAdd",
        "NumpadDivide",
        "NumpadMultiply",
        "Grave",
        "Minus",
        "Equal",
        "LeftBracket",
        "RightBracket",
        "BackSlash",
        "Semicolon",
        "Apostrophe",
        "Comma",
        "Dot",
        "Slash",
    ];
}

fn check_and_send() {
    unsafe {
        let _hold = LOCK.lock();

        if PUSHED.is_none() {
            PUSHED = Some(HashSet::new());
        }

        if PUSHED
            .as_ref()
            .unwrap()
            .symmetric_difference(KEYCODES.as_ref().expect("KEYCODES not set"))
            .collect::<Vec<_>>()
            .is_empty() {
            let signal: Message = new_signal();
            CONN.as_ref().unwrap().send(signal).expect("failed to send dbus signal");
        }
    }
}

fn init_keycode(keys: &Vec<String>) -> eyre::Result<()> {
    let mut keycode_already_set: bool = false;
    let mut conn_error: Option<Error> = None;

    unsafe {
        let _hold = LOCK.lock();

        if KEYCODES.is_some() {
            keycode_already_set = true;
        } else {
            let mut keycodes: HashSet<Keycode> = HashSet::new();

            for key in keys {
                match Keycode::from_str(key) {
                    Ok(kc) => keycodes.insert(kc),
                    Err(_) => return Err(eyre!("invalid keycode: {}", key)),
                };
            }

            KEYCODES = Some(keycodes);

            match Connection::new_session() {
                Ok(conn) => CONN = Some(conn),
                Err(err) => conn_error = Some(err),
            }
        }
    }

    if keycode_already_set {
        panic!("keycode already set");
    }

    if let Some(err) = conn_error {
        unsafe {
            KEYCODES = None;
        }

        return Err(eyre!("could not initiate dbus connection: {}", err));
    }

    return Ok(());
}

fn get_keycodes() -> Option<HashSet<Keycode>> {
    let keycodes: Option<HashSet<Keycode>> = unsafe {
        let _guard = LOCK.lock().unwrap();

        KEYCODES.clone()
    };

    return keycodes;
}

fn down(kc: Keycode) {
    unsafe {
        let _guard = LOCK.lock().unwrap();

        if PUSHED.is_none() {
            PUSHED = Some(HashSet::new());
        }

        PUSHED.as_mut().unwrap().insert(kc);
    }
}

fn up(kc: &Keycode) {
    unsafe {
        let _guard = LOCK.lock().unwrap();

        PUSHED.as_mut().unwrap().remove(kc);
    }
}

type GrabberGuard = (CallbackGuard<fn(&Keycode)>, CallbackGuard<fn(&Keycode)>);

pub fn enable_key_grabber_hack(
    quiet: bool,
    keys: &Vec<String>,
) -> eyre::Result<GrabberGuard> {
    init_keycode(keys)?;

    if !quiet {
        eprintln!(
            "================================================================================\n\
            Enabling key listener. Please note this is a hack and does not work well, as it will \
            not prevent the toggle key from reaching current active window, and this can interfere \
            with your work on that window. Also, the keycode for toggle shortcut may also be \
            carried into the virtual terminal inside Zoha.\n\
            The best option is to configure a keyboard shortcut through your window manager bound \
            to the toggle visibility command on Zoha: {} -s\n\
            ================================================================================",
            env::args().next().unwrap_or_else(|| "zoha".to_string()),
        );
        eprintln!(
            "Toggling key codes: {}",
            get_keycodes()
                .unwrap()
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<_>>()
                .join("+"),
        );
    }

    let state = device_query::DeviceState::new();

    let key_down: CallbackGuard<fn(&Keycode)> = state.on_key_down(move |kc| {
        down(*kc);
        check_and_send();
    });

    let key_up: CallbackGuard<fn(&Keycode)> = state.on_key_up(up);

    return Ok((key_down, key_up));
}
