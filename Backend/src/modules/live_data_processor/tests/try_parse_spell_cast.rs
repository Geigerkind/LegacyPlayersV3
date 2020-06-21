use crate::modules::armory::Armory;
use crate::modules::live_data_processor::dto::{DamageDone, HealDone, Message, MessageType, SpellCast, Threat, Unit};
use crate::modules::live_data_processor::tools::server::{try_parse_spell_cast};
use std::collections::HashMap;

#[test]
fn test_correct_shortcut_condition() {
    // setup dependencies
    let armory = Armory::default();

    let mut summons: HashMap<u64, u64> = HashMap::new();
    summons.insert(1, 1);

    // setup helper objects
    let sender_unit = Unit {
        is_player: false,
        unit_id: 0xF140000000000000,
    };
    let receiver_unit = Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 1,
    };
    let spell_cast_message = Message {
        api_version: 0,
        message_length: 0,
        timestamp: 0,
        message_type: MessageType::SpellCast(SpellCast {
            caster: sender_unit.clone(),
            target: Some(receiver_unit.clone()),
            spell_id: 42,
            hit_type: 7,
        }),
    };
    let heal_message = Message {
        api_version: 0,
        message_length: 0,
        timestamp: 0,
        message_type: MessageType::Heal(HealDone {
            caster: sender_unit.clone(),
            target: receiver_unit.clone(),
            spell_id: 42,
            total_heal: 420,
            effective_heal: 42,
            absorb: 2,
        }),
    };
    let spell_damage_message = Message {
        api_version: 0,
        message_length: 0,
        timestamp: 0,
        message_type: MessageType::SpellDamage(DamageDone {
            attacker: sender_unit.clone(),
            victim: receiver_unit.clone(),
            spell_id: Some(42),
            hit_type: None,
            blocked: 1,
            school: 2,
            damage: 10,
            resisted_or_glanced: 1,
            absorbed: 1,
        }),
    };
    let threat_message = Message {
        api_version: 0,
        message_length: 0,
        timestamp: 0,
        message_type: MessageType::Threat(Threat {
            threater: sender_unit,
            threatened: receiver_unit,
            spell_id: Some(42),
            amount: 42,
        }),
    };

    // define test case helper struct
    #[derive(Debug)]
    struct TestCase {
        non_committed_messages: Vec<Message>,
        is_some: bool,
    };

    // define test cases
    let mut test_cases = vec![
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), heal_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone(), heal_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), threat_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone(), threat_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), heal_message.clone(), threat_message.clone()],
            is_some: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message, spell_damage_message, heal_message, threat_message],
            is_some: true,
        },
    ];

    // execute test cases
    for test_case in test_cases.iter_mut() {
        let first_message = test_case.non_committed_messages.first().expect("there must be at least one non-committed message").clone();
        let non_committed_messages = test_case.non_committed_messages.clone();

        let spell_cast = try_parse_spell_cast(&mut test_case.non_committed_messages, &summons, &first_message, &armory, 42);

        assert_eq!(
            spell_cast.is_some(),
            test_case.is_some,
            "resulting spell cast should be {} (testing with non-committed messages: {:?})",
            test_case.is_some,
            non_committed_messages
        );
    }
}
