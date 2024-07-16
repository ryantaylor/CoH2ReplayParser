//! `vault` library integration tests.

extern crate vault;

use vault::Replay;

#[test]
fn parse_success() {
    let data = include_bytes!("../replays/USvDAK_v10612.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(unwrapped.version(), 10612);
    assert_eq!(
        unwrapped
            .players()
            .iter()
            .map(|player| { player.name() })
            .collect::<Vec<&str>>(),
        vec!["madhax", "Quixalotl"]
    );
}

#[test]
fn parse_failure() {
    let data = [1, 2, 3];
    let replay = Replay::from_bytes(&data);
    assert!(replay.is_err())
}

#[test]
fn parse_success_ai() {
    let data = include_bytes!("../replays/vs_ai.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(unwrapped.version(), 21283);
    assert_eq!(
        unwrapped
            .players()
            .iter()
            .map(|player| { player.name() })
            .collect::<Vec<&str>>(),
        vec!["Janne252", "CPU - Standard"]
    );
}

#[test]
fn parse_weird_description() {
    let data = include_bytes!("../replays/weird_description.rec");
    let replay = Replay::from_bytes(data);
    assert!(replay.is_ok());
    let unwrapped = replay.unwrap();
    assert_eq!(unwrapped.map().localized_name_id(), "Twin Beaches ML");
    assert_eq!(unwrapped.map().localized_description_id(), "TB ML");
}

#[test]
fn parse_battlegroup() {
    let data = include_bytes!("../replays/USvDAK_v10612.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(
        replay
            .players()
            .iter()
            .map(|player| { player.battlegroup() })
            .collect::<Vec<Option<u32>>>(),
        vec![Some(2072430), Some(196934)]
    );
}

#[test]
fn parse_new_map_chunk() {
    let data = include_bytes!("../replays/one_seven_zero.rec");
    let replay = Replay::from_bytes(data).unwrap();
    assert_eq!(
        replay.map_filename(),
        "data:scenarios\\multiplayer\\desert_airfield_6p_mkii\\desert_airfield_6p_mkii"
    );
    assert_eq!(replay.map_localized_name_id(), "$11233954");
    assert_eq!(replay.map_localized_description_id(), "$11233955");
}
