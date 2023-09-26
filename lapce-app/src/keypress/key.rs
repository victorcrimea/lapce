use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
};

use floem::keyboard::{Key, KeyCode};

#[derive(Clone, Debug, Eq)]
pub(crate) enum KeyInput {
    Keyboard(floem::keyboard::Key, floem::keyboard::KeyCode),
    Pointer(floem::pointer::PointerButton),
}

impl KeyInput {
    fn keyboard_from_str(s: &str) -> Option<(Key, KeyCode)> {
        // Checks if it is a character key
        fn is_key_string(s: &str) -> bool {
            s.chars().all(|c| !c.is_control())
                && s.chars().skip(1).all(|c| !c.is_ascii())
        }

        fn char_to_keycode(char: &str) -> KeyCode {
            match char {
                "a" => KeyCode::KeyA,
                "b" => KeyCode::KeyB,
                "c" => KeyCode::KeyC,
                "d" => KeyCode::KeyD,
                "e" => KeyCode::KeyE,
                "f" => KeyCode::KeyF,
                "g" => KeyCode::KeyG,
                "h" => KeyCode::KeyH,
                "i" => KeyCode::KeyI,
                "j" => KeyCode::KeyJ,
                "k" => KeyCode::KeyK,
                "l" => KeyCode::KeyL,
                "m" => KeyCode::KeyM,
                "n" => KeyCode::KeyN,
                "o" => KeyCode::KeyO,
                "p" => KeyCode::KeyP,
                "q" => KeyCode::KeyQ,
                "r" => KeyCode::KeyR,
                "s" => KeyCode::KeyS,
                "t" => KeyCode::KeyT,
                "u" => KeyCode::KeyU,
                "v" => KeyCode::KeyV,
                "w" => KeyCode::KeyW,
                "x" => KeyCode::KeyX,
                "y" => KeyCode::KeyY,
                "z" => KeyCode::KeyZ,
                "=" => KeyCode::Equal,
                "-" => KeyCode::Minus,
                "0" => KeyCode::Digit0,
                "1" => KeyCode::Digit1,
                "2" => KeyCode::Digit2,
                "3" => KeyCode::Digit3,
                "4" => KeyCode::Digit4,
                "5" => KeyCode::Digit5,
                "6" => KeyCode::Digit6,
                "7" => KeyCode::Digit7,
                "8" => KeyCode::Digit8,
                "9" => KeyCode::Digit9,
                "`" => KeyCode::Backquote,
                "/" => KeyCode::Slash,
                "\\" => KeyCode::Backslash,
                "," => KeyCode::Comma,
                "." => KeyCode::Period,
                "*" => KeyCode::NumpadMultiply,
                "+" => KeyCode::NumpadAdd,
                ";" => KeyCode::Semicolon,
                "'" => KeyCode::Quote,
                "[" => KeyCode::BracketLeft,
                "]" => KeyCode::BracketRight,
                "<" => KeyCode::IntlBackslash,
                _ => KeyCode::Fn,
            }
        }

        // Import into scope to reduce noise
        use floem::keyboard::Key::*;
        Some(match s {
            s if is_key_string(s) => {
                let char = Character(s.into());
                (char.clone(), char_to_keycode(char.to_text().unwrap()))
            }
            "alt" => (Alt, KeyCode::AltLeft),
            "altgraph" => (AltGraph, KeyCode::AltRight),
            "capslock" => (CapsLock, KeyCode::CapsLock),
            "control" => (Control, KeyCode::ControlLeft),
            "fn" => (Fn, KeyCode::Fn),
            "fnlock" => (FnLock, KeyCode::FnLock),
            "meta" => (Meta, KeyCode::Meta),
            "numlock" => (NumLock, KeyCode::NumLock),
            "scrolllock" => (ScrollLock, KeyCode::ScrollLock),
            "shift" => (Shift, KeyCode::ShiftLeft),
            "hyper" => (Hyper, KeyCode::Hyper),
            "super" => (Super, KeyCode::Meta),
            "enter" => (Enter, KeyCode::Enter),
            "tab" => (Tab, KeyCode::Tab),
            "arrowdown" => (ArrowDown, KeyCode::ArrowDown),
            "arrowleft" => (ArrowLeft, KeyCode::ArrowLeft),
            "arrowright" => (ArrowRight, KeyCode::ArrowRight),
            "arrowup" => (ArrowUp, KeyCode::ArrowUp),
            "end" => (End, KeyCode::End),
            "home" => (Home, KeyCode::Home),
            "pagedown" => (PageDown, KeyCode::PageDown),
            "pageup" => (PageUp, KeyCode::PageUp),
            "backspace" => (Backspace, KeyCode::Backspace),
            "copy" => (Copy, KeyCode::Copy),
            "cut" => (Cut, KeyCode::Cut),
            "delete" => (Delete, KeyCode::Delete),
            "insert" => (Insert, KeyCode::Insert),
            "paste" => (Paste, KeyCode::Paste),
            "undo" => (Undo, KeyCode::Undo),
            "again" => (Again, KeyCode::Again),
            "contextmenu" => (ContextMenu, KeyCode::ContextMenu),
            "escape" => (Escape, KeyCode::Escape),
            "find" => (Find, KeyCode::Find),
            "help" => (Help, KeyCode::Help),
            "pause" => (Pause, KeyCode::Pause),
            "play" => (Play, KeyCode::MediaPlayPause),
            "props" => (Props, KeyCode::Props),
            "select" => (Select, KeyCode::Select),
            "eject" => (Eject, KeyCode::Eject),
            "power" => (Power, KeyCode::Power),
            "printscreen" => (PrintScreen, KeyCode::PrintScreen),
            "wakeup" => (WakeUp, KeyCode::WakeUp),
            "nonconvert" => (NonConvert, KeyCode::NonConvert),
            "hiragana" => (Hiragana, KeyCode::Hiragana),
            "kanamode" => (KanaMode, KeyCode::KanaMode),
            "katakana" => (Katakana, KeyCode::Katakana),
            "f1" => (F1, KeyCode::F1),
            "f2" => (F2, KeyCode::F2),
            "f3" => (F3, KeyCode::F3),
            "f4" => (F4, KeyCode::F4),
            "f5" => (F5, KeyCode::F5),
            "f6" => (F6, KeyCode::F6),
            "f7" => (F7, KeyCode::F7),
            "f8" => (F8, KeyCode::F8),
            "f9" => (F9, KeyCode::F9),
            "f10" => (F10, KeyCode::F10),
            "f11" => (F11, KeyCode::F11),
            "f12" => (F12, KeyCode::F12),
            "f13" => (F13, KeyCode::F13),
            "f14" => (F14, KeyCode::F14),
            "f15" => (F15, KeyCode::F15),
            "f16" => (F16, KeyCode::F16),
            "f17" => (F17, KeyCode::F17),
            "f18" => (F18, KeyCode::F18),
            "f19" => (F19, KeyCode::F19),
            "f20" => (F20, KeyCode::F20),
            "f21" => (F21, KeyCode::F21),
            "f22" => (F22, KeyCode::F22),
            "f23" => (F23, KeyCode::F23),
            "f24" => (F24, KeyCode::F24),
            "f25" => (F25, KeyCode::F25),
            "f26" => (F26, KeyCode::F26),
            "f27" => (F27, KeyCode::F27),
            "f28" => (F28, KeyCode::F28),
            "f29" => (F29, KeyCode::F29),
            "f30" => (F30, KeyCode::F30),
            "mediaplaypause" => (MediaPlayPause, KeyCode::MediaPlayPause),
            "mediastop" => (MediaStop, KeyCode::MediaStop),
            "mediatracknext" => (MediaTrackNext, KeyCode::MediaTrackNext),
            "mediatrackprevious" => {
                (MediaTrackPrevious, KeyCode::MediaTrackPrevious)
            }
            "open" => (Open, KeyCode::Open),
            "audiovolumedown" => (AudioVolumeDown, KeyCode::AudioVolumeDown),
            "audiovolumeup" => (AudioVolumeUp, KeyCode::AudioVolumeUp),
            "audiovolumemute" => (AudioVolumeMute, KeyCode::AudioVolumeMute),
            "launchmail" => (LaunchMail, KeyCode::LaunchMail),
            "browserback" => (BrowserBack, KeyCode::BrowserBack),
            "browserfavorites" => (BrowserFavorites, KeyCode::BrowserFavorites),
            "browserforward" => (BrowserForward, KeyCode::BrowserForward),
            "browserhome" => (BrowserHome, KeyCode::BrowserHome),
            "browserrefresh" => (BrowserRefresh, KeyCode::BrowserRefresh),
            "browsersearch" => (BrowserSearch, KeyCode::BrowserSearch),
            "browserstop" => (BrowserStop, KeyCode::BrowserStop),

            // Custom key name mappings
            "esc" => (Escape, KeyCode::Escape),
            "space" => (Character(" ".into()), KeyCode::Space),
            "bs" => (Backspace, KeyCode::Backspace),
            "up" => (ArrowUp, KeyCode::ArrowUp),
            "down" => (ArrowDown, KeyCode::ArrowDown),
            "right" => (ArrowRight, KeyCode::ArrowRight),
            "left" => (ArrowLeft, KeyCode::ArrowLeft),
            "del" => (Delete, KeyCode::Delete),

            _ => return None,
        })
    }

    fn mouse_from_str(s: &str) -> Option<floem::pointer::PointerButton> {
        use floem::pointer::PointerButton as B;

        Some(match s {
            "mousemiddle" => B::Auxiliary,
            "mouseforward" => B::X2,
            "mousebackward" => B::X1,
            _ => return None,
        })
    }
}

impl Display for KeyInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use floem::pointer::PointerButton as B;

        match self {
            Self::Keyboard(key, _key_code) => {
                use floem::keyboard::Key::*;
                match key {
                    Character(ref s) => write!(f, "{}", s),
                    Unidentified(_) => f.write_str("Unidentified"),
                    Alt => f.write_str("Alt"),
                    AltGraph => f.write_str("AltGraph"),
                    CapsLock => f.write_str("CapsLock"),
                    Control => f.write_str("Control"),
                    Fn => f.write_str("Fn"),
                    FnLock => f.write_str("FnLock"),
                    Meta => match std::env::consts::OS {
                        "macos" => f.write_str("Cmd"),
                        "windows" => f.write_str("Win"),
                        _ => f.write_str("Meta"),
                    },
                    NumLock => f.write_str("NumLock"),
                    ScrollLock => f.write_str("ScrollLock"),
                    Shift => f.write_str("Shift"),
                    Symbol => f.write_str("Symbol"),
                    SymbolLock => f.write_str("SymbolLock"),
                    Hyper => f.write_str("Hyper"),
                    Super => f.write_str("Super"),
                    Enter => f.write_str("Enter"),
                    Tab => f.write_str("Tab"),
                    ArrowDown => f.write_str("ArrowDown"),
                    ArrowLeft => f.write_str("ArrowLeft"),
                    ArrowRight => f.write_str("ArrowRight"),
                    ArrowUp => f.write_str("ArrowUp"),
                    End => f.write_str("End"),
                    Home => f.write_str("Home"),
                    PageDown => f.write_str("PageDown"),
                    PageUp => f.write_str("PageUp"),
                    Backspace => f.write_str("Backspace"),
                    Clear => f.write_str("Clear"),
                    Copy => f.write_str("Copy"),
                    CrSel => f.write_str("CrSel"),
                    Cut => f.write_str("Cut"),
                    Delete => f.write_str("Delete"),
                    EraseEof => f.write_str("EraseEof"),
                    ExSel => f.write_str("ExSel"),
                    Insert => f.write_str("Insert"),
                    Paste => f.write_str("Paste"),
                    Redo => f.write_str("Redo"),
                    Undo => f.write_str("Undo"),
                    Accept => f.write_str("Accept"),
                    Again => f.write_str("Again"),
                    Attn => f.write_str("Attn"),
                    Cancel => f.write_str("Cancel"),
                    ContextMenu => f.write_str("ContextMenu"),
                    Escape => f.write_str("Escape"),
                    Execute => f.write_str("Execute"),
                    Find => f.write_str("Find"),
                    Help => f.write_str("Help"),
                    Pause => f.write_str("Pause"),
                    Play => f.write_str("Play"),
                    Props => f.write_str("Props"),
                    Select => f.write_str("Select"),
                    ZoomIn => f.write_str("ZoomIn"),
                    ZoomOut => f.write_str("ZoomOut"),
                    BrightnessDown => f.write_str("BrightnessDown"),
                    BrightnessUp => f.write_str("BrightnessUp"),
                    Eject => f.write_str("Eject"),
                    LogOff => f.write_str("LogOff"),
                    Power => f.write_str("Power"),
                    PowerOff => f.write_str("PowerOff"),
                    PrintScreen => f.write_str("PrintScreen"),
                    Hibernate => f.write_str("Hibernate"),
                    Standby => f.write_str("Standby"),
                    WakeUp => f.write_str("WakeUp"),
                    AllCandidates => f.write_str("AllCandidates"),
                    Alphanumeric => f.write_str("Alphanumeric"),
                    CodeInput => f.write_str("CodeInput"),
                    Compose => f.write_str("Compose"),
                    Convert => f.write_str("Convert"),
                    Dead(_) => f.write_str("Dead"),
                    FinalMode => f.write_str("FinalMode"),
                    GroupFirst => f.write_str("GroupFirst"),
                    GroupLast => f.write_str("GroupLast"),
                    GroupNext => f.write_str("GroupNext"),
                    GroupPrevious => f.write_str("GroupPrevious"),
                    ModeChange => f.write_str("ModeChange"),
                    NextCandidate => f.write_str("NextCandidate"),
                    NonConvert => f.write_str("NonConvert"),
                    PreviousCandidate => f.write_str("PreviousCandidate"),
                    Process => f.write_str("Process"),
                    SingleCandidate => f.write_str("SingleCandidate"),
                    HangulMode => f.write_str("HangulMode"),
                    HanjaMode => f.write_str("HanjaMode"),
                    JunjaMode => f.write_str("JunjaMode"),
                    Eisu => f.write_str("Eisu"),
                    Hankaku => f.write_str("Hankaku"),
                    Hiragana => f.write_str("Hiragana"),
                    HiraganaKatakana => f.write_str("HiraganaKatakana"),
                    KanaMode => f.write_str("KanaMode"),
                    KanjiMode => f.write_str("KanjiMode"),
                    Katakana => f.write_str("Katakana"),
                    Romaji => f.write_str("Romaji"),
                    Zenkaku => f.write_str("Zenkaku"),
                    ZenkakuHankaku => f.write_str("ZenkakuHankaku"),
                    Soft1 => f.write_str("Soft1"),
                    Soft2 => f.write_str("Soft2"),
                    Soft3 => f.write_str("Soft3"),
                    Soft4 => f.write_str("Soft4"),
                    ChannelDown => f.write_str("ChannelDown"),
                    ChannelUp => f.write_str("ChannelUp"),
                    Close => f.write_str("Close"),
                    MailForward => f.write_str("MailForward"),
                    MailReply => f.write_str("MailReply"),
                    MailSend => f.write_str("MailSend"),
                    MediaClose => f.write_str("MediaClose"),
                    MediaFastForward => f.write_str("MediaFastForward"),
                    MediaPause => f.write_str("MediaPause"),
                    MediaPlay => f.write_str("MediaPlay"),
                    MediaPlayPause => f.write_str("MediaPlayPause"),
                    MediaRecord => f.write_str("MediaRecord"),
                    MediaRewind => f.write_str("MediaRewind"),
                    MediaStop => f.write_str("MediaStop"),
                    MediaTrackNext => f.write_str("MediaTrackNext"),
                    MediaTrackPrevious => f.write_str("MediaTrackPrevious"),
                    New => f.write_str("New"),
                    Open => f.write_str("Open"),
                    Print => f.write_str("Print"),
                    Save => f.write_str("Save"),
                    SpellCheck => f.write_str("SpellCheck"),
                    Key11 => f.write_str("Key11"),
                    Key12 => f.write_str("Key12"),
                    AudioBalanceLeft => f.write_str("AudioBalanceLeft"),
                    AudioBalanceRight => f.write_str("AudioBalanceRight"),
                    AudioBassBoostDown => f.write_str("AudioBassBoostDown"),
                    AudioBassBoostToggle => f.write_str("AudioBassBoostToggle"),
                    AudioBassBoostUp => f.write_str("AudioBassBoostUp"),
                    AudioFaderFront => f.write_str("AudioFaderFront"),
                    AudioFaderRear => f.write_str("AudioFaderRear"),
                    AudioSurroundModeNext => f.write_str("AudioSurroundModeNext"),
                    AudioTrebleDown => f.write_str("AudioTrebleDown"),
                    AudioTrebleUp => f.write_str("AudioTrebleUp"),
                    AudioVolumeDown => f.write_str("AudioVolumeDown"),
                    AudioVolumeUp => f.write_str("AudioVolumeUp"),
                    AudioVolumeMute => f.write_str("AudioVolumeMute"),
                    MicrophoneToggle => f.write_str("MicrophoneToggle"),
                    MicrophoneVolumeDown => f.write_str("MicrophoneVolumeDown"),
                    MicrophoneVolumeUp => f.write_str("MicrophoneVolumeUp"),
                    MicrophoneVolumeMute => f.write_str("MicrophoneVolumeMute"),
                    SpeechCorrectionList => f.write_str("SpeechCorrectionList"),
                    SpeechInputToggle => f.write_str("SpeechInputToggle"),
                    LaunchApplication1 => f.write_str("LaunchApplication1"),
                    LaunchApplication2 => f.write_str("LaunchApplication2"),
                    LaunchCalendar => f.write_str("LaunchCalendar"),
                    LaunchContacts => f.write_str("LaunchContacts"),
                    LaunchMail => f.write_str("LaunchMail"),
                    LaunchMediaPlayer => f.write_str("LaunchMediaPlayer"),
                    LaunchMusicPlayer => f.write_str("LaunchMusicPlayer"),
                    LaunchPhone => f.write_str("LaunchPhone"),
                    LaunchScreenSaver => f.write_str("LaunchScreenSaver"),
                    LaunchSpreadsheet => f.write_str("LaunchSpreadsheet"),
                    LaunchWebBrowser => f.write_str("LaunchWebBrowser"),
                    LaunchWebCam => f.write_str("LaunchWebCam"),
                    LaunchWordProcessor => f.write_str("LaunchWordProcessor"),
                    BrowserBack => f.write_str("BrowserBack"),
                    BrowserFavorites => f.write_str("BrowserFavorites"),
                    BrowserForward => f.write_str("BrowserForward"),
                    BrowserHome => f.write_str("BrowserHome"),
                    BrowserRefresh => f.write_str("BrowserRefresh"),
                    BrowserSearch => f.write_str("BrowserSearch"),
                    BrowserStop => f.write_str("BrowserStop"),
                    AppSwitch => f.write_str("AppSwitch"),
                    Call => f.write_str("Call"),
                    Camera => f.write_str("Camera"),
                    CameraFocus => f.write_str("CameraFocus"),
                    EndCall => f.write_str("EndCall"),
                    GoBack => f.write_str("GoBack"),
                    GoHome => f.write_str("GoHome"),
                    HeadsetHook => f.write_str("HeadsetHook"),
                    LastNumberRedial => f.write_str("LastNumberRedial"),
                    Notification => f.write_str("Notification"),
                    MannerMode => f.write_str("MannerMode"),
                    VoiceDial => f.write_str("VoiceDial"),
                    TV => f.write_str("TV"),
                    TV3DMode => f.write_str("TV3DMode"),
                    TVAntennaCable => f.write_str("TVAntennaCable"),
                    TVAudioDescription => f.write_str("TVAudioDescription"),
                    TVAudioDescriptionMixDown => {
                        f.write_str("TVAudioDescriptionMixDown")
                    }
                    TVAudioDescriptionMixUp => {
                        f.write_str("TVAudioDescriptionMixUp")
                    }
                    TVContentsMenu => f.write_str("TVContentsMenu"),
                    TVDataService => f.write_str("TVDataService"),
                    TVInput => f.write_str("TVInput"),
                    TVInputComponent1 => f.write_str("TVInputComponent1"),
                    TVInputComponent2 => f.write_str("TVInputComponent2"),
                    TVInputComposite1 => f.write_str("TVInputComposite1"),
                    TVInputComposite2 => f.write_str("TVInputComposite2"),
                    TVInputHDMI1 => f.write_str("TVInputHDMI1"),
                    TVInputHDMI2 => f.write_str("TVInputHDMI2"),
                    TVInputHDMI3 => f.write_str("TVInputHDMI3"),
                    TVInputHDMI4 => f.write_str("TVInputHDMI4"),
                    TVInputVGA1 => f.write_str("TVInputVGA1"),
                    TVMediaContext => f.write_str("TVMediaContext"),
                    TVNetwork => f.write_str("TVNetwork"),
                    TVNumberEntry => f.write_str("TVNumberEntry"),
                    TVPower => f.write_str("TVPower"),
                    TVRadioService => f.write_str("TVRadioService"),
                    TVSatellite => f.write_str("TVSatellite"),
                    TVSatelliteBS => f.write_str("TVSatelliteBS"),
                    TVSatelliteCS => f.write_str("TVSatelliteCS"),
                    TVSatelliteToggle => f.write_str("TVSatelliteToggle"),
                    TVTerrestrialAnalog => f.write_str("TVTerrestrialAnalog"),
                    TVTerrestrialDigital => f.write_str("TVTerrestrialDigital"),
                    TVTimer => f.write_str("TVTimer"),
                    AVRInput => f.write_str("AVRInput"),
                    AVRPower => f.write_str("AVRPower"),
                    ColorF0Red => f.write_str("ColorF0Red"),
                    ColorF1Green => f.write_str("ColorF1Green"),
                    ColorF2Yellow => f.write_str("ColorF2Yellow"),
                    ColorF3Blue => f.write_str("ColorF3Blue"),
                    ColorF4Grey => f.write_str("ColorF4Grey"),
                    ColorF5Brown => f.write_str("ColorF5Brown"),
                    ClosedCaptionToggle => f.write_str("ClosedCaptionToggle"),
                    Dimmer => f.write_str("Dimmer"),
                    DisplaySwap => f.write_str("DisplaySwap"),
                    DVR => f.write_str("DVR"),
                    Exit => f.write_str("Exit"),
                    FavoriteClear0 => f.write_str("FavoriteClear0"),
                    FavoriteClear1 => f.write_str("FavoriteClear1"),
                    FavoriteClear2 => f.write_str("FavoriteClear2"),
                    FavoriteClear3 => f.write_str("FavoriteClear3"),
                    FavoriteRecall0 => f.write_str("FavoriteRecall0"),
                    FavoriteRecall1 => f.write_str("FavoriteRecall1"),
                    FavoriteRecall2 => f.write_str("FavoriteRecall2"),
                    FavoriteRecall3 => f.write_str("FavoriteRecall3"),
                    FavoriteStore0 => f.write_str("FavoriteStore0"),
                    FavoriteStore1 => f.write_str("FavoriteStore1"),
                    FavoriteStore2 => f.write_str("FavoriteStore2"),
                    FavoriteStore3 => f.write_str("FavoriteStore3"),
                    Guide => f.write_str("Guide"),
                    GuideNextDay => f.write_str("GuideNextDay"),
                    GuidePreviousDay => f.write_str("GuidePreviousDay"),
                    Info => f.write_str("Info"),
                    InstantReplay => f.write_str("InstantReplay"),
                    Link => f.write_str("Link"),
                    ListProgram => f.write_str("ListProgram"),
                    LiveContent => f.write_str("LiveContent"),
                    Lock => f.write_str("Lock"),
                    MediaApps => f.write_str("MediaApps"),
                    MediaAudioTrack => f.write_str("MediaAudioTrack"),
                    MediaLast => f.write_str("MediaLast"),
                    MediaSkipBackward => f.write_str("MediaSkipBackward"),
                    MediaSkipForward => f.write_str("MediaSkipForward"),
                    MediaStepBackward => f.write_str("MediaStepBackward"),
                    MediaStepForward => f.write_str("MediaStepForward"),
                    MediaTopMenu => f.write_str("MediaTopMenu"),
                    NavigateIn => f.write_str("NavigateIn"),
                    NavigateNext => f.write_str("NavigateNext"),
                    NavigateOut => f.write_str("NavigateOut"),
                    NavigatePrevious => f.write_str("NavigatePrevious"),
                    NextFavoriteChannel => f.write_str("NextFavoriteChannel"),
                    NextUserProfile => f.write_str("NextUserProfile"),
                    OnDemand => f.write_str("OnDemand"),
                    Pairing => f.write_str("Pairing"),
                    PinPDown => f.write_str("PinPDown"),
                    PinPMove => f.write_str("PinPMove"),
                    PinPToggle => f.write_str("PinPToggle"),
                    PinPUp => f.write_str("PinPUp"),
                    PlaySpeedDown => f.write_str("PlaySpeedDown"),
                    PlaySpeedReset => f.write_str("PlaySpeedReset"),
                    PlaySpeedUp => f.write_str("PlaySpeedUp"),
                    RandomToggle => f.write_str("RandomToggle"),
                    RcLowBattery => f.write_str("RcLowBattery"),
                    RecordSpeedNext => f.write_str("RecordSpeedNext"),
                    RfBypass => f.write_str("RfBypass"),
                    ScanChannelsToggle => f.write_str("ScanChannelsToggle"),
                    ScreenModeNext => f.write_str("ScreenModeNext"),
                    Settings => f.write_str("Settings"),
                    SplitScreenToggle => f.write_str("SplitScreenToggle"),
                    STBInput => f.write_str("STBInput"),
                    STBPower => f.write_str("STBPower"),
                    Subtitle => f.write_str("Subtitle"),
                    Teletext => f.write_str("Teletext"),
                    VideoModeNext => f.write_str("VideoModeNext"),
                    Wink => f.write_str("Wink"),
                    ZoomToggle => f.write_str("ZoomToggle"),
                    F1 => f.write_str("F1"),
                    F2 => f.write_str("F2"),
                    F3 => f.write_str("F3"),
                    F4 => f.write_str("F4"),
                    F5 => f.write_str("F5"),
                    F6 => f.write_str("F6"),
                    F7 => f.write_str("F7"),
                    F8 => f.write_str("F8"),
                    F9 => f.write_str("F9"),
                    F10 => f.write_str("F10"),
                    F11 => f.write_str("F11"),
                    F12 => f.write_str("F12"),
                    F13 => f.write_str("F13"),
                    F14 => f.write_str("F14"),
                    F15 => f.write_str("F15"),
                    F16 => f.write_str("F16"),
                    F17 => f.write_str("F17"),
                    F18 => f.write_str("F18"),
                    F19 => f.write_str("F19"),
                    F20 => f.write_str("F20"),
                    F21 => f.write_str("F21"),
                    F22 => f.write_str("F22"),
                    F23 => f.write_str("F23"),
                    F24 => f.write_str("F24"),
                    F25 => f.write_str("F25"),
                    F26 => f.write_str("F26"),
                    F27 => f.write_str("F27"),
                    F28 => f.write_str("F28"),
                    F29 => f.write_str("F29"),
                    F30 => f.write_str("F30"),
                    F31 => f.write_str("F31"),
                    F32 => f.write_str("F32"),
                    F33 => f.write_str("F33"),
                    F34 => f.write_str("F34"),
                    F35 => f.write_str("F35"),
                    _ => f.write_str("Unidentified"),
                }
            }
            Self::Pointer(B::Auxiliary) => f.write_str("MouseMiddle"),
            Self::Pointer(B::X2) => f.write_str("MouseForward"),
            Self::Pointer(B::X1) => f.write_str("MouseBackward"),
            Self::Pointer(_) => f.write_str("MouseUnimplemented"),
        }
    }
}

impl FromStr for KeyInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        KeyInput::keyboard_from_str(&s)
            .map(|key| KeyInput::Keyboard(key.0, key.1))
            .or_else(|| KeyInput::mouse_from_str(&s).map(KeyInput::Pointer))
            .ok_or(())
    }
}

impl Hash for KeyInput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Keyboard(_key, key_code) => key_code.hash(state),
            // TODO: Implement `Hash` for `druid::MouseButton`
            Self::Pointer(btn) => (*btn as u8).hash(state),
        }
    }
}

impl PartialEq for KeyInput {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                KeyInput::Keyboard(_key_a, key_code_a),
                KeyInput::Keyboard(_key_b, key_code_b),
            ) => key_code_a.eq(key_code_b),
            (KeyInput::Pointer(a), KeyInput::Pointer(b)) => a.eq(b),
            _ => false,
        }
    }
}
