use crate::modules::{
    armory::{
        domain_value::{CharacterGear, CharacterGuild, CharacterInfo, GuildRank},
        material::{Character, CharacterHistory, Guild},
        Armory,
    },
    tooltip::{tools::RetrieveGuildTooltip, Tooltip},
};

fn build_character_guild(guild_id: u32) -> CharacterGuild {
    CharacterGuild {
        guild_id: guild_id,
        rank: GuildRank { index: 0, name: "".to_string() },
    }
}

fn build_character_history(enable_character_guild: bool, guild_id: u32) -> CharacterHistory {
    CharacterHistory {
        id: 0,
        character_id: 0,
        character_info: CharacterInfo {
            id: 0,
            gear: CharacterGear {
                id: 0,
                head: None,
                neck: None,
                shoulder: None,
                back: None,
                chest: None,
                shirt: None,
                tabard: None,
                wrist: None,
                main_hand: None,
                off_hand: None,
                ternary_hand: None,
                glove: None,
                belt: None,
                leg: None,
                boot: None,
                ring1: None,
                ring2: None,
                trinket1: None,
                trinket2: None,
            },
            hero_class_id: 0,
            level: 0,
            gender: false,
            profession1: None,
            profession2: None,
            talent_specialization: None,
            race_id: 0,
        },
        character_name: "".to_string(),
        character_guild: if enable_character_guild { Some(build_character_guild(guild_id)) } else { None },
        character_title: None,
        profession_skill_points1: None,
        profession_skill_points2: None,
        facial: None,
        arena_teams: vec![],
        timestamp: 0,
    }
}

fn build_character(enable_character_history: bool, enable_character_guild: bool, guild_id: u32) -> Character {
    Character {
        id: 0,
        server_id: 0,
        server_uid: 0,
        last_update: if enable_character_history { Some(build_character_history(enable_character_guild, guild_id)) } else { None },
        history_moments: vec![],
    }
}

fn build_guild(guild_id: u32, guild_name: String) -> Guild {
    Guild {
        id: guild_id,
        server_id: 0,
        server_uid: 0,
        name: guild_name,
        ranks: vec![],
    }
}

#[test]
fn a1_b1_c1_d1_e1_f1_g2_h1_i2_j1_k2_l1_m1_n2_o2_p1() {
    let armory = Armory::default();
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 2);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a2_b1_c1_d1_e1_f1_g3_h2_i3_j2_k3_l2_m2_n3_o1_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(false, false, 0));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "h".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 2);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 2);
    assert_eq!(tooltip.guild_name, "h".to_string());
    assert_eq!(tooltip.num_member, 0);
}

#[test]
fn a2_b2_c2_d2_e1_f1_g1_h1_i1_j1_k1_l1_m1_n1_o2_p2() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 3);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b1_c1_d1_e1_f1_g1_h1_i1_j1_k1_l1_m1_n3_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(false, false, 0));
    armory.characters.write().unwrap().insert(1, build_character(false, false, 0));
    armory.characters.write().unwrap().insert(2, build_character(false, false, 0));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b2_c2_d1_e2_f1_g2_h1_i2_j1_k2_l1_m1_n1_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(false, false, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d3_e3_f2_g3_h2_i3_j2_k1_l3_m3_n2_o2_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 3));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 3));
    armory.characters.write().unwrap().insert(4, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "h".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "h".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "hs".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 3);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b2_c1_d1_e1_f1_g3_h1_i1_j2_k3_l3_m3_n1_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, false, 0));
    armory.characters.write().unwrap().insert(1, build_character(false, false, 0));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "h".to_string()));
    armory.guilds.write().unwrap().insert(3, build_guild(3, "h".to_string()));
    armory.guilds.write().unwrap().insert(4, build_guild(4, "hs".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 5);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c1_d1_e1_f1_g1_h1_i1_j1_k1_l1_m1_n2_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, false, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, false, 0));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 1);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c2_d2_e1_f1_g2_h1_i2_j1_k2_l1_m1_n3_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, false, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 0));
    armory.guilds.write().unwrap().insert(0, build_guild(1, "".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d1_e2_f3_g1_h1_i1_j1_k1_l1_m1_n1_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 2));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d2_e3_f3_g2_h1_i2_j1_k2_l1_m1_n3_o2_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 2));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d3_e1_f3_g3_h1_i2_j1_k3_l2_m2_n2_o1_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(4, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(5, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "".to_string()));
    armory.guilds.write().unwrap().insert(3, build_guild(3, "h".to_string()));
    armory.guilds.write().unwrap().insert(4, build_guild(4, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 1);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 1);
    assert_eq!(tooltip.guild_name, "".to_string());
    assert_eq!(tooltip.num_member, 0);
}

#[test]
fn a3_b3_c3_d2_e2_f2_g3_h2_i3_j2_k3_l1_m2_n1_o1_p2() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 0);
    assert_eq!(tooltip.guild_name, "".to_string());
    assert_eq!(tooltip.num_member, 1);
}

#[test]
fn a3_b3_c3_d3_e2_f1_g1_h1_i1_j1_k1_l1_m1_n3_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 1));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d1_e3_f2_g1_h1_i1_j1_k1_l1_m1_n1_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a2_b2_c2_d1_e2_f1_g2_h1_i2_j1_k1_l2_m1_n2_o1_p2() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "h".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 1);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 1);
    assert_eq!(tooltip.guild_name, "h".to_string());
    assert_eq!(tooltip.num_member, 1);
}

#[test]
fn a3_b3_c3_d3_e3_f1_g2_h2_i1_j1_k2_l1_m1_n1_o1_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 1));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 0);
    assert_eq!(tooltip.guild_name, "".to_string());
    assert_eq!(tooltip.num_member, 2);
}

#[test]
fn a2_b2_c2_d1_e1_f2_g3_h2_i3_j1_k2_l3_m3_n3_o2_p2() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "h".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "h".to_string()));
    armory.guilds.write().unwrap().insert(3, build_guild(3, "hs".to_string()));
    armory.guilds.write().unwrap().insert(4, build_guild(4, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d2_e3_f2_g2_h1_i2_j1_k1_l2_m1_n2_o2_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 2));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "h".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 2);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a1_b1_c1_d1_e1_f1_g1_h1_i1_j1_k1_l1_m1_n3_o2_p1() {
    let armory = Armory::default();
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a1_b1_c1_d1_e1_f1_g3_h2_i3_j2_k3_l3_m1_n1_o1_p1() {
    let armory = Armory::default();
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "h".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "h".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 0);
    assert_eq!(tooltip.guild_name, "".to_string());
    assert_eq!(tooltip.num_member, 0);
}

#[test]
fn a3_b3_c3_d2_e3_f3_g3_h2_i3_j2_k3_l2_m3_n1_o1_p2() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(4, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "h".to_string()));
    armory.guilds.write().unwrap().insert(3, build_guild(3, "hs".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 0);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 0);
    assert_eq!(tooltip.guild_name, "".to_string());
    assert_eq!(tooltip.num_member, 1);
}

#[test]
fn a2_b2_c2_d2_e1_f1_g3_h1_i1_j2_k3_l2_m2_n2_o2_p1() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "h".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 1);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a1_b1_c1_d1_e1_f1_g3_h1_i3_j2_k2_l2_m2_n3_o2_p1() {
    let armory = Armory::default();
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "h".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294767295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d2_e2_f3_g3_h2_i2_j2_k3_l3_m3_n3_o2_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "h".to_string()));
    armory.guilds.write().unwrap().insert(3, build_guild(3, "h".to_string()));
    armory.guilds.write().unwrap().insert(4, build_guild(4, "hs".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294767295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d1_e3_f3_g3_h2_i2_j2_k1_l3_m2_n2_o1_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(0, build_guild(0, "h".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "h".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(2, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 1);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 1);
    assert_eq!(tooltip.guild_name, "h".to_string());
    assert_eq!(tooltip.num_member, 2);
}

#[test]
fn a1_b1_c1_d1_e1_f1_g3_h2_i2_j2_k3_l1_m3_n3_o1_p1() {
    let armory = Armory::default();
    armory.guilds.write().unwrap().insert(0, build_guild(0, "".to_string()));
    armory.guilds.write().unwrap().insert(1, build_guild(1, "".to_string()));
    armory.guilds.write().unwrap().insert(2, build_guild(2, "hs".to_string()));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(2, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 4294967295);
    assert_eq!(tooltip.guild_name, "hs".to_string());
    assert_eq!(tooltip.num_member, 0);
}

#[test]
fn a3_b3_c3_d2_e1_f3_g2_h1_i1_j2_k1_l1_m2_n3_o1_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    armory.guilds.write().unwrap().insert(4294967295, build_guild(4294967295, "hs".to_string()));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294967295);
    assert!(tooltip_res.is_ok());
    let tooltip = tooltip_res.unwrap();
    assert_eq!(tooltip.guild_id, 4294967295);
    assert_eq!(tooltip.guild_name, "hs".to_string());
    assert_eq!(tooltip.num_member, 2);
}

#[test]
fn a3_b3_c3_d1_e2_f3_g1_h1_i1_j1_k1_l1_m1_n3_o2_p3() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 1));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 4294767295);
    assert!(!tooltip_res.is_ok());
}

#[test]
fn a3_b3_c3_d3_e3_f1_g1_h1_i1_j1_k1_l1_m1_n2_o2_p2() {
    let armory = Armory::default();
    armory.characters.write().unwrap().insert(0, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(1, build_character(true, true, 0));
    armory.characters.write().unwrap().insert(2, build_character(true, true, 4294967295));
    armory.characters.write().unwrap().insert(3, build_character(true, true, 4294967295));
    let tooltip = Tooltip::default().init();
    let tooltip_res = tooltip.get_guild(&armory, 1);
    assert!(!tooltip_res.is_ok());
}
