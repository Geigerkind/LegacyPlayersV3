use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveSpell, RetrieveLocalization, RetrieveSpellEffect};
use regex::Regex;
use language::domain_value::Language;
use language::material::Dictionary;
use language::tools::Get;

pub trait SpellDescription {
  fn get_localized_spell_description(&self, expansion_id: u8, language_id: u8, spell_id: u32) -> Option<String>;
}

impl SpellDescription for Data {
  fn get_localized_spell_description(&self, expansion_id: u8, language_id: u8, spell_id: u32) -> Option<String> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"\$(\d+)(s\d|x\d|a\d|o\d|d)").unwrap();
    }

    let spell_res = self.get_spell(expansion_id, spell_id);
    if spell_res.is_none() {
      return None;
    }
    let spell = spell_res.unwrap();
    let mut template = self.get_localization(language_id, spell.description_localization_id)
      .and_then(|localization| Some(localization.content)).unwrap();

    let spell_effects = self.get_spell_effects(expansion_id, spell_id).unwrap();
    template = template.replace("$d", &format_duration(&self.dictionary, language_id, spell.duration.abs() as u32));
    for i in 0..spell_effects.len() {
      template = template.replace(&format!("$s{}", i+1), &spell_effects[i].points_upper.abs().to_string());
      template = template.replace(&format!("$o{}", i+1), &spell_effects[i].points_upper.to_string());
      template = template.replace(&format!("$x{}", i+1), &spell_effects[i].chain_targets.to_string());
      template = template.replace(&format!("$a{}", i+1), &spell_effects[i].radius.to_string());
      if RE.is_match(&template) {
        let mut temp_res = template.clone();
        for capture in RE.captures_iter(&template) {
          let inner_spell_id_res = capture[1].parse::<u32>();
          if inner_spell_id_res.is_err() {
            return Some(template);
          }
          let inner_spell_id = inner_spell_id_res.unwrap();

          let inner_spell_res = self.get_spell(expansion_id, inner_spell_id);
          if inner_spell_res.is_none() {
            return Some(template);
          }
          let inner_spell = inner_spell_res.unwrap();
          temp_res = temp_res.replace(&format!("${}d", capture[1].to_string()), &format_duration(&self.dictionary, language_id, inner_spell.duration.abs() as u32));
          let inner_spell_effects = self.get_spell_effects(expansion_id, inner_spell_id).unwrap();
          for i in 0..inner_spell_effects.len() {
            temp_res = temp_res.replace(&format!("${}s{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].points_upper.abs().to_string());
            temp_res = temp_res.replace(&format!("${}o{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].points_upper.abs().to_string());
            temp_res = temp_res.replace(&format!("${}x{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].chain_targets.to_string());
            temp_res = temp_res.replace(&format!("${}a{}", capture[1].to_string(), i + 1), &inner_spell_effects[i].radius.to_string());
          }
        }
        template = temp_res.to_owned();
      }

    }

    Some(template)
  }
}

fn format_duration(dictionary: &Dictionary, language_id: u8, duration: u32) -> String {
  let language = Language::from_u8(language_id - 1);

  if duration > 24 * 60 * 60 * 1000 {
    return str_util::strformat::fmt(dictionary.get("days", language), &[&(duration / (24*60*60*1000)).to_string()]);
  } else if duration == 24 * 60 * 60 * 1000 {
    return dictionary.get("day", language);
  }

  if duration > 60 * 60 * 1000 {
    return str_util::strformat::fmt(dictionary.get("hours", language), &[&(duration / (60*60*1000)).to_string()]);
  } else if duration == 60 * 60 * 1000 {
    return dictionary.get("hour", language);
  }

  if duration > 60 * 1000 {
    return str_util::strformat::fmt(dictionary.get("minutes", language), &[&(duration / (60*1000)).to_string()]);
  } else if duration == 60 * 1000 {
    return dictionary.get("minute", language);
  }

  if duration > 1000 {
    return str_util::strformat::fmt(dictionary.get("seconds", language), &[&(duration / (1000)).to_string()]);
  } else if duration == 1000 {
    return dictionary.get("second", language);
  }

  return str_util::strformat::fmt(dictionary.get("milliseconds", language), &[&(duration / (24*60*60*1000)).to_string()]);
}