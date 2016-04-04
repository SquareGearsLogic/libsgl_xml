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
//

//! Library represents XML as a browsable DOM
//!
//! Examples
//!
//! ```
//! extern crate libsgl_xml;
//!
//! use std::path::Path;
//! 
//! use libsgl_xml::{XmlItem, XmlItemRc};
//! use libsgl_xml::XmlDom;
//!
//! fn main() {
//!     // Load from xml file
//!     match XmlDom::open(Path::new("./tests/test.xml")) {
//!         Ok(root) => {
//!             // Print it
//!             println!("test.xml starts with \"{}\" element :\n{}\n----------",
//!                      XmlItem::get_name(root.clone()),
//!                      XmlItem::as_string(root.clone()));
//!
//!             // Get array of Root's sub-nodes and selet first item.
//!             // All nodes, including first one, are Counted References, so simply clone() them everywhere.
//!             let first_node_of_root: XmlItemRc = XmlItem::get_nodes(root.clone())[0].clone();
//!
//!             // Attach another node with attribute to "first_node_of_root"
//!             let new_node = XmlItem::add_node(first_node_of_root.clone(),
//!                                              XmlItem::new("YetAnotherNode".to_string()));
//!             XmlItem::set_attribute(new_node.clone(), "ID".to_string(), "42".to_string());
//!
//!             // Save it result to another xml file
//!             if let Err(val) = XmlDom::save_file(root.clone(), Path::new("./tests/result.xml")) {
//!                 // Support error messages
//!                 println!("Error: \"{}\"", val);
//!             } else {
//!                 println!("Saved result.xml :\n{}\n----------",
//!                          XmlItem::as_string(root.clone()));
//!             }
//!
//!             // You may clean memory manually
//!             XmlItem::clean(root.clone());
//!         }
//!         Err(val) => {
//!             // Support error messages
//!             println!("Error: \"{}\"", val);
//!         }
//!     };
//! }
//! ```
//!
//! Output:
//!
//! ```xml
//! test.xml starts with "root" element :
//! <root a="b" c="d">
//! 	<node_1.1>
//! 		<node_2.1/>
//! 	</node_1.1>
//! 	<node_1.2>
//! 		<node_2.2/>
//! 	</node_1.2>
//! </root>
//! ----------
//! Saved result.xml :
//! <root a="b" c="d">
//! 	<node_1.1>
//! 		<node_2.1/>
//! 		<YetAnotherNode ID="42"/>
//! 	</node_1.1>
//! 	<node_1.2>
//! 		<node_2.2/>
//! 	</node_1.2>
//! </root>
//! ----------
//! ```
//!
mod xml_item;
mod xml_dom;

pub use self::xml_item::{XmlItem, XmlItemRc};
pub use self::xml_dom::XmlDom;
