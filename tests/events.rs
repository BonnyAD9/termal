use termal::raw::events::{
    mouse::{self, Mouse},
    AmbigousEvent, AnyEvent, Event, Key, KeyCode, Modifiers, Status,
};

#[test]
fn test_constructors() {
    assert_eq!(
        Key::new(KeyCode::Esc, Modifiers::SHIFT | Modifiers::META, 'k'),
        Key {
            key_char: Some('k'),
            code: KeyCode::Esc,
            modifiers: Modifiers::SHIFT | Modifiers::META
        }
    );

    assert_eq!(
        Key::mcode(KeyCode::Backspace, Modifiers::ALT | Modifiers::CONTROL),
        Key {
            key_char: None,
            code: KeyCode::Backspace,
            modifiers: Modifiers::ALT | Modifiers::CONTROL
        }
    );

    assert_eq!(
        Key::code(KeyCode::Enter),
        Key::mcode(KeyCode::Enter, Modifiers::NONE)
    );

    assert_eq!(
        AmbigousEvent::unknown("\x1b[2;2H"),
        AmbigousEvent {
            event: AnyEvent::Unknown("\x1b[2;2H".into()),
            other: vec![],
        }
    );

    assert_eq!(
        AmbigousEvent::event(Event::Focus),
        AmbigousEvent {
            event: AnyEvent::Known(Event::Focus),
            other: vec![],
        }
    );

    assert_eq!(
        AmbigousEvent::key(Key::code(KeyCode::Esc)),
        AmbigousEvent::event(Event::KeyPress(Key::code(KeyCode::Esc)))
    );

    assert_eq!(
        AmbigousEvent::mouse(Mouse {
            button: mouse::Button::Left,
            event: mouse::Event::Up,
            modifiers: Modifiers::ALT,
            x: 5,
            y: 7
        }),
        AmbigousEvent::event(Event::Mouse(Mouse {
            button: mouse::Button::Left,
            event: mouse::Event::Up,
            modifiers: Modifiers::ALT,
            x: 5,
            y: 7
        }))
    );

    assert_eq!(
        AmbigousEvent::status(Status::Ok),
        AmbigousEvent::event(Event::Status(Status::Ok)),
    );
}

#[test]
fn test_unknown() {
    assert_eq!(
        AmbigousEvent::from_code(b"\x1b;;"),
        AmbigousEvent::unknown(b"\x1b;;")
    );
}

#[test]
fn test_key() {
    assert_eq!(
        AmbigousEvent::from_char_code('K'),
        AmbigousEvent::key(Key::new(
            KeyCode::Char('k'),
            Modifiers::SHIFT,
            'K'
        )),
    );

    assert_eq!(
        AmbigousEvent::from_char_code('\x03'),
        AmbigousEvent::key(Key::mcode(KeyCode::Char('c'), Modifiers::CONTROL)),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"K"),
        AmbigousEvent::from_char_code('K'),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1bJ"),
        AmbigousEvent::key(Key::mcode(
            KeyCode::Char('j'),
            Modifiers::ALT | Modifiers::SHIFT
        )),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b"),
        AmbigousEvent::key(Key::code(KeyCode::Esc)),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b\x1b"),
        AmbigousEvent::key(Key::mcode(KeyCode::Esc, Modifiers::ALT)),
    );

    assert_eq!(
        AmbigousEvent::from_code("š".as_bytes()),
        AmbigousEvent::key(Key::new(KeyCode::Char('š'), Modifiers::NONE, 'š')),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b["),
        AmbigousEvent::key(Key::mcode(KeyCode::Char('['), Modifiers::ALT)),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[1;5H"),
        AmbigousEvent::key(Key::mcode(KeyCode::Home, Modifiers::CONTROL))
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[1;13G"),
        AmbigousEvent::key(Key::mcode(
            KeyCode::Char('5'),
            Modifiers::CONTROL | Modifiers::META
        )),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[A"),
        AmbigousEvent::key(Key::code(KeyCode::Up)),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[18;4~"),
        AmbigousEvent::key(Key::mcode(
            KeyCode::F7,
            Modifiers::SHIFT | Modifiers::ALT
        )),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1bO1;2Q"),
        AmbigousEvent::key(Key::mcode(KeyCode::F2, Modifiers::SHIFT)),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[1R"),
        AmbigousEvent::key(Key::code(KeyCode::F3)),
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[1;9P"),
        AmbigousEvent::key(Key::mcode(KeyCode::F1, Modifiers::META)),
    );
}

#[test]
fn test_ambiguous() {
    assert_eq!(
        AmbigousEvent::from_code(b"\x1b[1;2R"),
        AmbigousEvent {
            event: AnyEvent::Known(Event::KeyPress(Key::mcode(
                KeyCode::F3,
                Modifiers::SHIFT
            ))),
            other: vec![Event::Status(Status::CursorPosition { x: 2, y: 1 })]
        },
    );

    assert_eq!(
        AmbigousEvent::from_code(b"\x1bd"),
        AmbigousEvent {
            event: AnyEvent::Known(Event::KeyPress(Key::mcode(
                KeyCode::Char('d'),
                Modifiers::ALT
            ))),
            other: vec![Event::KeyPress(Key::mcode(
                KeyCode::Delete,
                Modifiers::CONTROL
            ))]
        },
    );
}

#[test]
fn test_mouse() {
    // TODO
}
