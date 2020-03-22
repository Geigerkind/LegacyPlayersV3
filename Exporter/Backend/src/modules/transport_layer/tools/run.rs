use std::{thread, env};
use std::time::Duration;

use crate::modules::transport_layer::tools::{ReceiveConsent, GUID};
use crate::modules::TransportLayer;
use crate::Run;
use reqwest::header::{HeaderValue, CONTENT_TYPE};

use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

impl Run for TransportLayer {
  fn run(&mut self) {
    let rate = env::var("REQUESTS_TO_LP_PER_SECOND").unwrap().parse::<f64>().unwrap();
    let api_token = env::var("LP_API_TOKEN").unwrap();
    let url_set_character = env::var("URL_SET_CHARACTER").unwrap();
    let opt_in_mode = env::var("OPT_IN_MODE").unwrap().parse::<bool>().unwrap();

    // Relay for Server messages
    thread::spawn(|| {
      let context = zmq::Context::new();
      let responder = context.socket(zmq::PULL).unwrap();
      assert!(responder.bind("tcp://0.0.0.0:5690").is_ok());
      println!("Established ZMQ socket!");

      loop {
        let msg = responder.recv_bytes(0).unwrap();
        println!("Received {:?}", msg);

        // Reading the bytes as Big Endian
        let message_type = &msg[0];
        println!("Message type: {}", *message_type);
        let message_length = &msg[1];
        println!("Message length: {}", message_length);
        let mut rdr = Cursor::new(&msg[2..10]);
        let timestamp = rdr.read_u64::<LittleEndian>().unwrap();
        println!("Timestamp: {}", timestamp);
        /*
        if *message_type == 0 {
          let mut rdr = Cursor::new(&msg[10..14]);
          let map_id = rdr.read_u32::<LittleEndian>().unwrap();
          println!("Map Id: {}", map_id);

          let mut rdr = Cursor::new(&msg[14..18]);
          let instance_id = rdr.read_u32::<LittleEndian>().unwrap();
          println!("Instance_id: {}", instance_id);
          println!("Difficulty: {}", &msg[18]);

          let mut rdr = Cursor::new(&msg[19..23]);
          let target_max_health = rdr.read_u32::<LittleEndian>().unwrap();
          println!("Target max health: {}", target_max_health);

          let mut rdr = Cursor::new(&msg[23..27]);
          let target_health = rdr.read_u32::<LittleEndian>().unwrap();
          println!("Target health: {}", target_health);

          let mut rdr = Cursor::new(&msg[27..35]);
          let attacker_guid = rdr.read_u64::<LittleEndian>().unwrap();
          println!("Attacker GUID: {} => {}", attacker_guid, attacker_guid.is_player());

          let mut rdr = Cursor::new(&msg[35..43]);
          let victim_guid = rdr.read_u64::<LittleEndian>().unwrap();
          println!("Victim GUID: {} => {} / {:?}", victim_guid, victim_guid.is_any_creature(), victim_guid.get_entry());

          let mut rdr = Cursor::new(&msg[43..47]);
          let hit_info = rdr.read_u32::<LittleEndian>().unwrap();
          println!("HitInfo: {}", hit_info);

          let mut rdr = Cursor::new(&msg[47..51]);
          let blocked = rdr.read_u32::<LittleEndian>().unwrap();
          println!("Blocked: {}", blocked);

          let mut rdr = Cursor::new(&msg[51..55]);
          let spell_id = rdr.read_u32::<LittleEndian>().unwrap();
          println!("SpellId: {}", spell_id);

          for i in (55..(*message_length)).step_by(16) {
            let i: usize = i as usize;

            let mut rdr = Cursor::new(&msg[i..(i+4)]);
            let school_mask = rdr.read_u32::<LittleEndian>().unwrap();ß
            println!("SchoolMask: {}", school_mask);

            let mut rdr = Cursor::new(&msg[(i+4)..(i+8)]);
            let damage = rdr.read_u32::<LittleEndian>().unwrap();
            println!("Damage: {}", damage);

            let mut rdr = Cursor::new(&msg[(i+8)..(i+12)]);
            let absorb = rdr.read_u32::<LittleEndian>().unwrap();
            println!("Absorb: {}", absorb);

            let mut rdr = Cursor::new(&msg[(i+12)..(i+16)]);
            let resist = rdr.read_u32::<LittleEndian>().unwrap();
            println!("Resist: {}", resist);
          }
        }
        */
      }
    });

    let sleep_duration_rate = Duration::new(0, (1000000000.0 as f64 * (1.0 / rate)).ceil() as u32);
    let sleep_duration_wait = Duration::new(1, 0);
    loop {
      thread::sleep(sleep_duration_rate);

      // Receive new consent decisions
      self.receive_character_consent();
      self.receive_guild_consent();

      // Relay Character DTOs
      let receiver = self.receiver_character.as_ref().unwrap();
      let received_res = receiver.try_recv();
      if received_res.is_ok() {
        let received = received_res.unwrap();
        if (opt_in_mode && !self.character_consent.contains(&received.0)) || (!opt_in_mode && self.character_consent.contains(&received.0)) {
          println!("{} ({}) has not given consent, skipping!", received.1.character_history.unwrap().character_name, received.1.server_uid);
          continue;
        }

        let response = self.client
          .post(&url_set_character)
          .header("X-Authorization", HeaderValue::from_str(api_token.as_str()).unwrap())
          .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
          .body(serde_json::to_string(&received.1).unwrap())
          .send();
        println!("Response okay => {:?} for {} ({})", response.is_ok(), received.1.character_history.unwrap().character_name, received.1.server_uid);
      } else {
        thread::sleep(sleep_duration_wait);
      }
    }
  }
}