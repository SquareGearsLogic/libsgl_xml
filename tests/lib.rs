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

extern crate libsgl_xml;

use std::path::Path;

use libsgl_xml::XmlItem;
use libsgl_xml::XmlDom;

#[test]
fn open() {
    let mut result = XmlDom::open(Path::new("./tests/test.xml"));
    assert_eq!(result.is_ok(), true);
    match &mut result {
        &mut Ok(ref mut val) => {
            assert_eq!(XmlItem::get_name(val.clone()), "root".to_string());
        }
        &mut Err(_) => assert!(false),
    }
}
