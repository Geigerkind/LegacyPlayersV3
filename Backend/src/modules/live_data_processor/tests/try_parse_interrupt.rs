use crate::modules::live_data_processor::tools::server::try_parse_interrupt;
use crate::modules::live_data_processor::dto as dto;
use crate::modules::live_data_processor::domain_value::{Event, EventType, SpellCast, HitType, Unit, Creature, AuraApplication};

#[test]
fn test_no_events() {
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };

    let subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });

    let timestamp: u64 = 123;

    let result = try_parse_interrupt(
        &interrupt, [].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_timestamp() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let spell_cast = SpellCast {
        victim: None,
        hit_type: HitType::Crit,
        spell_id: None,
        damage: vec![],
        heal: vec![],
        threat: vec![]
    };
    let event_type = EventType::SpellCast(spell_cast);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });

    let timestamp: u64 = 123;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_spellcast_no_spell_id() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let spell_cast = SpellCast {
        victim: None,
        hit_type: HitType::Crit,
        spell_id: None,
        damage: vec![],
        heal: vec![],
        threat: vec![]
    };
    let event_type = EventType::SpellCast(spell_cast);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_spellcast_different_victim_no_direct_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let victim = Unit::Creature(Creature {
        creature_id: 2,
        entry: 2,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let spell_cast = SpellCast {
        victim: Some(victim),
        hit_type: HitType::Crit,
        spell_id: Some(73),
        damage: vec![],
        heal: vec![],
        threat: vec![]
    };
    let event_type = EventType::SpellCast(spell_cast);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_spellcast_hit_victim_no_direct_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let victim = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let spell_cast = SpellCast {
        victim: Some(victim),
        hit_type: HitType::Crit,
        spell_id: Some(73),
        damage: vec![],
        heal: vec![],
        threat: vec![]
    };
    let event_type = EventType::SpellCast(spell_cast);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_spellcast_different_victim_direct_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let victim = Unit::Creature(Creature {
        creature_id: 2,
        entry: 2,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let spell_cast = SpellCast {
        victim: Some(victim),
        hit_type: HitType::Crit,
        spell_id: Some(72),
        damage: vec![],
        heal: vec![],
        threat: vec![]
    };
    let event_type = EventType::SpellCast(spell_cast);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_spellcast_hit_victim_direct_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let victim = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let spell_cast = SpellCast {
        victim: Some(victim),
        hit_type: HitType::Crit,
        spell_id: Some(72),
        damage: vec![],
        heal: vec![],
        threat: vec![]
    };
    let event_type = EventType::SpellCast(spell_cast);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_ok());
}

#[test]
fn test_aura_app_wrong_subject_no_indirect_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let aura_caster = Unit::Creature(Creature {
        creature_id: 2,
        entry: 2,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let aura_application = AuraApplication {
        caster: aura_caster,
        stack_amount: 0,
        spell_id: 407
    };
    let event_type = EventType::AuraApplication(aura_application);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_aura_app_correct_subject_no_indirect_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });
    let aura_caster = Unit::Creature(Creature {
        creature_id: 2,
        entry: 2,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let aura_application = AuraApplication {
        caster: aura_caster,
        stack_amount: 0,
        spell_id: 407
    };
    let event_type = EventType::AuraApplication(aura_application);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_aura_app_wrong_subject_indirect_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let aura_caster = Unit::Creature(Creature {
        creature_id: 2,
        entry: 2,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let aura_application = AuraApplication {
        caster: aura_caster,
        stack_amount: 0,
        spell_id: 408
    };
    let event_type = EventType::AuraApplication(aura_application);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}

#[test]
fn test_aura_app_correct_subject_indirect_interrupt() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });
    let aura_caster = Unit::Creature(Creature {
        creature_id: 2,
        entry: 2,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let aura_application = AuraApplication {
        caster: aura_caster,
        stack_amount: 0,
        spell_id: 408
    };
    let event_type = EventType::AuraApplication(aura_application);
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_ok());
}

#[test]
fn test_unhandled_event() {
    let event_subject = Unit::Creature(Creature {
        creature_id: 0,
        entry: 0,
        owner: None
    });
    let interrupt = dto::Interrupt {
        target: dto::Unit {
            is_player: true,
            unit_id: 1
        },
        interrupted_spell_id: 1,
    };
    let event_type = EventType::ThreatWipe;
    let committed_event = Event::new(123456, event_subject, event_type);

    let subject = Unit::Creature(Creature {
        creature_id: 1,
        entry: 1,
        owner: None
    });

    let timestamp: u64 = 123455;

    let result = try_parse_interrupt(
        &interrupt, [committed_event].as_ref(), timestamp, &subject);
    assert!(result.is_err());
}
