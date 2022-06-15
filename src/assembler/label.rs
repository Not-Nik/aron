// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::collections::hash_map::Iter as HMIter;
use std::collections::HashMap;

pub struct Label {
    pub name: String,
    pub at: usize,
    pub global: bool
}

struct UnnamedLabel {
    at: Option<usize>,
    global: bool,
}

pub struct LabelMap {
    map: HashMap<String, UnnamedLabel>,
}

pub struct Iter<'a> {
    inner: HMIter<'a, String, UnnamedLabel>
}

impl LabelMap {
    pub fn new() -> Self {
        LabelMap { map: HashMap::new() }
    }

    pub fn insert_label(&mut self, name: String, at: usize) {
        self.map.insert(name, UnnamedLabel { at: Some(at), global: false });
    }

    pub fn make_global(&mut self, name: String) {
        if self.map.contains_key(&*name) {
            self.map.get_mut(name.as_str()).unwrap().global = true;
        } else {
            self.map.insert(name, UnnamedLabel { at: None, global: true });
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            inner: self.map.iter()
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Label;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.inner.next();

        if let Some(i) = i {
            if i.1.at.is_some() {
                Some(Label {
                    name: i.0.clone(),
                    at: i.1.at.unwrap(),
                    global: i.1.global
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

