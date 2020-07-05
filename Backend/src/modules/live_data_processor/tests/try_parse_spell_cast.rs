use crate::modules::armory::Armory;
use crate::modules::live_data_processor::dto::{DamageDone, HealDone, Message, MessageType, SpellCast, Threat, Unit};
use crate::modules::live_data_processor::tools::server::try_parse_spell_cast;
use std::collections::HashMap;
use crate::modules::live_data_processor::tools::MapUnit;

#[test]
fn test_correct_shortcut_condition() {
    // setup dependencies
    let armory = Armory::default();

    let mut summons: HashMap<u64, u64> = HashMap::new();
    summons.insert(1, 1);

    // setup helper objects
    let sender_unit = Unit { is_player: false, unit_id: 0xF140000000000000 };
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
        is_ok: bool,
    };

    // define test cases
    let test_cases = vec![
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone()],
            is_ok: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone()],
            is_ok: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone(), heal_message.clone()],
            is_ok: false,
        },
        TestCase {
            non_committed_messages: vec![spell_cast_message.clone(), spell_damage_message.clone(), heal_message.clone(), threat_message.clone()],
            is_ok: true,
        }
    ];

    let mut last_message = spell_cast_message.clone();
    last_message.timestamp = 42;

    // execute test cases
    for i in 0..test_cases.len() {
        let test_case = test_cases.get(i).unwrap();
        let first_message = test_case.non_committed_messages.first().expect("there must be at least one non-committed message").clone();
        let subject = first_message.message_type.extract_subject().unwrap().to_unit(&armory, 42, &summons).expect("Subject should exist");
        let non_committed_messages = test_case.non_committed_messages.clone();
        let next_message = if i + 1 == test_cases.len() { last_message.clone() } else { test_cases.get(i+1).unwrap().non_committed_messages.last().unwrap().clone() };

        let spell_cast = try_parse_spell_cast(&armory, 42, &summons, &test_case.non_committed_messages, &next_message, &subject);

        assert_eq!(
            spell_cast.is_ok(),
            test_case.is_ok,
            "resulting spell cast should be {} (testing with non-committed messages: {:?})",
            test_case.is_ok,
            non_committed_messages
        );
    }
}
