use termal::{codes, formatc, formatmc, gradient};

#[test]
fn test_gradient() {
    let g = gradient("BonnyAD9", (250, 50, 170), (180, 50, 240));
    let v = "\x1b[38;2;250;50;170mB\x1b[38;2;240;50;180mo\
        \x1b[38;2;230;50;190mn\x1b[38;2;220;50;200mn\x1b[38;2;210;50;210my\
        \x1b[38;2;200;50;220mA\x1b[38;2;190;50;230mD\x1b[38;2;180;50;240m9";

    assert_eq!(g, v);
}

#[test]
fn test_formatc() {
    let s = "Hello";
    let num = 4;
    let num2 = 0.5;
    assert_eq!(formatc!("{'y}{s} {num}{'_}{}", num2), "\x1b[93mHello 4\x1b[0m0.5");
    assert_eq!(formatmc!(false, "{'y}{s} {num}{'_}{}", num2), "Hello 40.5");
}

#[test]
fn test_formatc_codes() {
    // True RGB
    assert_eq!(formatc!("{'#123456}"), codes::fg!(0x12, 0x34, 0x56));
    assert_eq!(formatc!("{'#123}"), formatc!("{'#112233}"));
    assert_eq!(formatc!("{'#12}"), formatc!("{'#121212}"));
    assert_eq!(formatc!("{'#1}"), formatc!("{'#111111}"));

    assert_eq!(formatc!("{'#123456_}"), codes::bg!(0x12, 0x34, 0x56));
    assert_eq!(formatc!("{'#123_}"), formatc!("{'#112233_}"));
    assert_eq!(formatc!("{'#12_}"), formatc!("{'#121212_}"));
    assert_eq!(formatc!("{'#1_}"), formatc!("{'#111111_}"));

    // Ascii
    assert_eq!(formatc!("{'bell}"), codes::BELL.to_string());
    assert_eq!(formatc!("{'backspace}"), codes::BACKSPACE.to_string());
    assert_eq!(formatc!("{'tab}"), "\t");
    assert_eq!(formatc!("{'htab}"), "\t");
    assert_eq!(formatc!("{'move_down_scrl}"), "\n");
    assert_eq!(formatc!("{'mds}"), "\n");
    assert_eq!(formatc!("{'newline}"), "\n\r");
    assert_eq!(formatc!("{'nl}"), "\n\r");
    assert_eq!(formatc!("{'vtab}"), codes::VTAB.to_string());
    assert_eq!(formatc!("{'carriage_return}"), "\r");
    assert_eq!(formatc!("{'cr}"), "\r");

    // Moving cursor
    assert_eq!(formatc!("{'move_to5,4}"), codes::move_to!(5, 4));
    assert_eq!(formatc!("{'mt8,1}"), codes::move_to!(8, 1));
    assert_eq!(formatc!("{'move_up5}"), codes::move_up!(5));
    assert_eq!(formatc!("{'mu5}"), codes::move_up!(5));
    assert_eq!(formatc!("{'move_down5}"), codes::move_down!(5));
    assert_eq!(formatc!("{'md5}"), codes::move_down!(5));
    assert_eq!(formatc!("{'move_right5}"), codes::move_right!(5));
    assert_eq!(formatc!("{'mr5}"), codes::move_right!(5));
    assert_eq!(formatc!("{'move_left5}"), codes::move_left!(5));
    assert_eq!(formatc!("{'ml5}"), codes::move_left!(5));
    assert_eq!(formatc!("{'set_down5}"), codes::set_down!(5));
    assert_eq!(formatc!("{'sd5}"), codes::set_down!(5));
    assert_eq!(formatc!("{'set_up5}"), codes::set_up!(5));
    assert_eq!(formatc!("{'su5}"), codes::set_up!(5));
    assert_eq!(formatc!("{'move_to_column5}"), codes::column!(5));
    assert_eq!(formatc!("{'mc5}"), codes::column!(5));

    assert_eq!(formatc!("{'move_up_scrl}"), codes::UP_SCRL);
    assert_eq!(formatc!("{'save_cur}"), codes::CUR_SAVE);
    assert_eq!(formatc!("{'save}"), codes::CUR_SAVE);
    assert_eq!(formatc!("{'s}"), codes::CUR_SAVE);
    assert_eq!(formatc!("{'load_cur}"), codes::CUR_LOAD);
    assert_eq!(formatc!("{'load}"), codes::CUR_LOAD);
    assert_eq!(formatc!("{'l}"), codes::CUR_LOAD);

    // TODO
}
