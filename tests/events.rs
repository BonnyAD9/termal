use termal::{
    Rgb,
    raw::events::{
        AmbiguousEvent, AnyEvent, Event, Key, KeyCode, Modifiers, StateChange,
        Status, TermAttr, TermFeatures, TermType,
        mouse::{self, Mouse},
    },
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
        Key::verbatim('\x1b'),
        Key::new(KeyCode::Char('\x1b'), Modifiers::NONE, '\x1b')
    );

    assert_eq!(
        AmbiguousEvent::unknown("\x1b[2;2H"),
        AmbiguousEvent {
            event: AnyEvent::Unknown("\x1b[2;2H".into()),
            other: vec![],
        }
    );

    assert_eq!(
        AmbiguousEvent::event(Event::Focus),
        AmbiguousEvent {
            event: AnyEvent::Known(Event::Focus),
            other: vec![],
        }
    );

    assert_eq!(
        AmbiguousEvent::key(Key::code(KeyCode::Esc)),
        AmbiguousEvent::event(Event::KeyPress(Key::code(KeyCode::Esc)))
    );

    assert_eq!(
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Left,
            event: mouse::Event::Up,
            modifiers: Modifiers::ALT,
            x: 5,
            y: 7
        }),
        AmbiguousEvent::event(Event::Mouse(Mouse {
            button: mouse::Button::Left,
            event: mouse::Event::Up,
            modifiers: Modifiers::ALT,
            x: 5,
            y: 7
        }))
    );

    assert_eq!(
        AmbiguousEvent::status(Status::Ok),
        AmbiguousEvent::event(Event::Status(Status::Ok)),
    );

    assert_eq!(
        AmbiguousEvent::verbatim('\x1b'),
        AmbiguousEvent::key(Key::verbatim('\x1b')),
    );

    assert_eq!(
        AmbiguousEvent::state_change(StateChange::BracketedPasteStart),
        AmbiguousEvent::event(Event::StateChange(
            StateChange::BracketedPasteStart
        )),
    );
}

#[test]
fn test_unknown() {
    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b;;"),
        AmbiguousEvent::unknown(b"\x1b;;")
    );
}

#[test]
fn test_key() {
    assert_eq!(
        AmbiguousEvent::from_char_code('K'),
        AmbiguousEvent::key(Key::new(
            KeyCode::Char('k'),
            Modifiers::SHIFT,
            'K'
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_char_code('\x03'),
        AmbiguousEvent::key(Key::mcode(
            KeyCode::Char('c'),
            Modifiers::CONTROL
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"K"),
        AmbiguousEvent::from_char_code('K'),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1bJ"),
        AmbiguousEvent::key(Key::mcode(
            KeyCode::Char('j'),
            Modifiers::ALT | Modifiers::SHIFT
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b"),
        AmbiguousEvent::key(Key::code(KeyCode::Esc)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b\x1b"),
        AmbiguousEvent::key(Key::mcode(KeyCode::Esc, Modifiers::ALT)),
    );

    assert_eq!(
        AmbiguousEvent::from_code("š".as_bytes()),
        AmbiguousEvent::key(Key::new(
            KeyCode::Char('š'),
            Modifiers::NONE,
            'š'
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b["),
        AmbiguousEvent::key(Key::mcode(KeyCode::Char('['), Modifiers::ALT)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[1;5H"),
        AmbiguousEvent::key(Key::mcode(KeyCode::Home, Modifiers::CONTROL))
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[1;13G"),
        AmbiguousEvent::key(Key::mcode(
            KeyCode::Char('5'),
            Modifiers::CONTROL | Modifiers::META
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[A"),
        AmbiguousEvent::key(Key::code(KeyCode::Up)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[18;4~"),
        AmbiguousEvent::key(Key::mcode(
            KeyCode::F7,
            Modifiers::SHIFT | Modifiers::ALT
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1bO1;2Q"),
        AmbiguousEvent::key(Key::mcode(KeyCode::F2, Modifiers::SHIFT)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[1R"),
        AmbiguousEvent::key(Key::code(KeyCode::F3)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[1;9P"),
        AmbiguousEvent::key(Key::mcode(KeyCode::F1, Modifiers::META)),
    );
}

#[test]
fn test_ambiguous() {
    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[1;2R"),
        AmbiguousEvent {
            event: AnyEvent::Known(Event::KeyPress(Key::mcode(
                KeyCode::F3,
                Modifiers::SHIFT
            ))),
            other: vec![Event::Status(Status::CursorPosition { x: 2, y: 1 })]
        },
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1bd"),
        AmbiguousEvent {
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
    // Normal mode

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[M\x20\x28\x2F"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Left,
            modifiers: Modifiers::NONE,
            event: mouse::Event::Down,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[M\x36\x28\x2F"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Right,
            modifiers: Modifiers::CONTROL | Modifiers::SHIFT,
            event: mouse::Event::Down,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[M\x71\x28\x2F"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Middle,
            modifiers: Modifiers::CONTROL,
            event: mouse::Event::ScrollDown,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[M\x45\x28\x2F"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Middle,
            modifiers: Modifiers::SHIFT,
            event: mouse::Event::Move,
            x: 8,
            y: 15,
        })
    );

    // UTF-8

    assert_eq!(
        AmbiguousEvent::from_code("\x1b[M\x47\u{5fc}\x2F".as_bytes()),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::None,
            modifiers: Modifiers::SHIFT,
            event: mouse::Event::Move,
            x: 1500,
            y: 15,
        })
    );

    // SGR

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[<0;8;15m"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Left,
            modifiers: Modifiers::NONE,
            event: mouse::Event::Up,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[<22;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Right,
            modifiers: Modifiers::CONTROL | Modifiers::SHIFT,
            event: mouse::Event::Down,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[<81;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Middle,
            modifiers: Modifiers::CONTROL,
            event: mouse::Event::ScrollDown,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[<37;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Middle,
            modifiers: Modifiers::SHIFT,
            event: mouse::Event::Move,
            x: 8,
            y: 15,
        })
    );

    // URXVT

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[32;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Left,
            modifiers: Modifiers::NONE,
            event: mouse::Event::Down,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[54;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Right,
            modifiers: Modifiers::CONTROL | Modifiers::SHIFT,
            event: mouse::Event::Down,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[113;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Middle,
            modifiers: Modifiers::CONTROL,
            event: mouse::Event::ScrollDown,
            x: 8,
            y: 15,
        })
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[69;8;15M"),
        AmbiguousEvent::mouse(Mouse {
            button: mouse::Button::Middle,
            modifiers: Modifiers::SHIFT,
            event: mouse::Event::Move,
            x: 8,
            y: 15,
        })
    );
}

#[test]
fn test_status() {
    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[?62;1;4;21;22c"),
        AmbiguousEvent::status(Status::Attributes(TermAttr {
            typ: TermType::Vt220,
            features: TermFeatures::COLUMNS132
                | TermFeatures::SIXEL_GRAPHICS
                | TermFeatures::HORIZONTAL_SCROLLING
                | TermFeatures::ANSI_COLOR,
        }))
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[0n"),
        AmbiguousEvent::status(Status::Ok),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[?10;17R"),
        AmbiguousEvent::status(Status::CursorPosition { x: 17, y: 10 }),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1bP>|My Terminal\x1b\\"),
        AmbiguousEvent::status(Status::TerminalName(
            "My Terminal".to_string()
        )),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[4;17;10t"),
        AmbiguousEvent::status(Status::TextAreaSizePx { w: 10, h: 17 }),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[8;17;10t"),
        AmbiguousEvent::status(Status::TextAreaSize { w: 10, h: 17 }),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[6;17;10t"),
        AmbiguousEvent::status(Status::CharSize { w: 10, h: 17 }),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[?1;0;256S"),
        AmbiguousEvent::status(Status::SixelColors(256)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[?1;0;256S"),
        AmbiguousEvent::status(Status::SixelColors(256)),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b]10;rgb:12/34/56\x1b\\"),
        AmbiguousEvent::status(Status::DefaultFgColor(Rgb::<u16>::new(
            0x1212, 0x3434, 0x5656
        ))),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b]11;rgb:12/34/56\x1b\\"),
        AmbiguousEvent::status(Status::DefaultBgColor(Rgb::<u16>::new(
            0x1212, 0x3434, 0x5656
        ))),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b]12;rgb:12/34/56\x1b\\"),
        AmbiguousEvent::status(Status::CursorColor(Rgb::<u16>::new(
            0x1212, 0x3434, 0x5656
        ))),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b]52;;aGVsbG8gdGhlcmU=\x1b\\"),
        AmbiguousEvent::status(Status::SelectionData(b"hello there".into())),
    );
}

#[test]
fn test_state_change() {
    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[200~"),
        AmbiguousEvent::state_change(StateChange::BracketedPasteStart),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[201~"),
        AmbiguousEvent::state_change(StateChange::BracketedPasteEnd),
    );
}

#[test]
fn test_other() {
    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[I"),
        AmbiguousEvent::event(Event::Focus),
    );

    assert_eq!(
        AmbiguousEvent::from_code(b"\x1b[O"),
        AmbiguousEvent::event(Event::FocusLost),
    );
}
