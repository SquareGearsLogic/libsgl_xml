// Licensed to the Square Gears Logic (SGL) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SGL licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::path::Path;

use xml_item::{XmlItem, XmlItemRc};

/// Represents a bunch of DOM-related algorithms. 
pub struct XmlDom;

impl XmlDom {
    /// Reads XML document into browsable DOM structure with single root element.
    ///
    /// This parser supports:
    ///
    /// ```xml
    ///  <One/> 
    ///  <tag/>
    ///  <per/>
    ///  <line/>
    /// ```
    ///
    /// ```xml
    ///  <!-- Multi
    ///         Line
    ///           comments -->
    /// ```
    ///
    /// ```xml
    ///  <  multi
    ///          line="
    ///             tags"
    /// 
    ///  >
    /// ```
    ///
    /// and multiple attributes, that may contain slashed quoutes \\"
    ///
    pub fn open(filename: &Path) -> Result<XmlItemRc, String> {
        let mut result = Err("Can't open xml".to_string());
        let file = match File::open(filename.as_os_str()) {
            Ok(file) => file,
            Err(_) => return result,
        };

        let mut tag_content = "".to_string();
        let mut is_tag_begin_found = false;
        let mut is_tag_end_found = false;
        let mut is_comment = false;
        let mut current_parrent = XmlItem::get_empty_item();

        let file = BufReader::new(&file);
        for line in file.lines() {
            let line = line.unwrap();

            // Checking closing scope first because in case of comment we can
            // win time by continuing the loop early.
            if let &Some(pos) = &line.find(">") {
                if pos >= 3 && substr_any(&line, &(pos - 2), &pos).eq("-->") {
                    is_comment = false;
                    continue;
                }
                is_tag_end_found = true;
            }

            if let Some(pos) = line.find("<") {
                if line.chars().count() >= pos + 2 &&
                   substr_any(&line, &pos, &(pos + 2)).eq("<--") {
                    is_comment = true;
                    continue;
                }
                is_tag_begin_found = true;
            }

            if is_comment == true {
                continue;
            }

            if !line.trim().is_empty() {
                tag_content = format!("{}\n{}", tag_content, line.trim());
            }

            if is_tag_end_found == false {
                continue;
            } else if is_tag_begin_found == false {
                return Err("There is no matching '<' for '>'".to_string());
            }

            if let Some(_) = line.find("</") {
                let parent = XmlItem::get_parent(current_parrent.clone());
                if (*parent.borrow_mut()).is_some() {
                    current_parrent = parent.clone();
                }
            } else if let Some(_) = line.find("/>") {
                match &mut Self::parse_tag(tag_content) {
                    &mut Ok(ref mut val) => {
                        XmlItem::add_node(current_parrent.clone(), val.clone());
                    }
                    &mut Err(ref mut val) => {
                        return Err(val.clone());
                    }
                }
            } else {
                match &mut Self::parse_tag(tag_content) {
                    &mut Ok(ref mut val) => {
                        if (*current_parrent.borrow_mut()).is_some() {
                            XmlItem::add_node(current_parrent.clone(), val.clone());
                        }
                        current_parrent = val.clone();
                        if result.is_err() {
                            result = Ok(current_parrent.clone());
                        }
                    }
                    &mut Err(ref mut val) => {
                        return Err(val.clone());
                    }
                }
            }

            is_tag_begin_found = false;
            is_tag_end_found = false;
            tag_content = "".to_string();
        }

        result
    }

    fn parse_tag(tag: String) -> Result<XmlItemRc, String> {

        // TDOD: add Regex from crates.io as a build option.
        if tag.find("<").is_none() || tag.rfind(">").is_none() {
            return Err(format!("malformed tag \"{}\"", tag));
        }

        let tag = tag.replace("\n", " ").trim().to_string();
        let mut tag_begin = tag.find("<").unwrap() + 1;
        if let Some(_) = tag.find("</") {
            tag_begin += 1;
        }

        let mut tag_end = tag.rfind(">").unwrap() - 1;
        if let Some(_) = tag.rfind("/>") {
            tag_end -= 1;
        }

        let tag = substr_any(&tag, &tag_begin, &tag_end).trim().to_string();

        let mut tag_name_end = tag.len() - 1;
        if let Some(pos) = tag.find(" ") {
            tag_name_end = pos;
        }

        let tag_name = substr_any(&tag, &0, &tag_name_end).trim().to_string();
        let tag_attributes = substr_any(&tag, &(tag_name_end + 1), &(tag.len() - 1))
                                 .trim()
                                 .to_string();

        if tag_name.is_empty() {
            return Err(format!("Can't parse tag \"<{}/>\"", tag));
        }
        let result = XmlItem::new(tag_name);

        if !tag_attributes.is_empty() {
            Self::parse_attributes(result.clone(), tag_attributes);
        }

        Ok(result.clone())
    }

    fn parse_attributes(rc: XmlItemRc, attributes: String) {

        let mut attributes = attributes.trim().to_string();
        if attributes.is_empty() {
            return;
        }


        let mut eq_pos = 0;
        if let Some(pos) = attributes.find("=") {
            if pos == 0 {
                return;
            }
            eq_pos = pos;
        }
        let name = substr_any(&attributes, &0, &(eq_pos - 1)).trim().to_string();
        if attributes.find("\"").is_none() {
            return;
        }
        attributes = substr_any(&attributes,
                                &(attributes.find("\"").unwrap() + 1),
                                &(attributes.len() - 1))
                         .trim()
                         .to_string();

        let mut is_value_end_found = false;
        let mut val_end = 0;
        {
            let mut iter = attributes.match_indices("\"").filter(|ch| {
                let val = substr_any(&attributes, &(ch.0 - 1), &ch.0);
                !val.eq("\\\"")
            });
            while let Some(ch) = iter.next() {
                is_value_end_found = true;
                val_end = ch.0;
                break;
            }
        }

        if is_value_end_found {
            XmlItem::set_attribute(rc.clone(),
                                   name,
                                   substr_any(&attributes, &0, &(val_end - 1)));
        } else {
            return;
        }
        attributes = substr_any(&attributes, &(val_end + 1), &(attributes.len() - 1))
                         .trim()
                         .to_string();
        if !attributes.is_empty() {
            Self::parse_attributes(rc.clone(), attributes);
        }
    }

    pub fn save_file(rc: XmlItemRc, filename: &Path) -> Result<(), Error> {

        let mut file = try!(File::create(filename.as_os_str()));
        try!(file.write_all(XmlItem::as_string(rc.clone()).as_bytes()));
        try!(file.sync_all());
        Ok(())
    }
}

#[allow(dead_code)]
pub fn substr_any(string: &String, begin: &usize, end: &usize) -> String {
    if begin > end {
        return "".to_string();
    }
    match string.char_indices().nth(*begin) {
        Some((begin_pos, _)) => {
            match string.char_indices().nth(*end) {
                Some((end_pos, _)) => (&string[begin_pos..end_pos + 1]).to_string(),
                None => "".to_string(),
            }
        }
        None => "".to_string(),
    }
}

#[allow(dead_code)]
pub fn substr_try(string: &String, begin: &usize, end: &usize) -> Result<String, &'static str> {
    if begin > end {
        return Err("invalid begin > end");
    }
    match string.char_indices().nth(*begin) {
        Some((begin_pos, _)) => {
            match string.char_indices().nth(*end) {
                Some((end_pos, _)) => Ok((&string[begin_pos..end_pos + 1]).to_string()),
                None => Err("invalid end index"),
            }
        }
        None => Err("invalid begin index"),
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use xml_item::{XmlItem, XmlItemRc};
    #[allow(unused_imports)]
    use XmlDom;

    #[test]
    fn substr_any() {
        assert_eq!("bcd".to_string(),
                   super::substr_any(&"abcde".to_string(), &1, &3));
        assert_eq!("c".to_string(),
                   super::substr_any(&"abcde".to_string(), &2, &2));
        assert_eq!("".to_string(),
                   super::substr_any(&"abcde".to_string(), &3, &1));
        assert_eq!("".to_string(),
                   super::substr_any(&"abcde".to_string(), &0, &42));
        assert_eq!("".to_string(),
                   super::substr_any(&"abcde".to_string(), &42, &100));
    }

    #[test]
    fn substr_try() {
        assert_eq!(Ok("bcd".to_string()),
                   super::substr_try(&"abcde".to_string(), &1, &3));
        assert_eq!(Ok("c".to_string()),
                   super::substr_try(&"abcde".to_string(), &2, &2));
        assert_eq!(true,
                   super::substr_try(&"abcde".to_string(), &3, &1).is_err());
        assert_eq!(true,
                   super::substr_try(&"abcde".to_string(), &0, &42).is_err());
        assert_eq!(true,
                   super::substr_try(&"abcde".to_string(), &42, &100).is_err());
    }

}
