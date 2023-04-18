use std::{collections::HashMap, io, hash::Hash};

use dialoguer::{Select, theme::ColorfulTheme};

pub fn hashmap_select<K, V>(hashmap: &HashMap<K,V>, prompt: &str) -> io::Result<(K,V)> 
    where K: ToString + PartialEq + Eq + Hash + Clone,
        V: Clone {
    let keys = hashmap.keys().map(|x| x.clone()).collect::<Vec<K>>();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(keys.as_slice())
        .default(0)
        .with_prompt(prompt)
        .interact()?;
    let key = keys.get(selection).unwrap(); // Infallible
    let value = hashmap.get(key).unwrap(); // Infallible
    Ok((
        key.clone(),
        value.clone()
    ))
}