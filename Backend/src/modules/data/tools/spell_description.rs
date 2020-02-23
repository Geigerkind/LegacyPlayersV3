use language::domain_value::Language;
use language::material::Dictionary;
use language::tools::Get;
use regex::Regex;

use crate::modules::data::{Data, Stat};
use crate::modules::data::tools::{RetrieveLocalization, RetrieveSpell, RetrieveSpellEffect};

pub trait SpellDescription {
  fn get_localized_spell_description(&self, expansion_id: u8, language_id: u8, spell_id: u32) -> Option<String>;
  fn parse_stats(&self, expansion_id: u8, spell_id: u32) -> Vec<Stat>;
}

impl SpellDescription for Data {
  fn get_localized_spell_description(&self, expansion_id: u8, language_id: u8, spell_id: u32) -> Option<String> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"\$(\d+)(s\d|x\d|a\d|o\d|m\d|M\d|d|o)").unwrap();
    }

    let spell_res = self.get_spell(expansion_id, spell_id);
    if spell_res.is_none() {
      return None;
    }
    let spell = spell_res.unwrap();
    let mut template = self.get_localization(language_id, spell.description_localization_id)
      .and_then(|localization| Some(localization.content)).unwrap();

    let spell_effects = self.get_spell_effects(expansion_id, spell_id).unwrap();
    template = template.replace("$d1", &format_duration(&self.dictionary, language_id, spell.duration.abs() as u32));
    template = template.replace("$d", &format_duration(&self.dictionary, language_id, spell.duration.abs() as u32));
    for i in 0..spell_effects.len() {
      template = template.replace(&format!("$s{}", i + 1), &spell_effects[i].points_upper.abs().to_string());
      template = template.replace(&format!("${{$m{}/-1000}}.1", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 1000.0).abs()));
      template = template.replace(&format!("${{$m{}/-1000}}.2", i + 1), &format!("{:.2}", (spell_effects[i].points_upper as f64 / 1000.0).abs()));
      template = template.replace(&format!("${{$m{}/1000}}", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 1000.0).abs()));
      template = template.replace(&format!("${{$m{}/-1000}}", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 1000.0).abs()));
      template = template.replace(&format!("${{$m{}/-10}}", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 10.0).abs()));
      template = template.replace(&format!("$/1000;s{}", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 1000.0).abs()));
      template = template.replace(&format!("$/1000;S{}", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 1000.0).abs()));
      template = template.replace(&format!("$/10;s{}", i + 1), &format!("{:.1}", (spell_effects[i].points_upper as f64 / 10.0).abs()));
      template = template.replace(&format!("$M{}", i + 1), &spell_effects[i].points_upper.to_string());
      template = template.replace(&format!("$o{}", i + 1), &spell_effects[i].points_upper.to_string());
      template = template.replace(&format!("$x{}", i + 1), &spell_effects[i].chain_targets.to_string());
      template = template.replace(&format!("$a{}", i + 1), &spell_effects[i].radius.to_string());

      template = template.replace(&format!("$o"), &spell_effects[i].points_upper.to_string()); // ?!
      if RE.is_match(&template) {
        let mut temp_res = template.clone();
        for capture in RE.captures_iter(&template) {
          if capture.len() < 2 {
            continue;
          }

          let inner_spell_id_res = capture[1].parse::<u32>();
          if inner_spell_id_res.is_err() {
            continue;
          }
          let inner_spell_id = inner_spell_id_res.unwrap();

          let inner_spell_res = self.get_spell(expansion_id, inner_spell_id);
          if inner_spell_res.is_none() {
            continue;
          }
          let inner_spell = inner_spell_res.unwrap();
          temp_res = temp_res.replace(&format!("${}d1", capture[1].to_string()), &format_duration(&self.dictionary, language_id, inner_spell.duration.abs() as u32));
          temp_res = temp_res.replace(&format!("${}d", capture[1].to_string()), &format_duration(&self.dictionary, language_id, inner_spell.duration.abs() as u32));
          let inner_spell_effects = self.get_spell_effects(expansion_id, inner_spell_id).unwrap();
          for i in 0..inner_spell_effects.len() {
            temp_res = temp_res.replace(&format!("${}s{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].points_upper.abs().to_string());
            temp_res = temp_res.replace(&format!("${{${}m{}/-1000}}.1", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 1000.0).abs()));
            temp_res = temp_res.replace(&format!("${{${}m{}/-1000}}.2", capture[1].to_string(), i + 1), &format!("{:.2}", (inner_spell_effects[i].points_upper as f64 / 1000.0).abs()));
            temp_res = temp_res.replace(&format!("${}{{$m{}/1000}}", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 1000.0).abs()));
            temp_res = temp_res.replace(&format!("${}{{$m{}/-1000}}", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 1000.0).abs()));
            temp_res = temp_res.replace(&format!("${}{{$m{}/-10}}", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 10.0).abs()));
            temp_res = temp_res.replace(&format!("${}/1000;s{}", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 1000.0).abs()));
            temp_res = temp_res.replace(&format!("${}/1000;S{}", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 1000.0).abs()));
            temp_res = temp_res.replace(&format!("${}/10;s{}", capture[1].to_string(), i + 1), &format!("{:.1}", (inner_spell_effects[i].points_upper as f64 / 10.0).abs()));
            temp_res = temp_res.replace(&format!("${}o{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].points_upper.abs().to_string());
            temp_res = temp_res.replace(&format!("${}x{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].chain_targets.to_string());
            temp_res = temp_res.replace(&format!("${}a{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].radius.to_string());
            temp_res = temp_res.replace(&format!("${}o", capture[1].to_string()), &inner_spell_effects[i].points_upper.abs().to_string()); // ?!
          }
        }
        template = temp_res.to_owned();
      }
    }

    Some(template)
  }

  // Parsing it for now!
  // TODO: BIG REFACTOR!
  fn parse_stats(&self, expansion_id: u8, spell_id: u32) -> Vec<Stat> {
    lazy_static! {
      static ref RE_DURATIONAL: Regex = Regex::new(r"for \d+ seconds.").unwrap();
    }

    let mut stats = Vec::new();
    if let Some(spell_desc) = self.get_localized_spell_description(expansion_id, 1, spell_id) {
      let spell_desc = spell_desc.to_lowercase();
      if RE_DURATIONAL.is_match(&spell_desc) {
        return stats;
      }

      let spell_effects = self.get_spell_effects(expansion_id, spell_id).unwrap();
      if spell_desc.contains("increases") || spell_desc.contains("improves") {
        if spell_desc.contains("spell critical strike rating") {
          stats.push(Stat { stat_type: 24, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("critical strike rating") {
          stats.push(Stat { stat_type: 8, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("spell hit rating") {
          stats.push(Stat { stat_type: 23, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("hit rating") {
          stats.push(Stat { stat_type: 7, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("resilience rating") {
          stats.push(Stat { stat_type: 39, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage and healing done by") {
          stats.push(Stat { stat_type: 13, stat_value: spell_effects[0].points_upper.abs() as u16 });
          stats.push(Stat { stat_type: 14, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("healing done by up to") && spell_desc.contains("damage done by up to") {
          stats.push(Stat { stat_type: 14, stat_value: spell_effects[0].points_upper.abs() as u16 });
          stats.push(Stat { stat_type: 13, stat_value: spell_effects[1].points_upper.abs() as u16 });
        } else if spell_desc.contains("spell haste rating") {
          stats.push(Stat { stat_type: 42, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("haste rating") {
          stats.push(Stat { stat_type: 37, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("ranged attack power") {
          stats.push(Stat { stat_type: 25, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("attack power") {
          stats.push(Stat { stat_type: 9, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("defense rating") {
          stats.push(Stat { stat_type: 22, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("dodge rating") {
          stats.push(Stat { stat_type: 10, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("parry rating") {
          stats.push(Stat { stat_type: 11, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("block rating") {
          stats.push(Stat { stat_type: 12, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("block chance") {
          stats.push(Stat { stat_type: 26, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage done by shadow spells") {
          stats.push(Stat { stat_type: 20, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage done by arcane spells") {
          stats.push(Stat { stat_type: 19, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage done by fire spells") {
          stats.push(Stat { stat_type: 18, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage done by nature spells") {
          stats.push(Stat { stat_type: 17, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage done by frost spells") {
          stats.push(Stat { stat_type: 16, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("damage done by holy spells") {
          stats.push(Stat { stat_type: 15, stat_value: spell_effects[0].points_upper.abs() as u16 });
        }
      } else if spell_desc.contains("restores") {
        if spell_desc.contains("mana per") {
          stats.push(Stat { stat_type: 21, stat_value: spell_effects[0].points_upper.abs() as u16 });
        } else if spell_desc.contains("health per") {
          stats.push(Stat { stat_type: 43, stat_value: spell_effects[0].points_upper.abs() as u16 });
        }
      } else if spell_desc.contains("attacks ignore") && spell_desc.contains("opponent's armor") {
        stats.push(Stat { stat_type: 41, stat_value: spell_effects[0].points_upper.abs() as u16 });
      } else {
        let mut index = 0;
        for spell_desc_substr in spell_desc.split(" and ") {
          if spell_desc_substr.contains("+") {
            if spell_desc_substr.contains("chance to") {
              continue;
            }

            if spell_desc_substr.contains("holy resistance") {
              stats.push(Stat { stat_type: 1, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("fire resistance") {
              stats.push(Stat { stat_type: 2, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("nature resistance") {
              stats.push(Stat { stat_type: 3, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("frost resistance") {
              stats.push(Stat { stat_type: 4, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("arcane resistance") {
              stats.push(Stat { stat_type: 5, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("shadow resistance") {
              stats.push(Stat { stat_type: 6, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("spell hit rating") {
              stats.push(Stat { stat_type: 23, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("spell critical") {
              stats.push(Stat { stat_type: 24, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("hit rating") {
              stats.push(Stat { stat_type: 7, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("critical strike") {
              stats.push(Stat { stat_type: 8, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("ranged attack power") {
              stats.push(Stat { stat_type: 25, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("attack power") {
              stats.push(Stat { stat_type: 9, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("dodge rating") {
              stats.push(Stat { stat_type: 10, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("parry rating") {
              stats.push(Stat { stat_type: 11, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("block") {
              stats.push(Stat { stat_type: 12, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("spell damage") {
              stats.push(Stat { stat_type: 13, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("healing") {
              stats.push(Stat { stat_type: 14, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("holy spell damage") {
              stats.push(Stat { stat_type: 15, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("frost spell damage") {
              stats.push(Stat { stat_type: 16, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("nature spell damage") {
              stats.push(Stat { stat_type: 17, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("fire spell damage") {
              stats.push(Stat { stat_type: 18, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("arcane spell damage") {
              stats.push(Stat { stat_type: 19, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("shadow spell damage") {
              stats.push(Stat { stat_type: 20, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("mana regen") || spell_desc_substr.contains("mana every") || spell_desc_substr.contains("mana per") {
              stats.push(Stat { stat_type: 21, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("defense rating") {
              stats.push(Stat { stat_type: 22, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("block chance") {
              stats.push(Stat { stat_type: 26, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("strength") {
              stats.push(Stat { stat_type: 27, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("agility") {
              stats.push(Stat { stat_type: 28, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("stamina") {
              stats.push(Stat { stat_type: 29, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("intellect") {
              stats.push(Stat { stat_type: 30, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("spirit") {
              stats.push(Stat { stat_type: 31, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("mana") {
              stats.push(Stat { stat_type: 32, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("health regen") || spell_desc_substr.contains("health every") || spell_desc_substr.contains("health per") {
              stats.push(Stat { stat_type: 43, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("health") {
              stats.push(Stat { stat_type: 33, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("armor") {
              stats.push(Stat { stat_type: 34, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("spell haste") {
              stats.push(Stat { stat_type: 42, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("haste") {
              stats.push(Stat { stat_type: 37, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("expertise rating") {
              stats.push(Stat { stat_type: 38, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("resilience rating") {
              stats.push(Stat { stat_type: 39, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("spell penetration") {
              stats.push(Stat { stat_type: 40, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("armor penetration") {
              stats.push(Stat { stat_type: 41, stat_value: spell_effects[index].points_upper.abs() as u16 });
            } else if spell_desc_substr.contains("all stats") {
              stats.push(Stat { stat_type: 27, stat_value: spell_effects[index].points_upper.abs() as u16 });
              stats.push(Stat { stat_type: 28, stat_value: spell_effects[index].points_upper.abs() as u16 });
              stats.push(Stat { stat_type: 29, stat_value: spell_effects[index].points_upper.abs() as u16 });
              stats.push(Stat { stat_type: 30, stat_value: spell_effects[index].points_upper.abs() as u16 });
              stats.push(Stat { stat_type: 31, stat_value: spell_effects[index].points_upper.abs() as u16 });
            }
            index += 1;
          }
        }
      }
    }
    stats
  }
}

fn format_duration(dictionary: &Dictionary, language_id: u8, duration: u32) -> String {
  let language = Language::from_u8(language_id - 1);

  if duration > 24 * 60 * 60 * 1000 {
    return str_util::strformat::fmt(dictionary.get("days", language), &[&(duration / (24 * 60 * 60 * 1000)).to_string()]);
  } else if duration == 24 * 60 * 60 * 1000 {
    return dictionary.get("day", language);
  }

  if duration > 60 * 60 * 1000 {
    return str_util::strformat::fmt(dictionary.get("hours", language), &[&(duration / (60 * 60 * 1000)).to_string()]);
  } else if duration == 60 * 60 * 1000 {
    return dictionary.get("hour", language);
  }

  if duration > 60 * 1000 {
    return str_util::strformat::fmt(dictionary.get("minutes", language), &[&(duration / (60 * 1000)).to_string()]);
  } else if duration == 60 * 1000 {
    return dictionary.get("minute", language);
  }

  if duration > 1000 {
    return str_util::strformat::fmt(dictionary.get("seconds", language), &[&(duration / (1000)).to_string()]);
  } else if duration == 1000 {
    return dictionary.get("second", language);
  }

  return str_util::strformat::fmt(dictionary.get("milliseconds", language), &[&(duration / (24 * 60 * 60 * 1000)).to_string()]);
}