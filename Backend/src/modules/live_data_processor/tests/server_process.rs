use crate::modules::live_data_processor::tools::server::ParseEvents;
use crate::modules::live_data_processor::material::Server;
use crate::modules::live_data_processor::dto::{Message, MessageType, DamageDone, SpellCast, Unit};
use crate::modules::armory::Armory;

#[test]
fn parse_spell_damage() {
  // Arrange
  let mut server = Server::default();
  let armory = Armory::default();
  let server_id = 2;
  let mut messages = Vec::new();
  messages.push(Message {
    api_version: 0,
    message_length: 0,
    timestamp: 0,
    message_type: MessageType::SpellDamage(DamageDone {
      attacker: Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 42
      },
      victim: Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 43
      },
      spell_id: Some(26),
      hit_type: None,
      blocked: 1,
      school: 2,
      damage: 10,
      resisted_or_glanced: 1,
      absorbed: 1
    })
  });

  // Act + Assert
  let parse_result1 = server.parse_events(&armory, server_id, messages);
  assert!(parse_result1.is_ok());
  assert!(!server.non_committed_messages.is_empty());

  let mut messages = Vec::new();
  messages.push(Message {
    api_version: 0,
    message_length: 0,
    timestamp: 5,
    message_type: MessageType::SpellCast(SpellCast {
      caster: Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 42
      },
      target: Some(Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 43
      }),
      spell_id: 26,
      hit_type: 7
    })
  });

  let parse_result2 = server.parse_events(&armory, server_id, messages);
  assert!(parse_result2.is_ok());
  assert_eq!(server.non_committed_messages.len(), 2);

  let mut messages = Vec::new();
  messages.push(Message {
    api_version: 0,
    message_length: 0,
    timestamp: 75,
    message_type: MessageType::SpellCast(SpellCast {
      caster: Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 43
      },
      target: Some(Unit {
        is_player: false,
        unit_id: 0xF140000000000000 + 22
      }),
      spell_id: 22,
      hit_type: 1
    })
  });

  let parse_result3 = server.parse_events(&armory, server_id, messages);
  assert!(parse_result3.is_ok());
  assert_eq!(server.non_committed_messages.len(), 1);
  assert_eq!(server.committed_events.len(), 1);
}