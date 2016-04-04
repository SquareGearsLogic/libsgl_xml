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
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use std::collections::HashMap;

/// Any Element in DOM structure is a Counted Reference to Cell,
/// that contains Option of actual XmlItem.
/// 
/// On practice that means any received DOM Element should be checked for None/Some:
///
/// ```
/// # use libsgl_xml::{XmlItem, XmlItemRc};
/// let new_node: XmlItemRc = XmlItem::new("YetAnotherNode".to_string());
/// if new_node.borrow_mut().is_some() {}
/// ```
///
/// To pass an element somewhere simply use **clone()** method.
/// It will create a tiny coumted reference to location of actual XmlItem data.
///
/// # Examples
///
/// ```
/// # use libsgl_xml::{XmlItem, XmlItemRc};
/// fn foo (node: XmlItemRc) -> XmlItemRc {
///     // do something
///     node.clone()
/// }
///
/// let new_node = XmlItem::new("YetAnotherNode".to_string());
/// if new_node.borrow_mut().is_some() {
///     foo(new_node.clone());
/// }
/// ```
///
pub type XmlItemRc = Rc<RefCell<Option<XmlItem>>>;

/// Represents an Element of XML DOM structure.
///
/// Each Node of DOM structure contains reference to it's parent and an array 
/// of child nodes. Each child node may contain more nodes and so on.
///
/// Generally memory model is handled by [XmlItemRc](../XmlItemRc) structure.
/// Because of XmlItemRc internal complexity moste operations on it done
/// via assosiated function. To access option simply call borrow_mut() on it.
pub struct XmlItem {
    /// Node name
    pub name: String,
    /// Map of node attributes. Position is not guaranteed.
    pub attributes: HashMap<String, String>,
    /// Vector of sub-nodes 
    pub nodes: Vec<XmlItemRc>,
    /// The parent of this node. None assigned for the first item in DOM structure.
    pub parent: XmlItemRc, // None by default.
}

impl XmlItem {
    pub fn new(name: String) -> XmlItemRc {
        Rc::new(RefCell::new(Some(XmlItem {
            parent: Self::get_empty_item(),
            nodes: vec![],
            name: name,
            attributes: HashMap::new(),
        })))
    }

    /// Adds child node to a given parent and given parent to a new child, so both know each other.
    pub fn add_node(parent: XmlItemRc, node: XmlItemRc) -> XmlItemRc {

        if let &mut Some(ref mut xml_item) = &mut *node.borrow_mut() {
            xml_item.parent = parent.clone();
        }

        if let &mut Some(ref mut xml_item) = &mut *parent.borrow_mut() {
            xml_item.nodes.push(node.clone());
        }

        node
    }

    pub fn set_attribute(rc: XmlItemRc, attribute: String, value: String) {
        if let &mut Some(ref mut test) = &mut *rc.borrow_mut() {
            test.attributes.insert(attribute, value);
        }
    }

    pub fn get_name(rc: XmlItemRc) -> String {
        match &mut *rc.borrow_mut() {
            &mut Some(ref mut xml_item) => {
                return xml_item.name.clone();
            }
            _ => {
                return "".to_string();
            }
        };
    }

    pub fn get_nodes(rc: XmlItemRc) -> Vec<XmlItemRc> {
        match &mut *rc.borrow_mut() {
            &mut Some(ref mut xml_item) => {
                // TODO: How vector of Rc behaves?
                let mut result: Vec<XmlItemRc> = vec![];
                for node in &xml_item.nodes {
                    result.push(node.clone());
                }
                return result;
            }
            _ => {
                return vec![];
            }
        };
    }

    pub fn get_parent(rc: XmlItemRc) -> XmlItemRc {
        match &mut *rc.borrow_mut() {
            &mut Some(ref mut xml_item) => {
                return xml_item.parent.clone();
            }
            _ => {
                return Self::get_empty_item();
            }
        };
    }

    /// Creates a new unbound None.
    pub fn get_empty_item() -> XmlItemRc {
        Rc::new(RefCell::new(None))
    }

    fn delete_nodes(&mut self) {
        for node in &self.nodes {
            if let &mut Some(ref mut xml_item) = &mut *node.borrow_mut() {
                xml_item.delete_nodes();
            }
        }
        self.nodes = vec![];
    }

    /// Deletes all children of this node.
    pub fn clean(rc: XmlItemRc) {
        if let &mut Some(ref mut test) = &mut *rc.borrow_mut() {
            test.delete_nodes();
        }
    }

    fn to_string(&mut self, offset: usize) -> String {
        let mut result = format!("{}<{}", Self::get_tabs(&offset), self.name.clone());
        for (key, value) in &self.attributes {
            result = format!("{} {}=\"{}\"", result, key, value);
        }
        if self.nodes.is_empty() {
            result = format!("{}/>", result);
            return result;
        }
        result = format!("{}>", result);

        for node in &self.nodes {
            if let &mut Some(ref mut xml_item) = &mut *node.borrow_mut() {
                let node_result = xml_item.to_string(offset + 1);
                result = format!("{}\n{}", result, node_result);
            }
        }

        result = format!("{}\n{}</{}>",
                         result,
                         Self::get_tabs(&offset),
                         self.name.clone());
        return result;
    }

    #[allow(unused_variables)]
    fn get_tabs(offset: &usize) -> String {
        let mut result = "".to_string();
        for x in 0..*offset {
            result = format!("{}{}", result, "\t");
        }
        result
    }

    /// Prints item and it's child nodes into a String in a human-readable form.
    pub fn as_string(rc: XmlItemRc) -> String {
        match &mut *rc.borrow_mut() {
            &mut Some(ref mut xml_item) => {
                return xml_item.to_string(0);
            }
            _ => {
                return "".to_string();
            }
        };
    }
}

// TODO: Remove it. This is done generalluy for test purpose.
//
// impl Drop for XmlItem {
// fn drop(&mut self) {
// println!("{} is dropped", self.name);
// }
// }
//



#[cfg(test)]
mod test {
    use {XmlItem, XmlItemRc};

    #[test]
    fn cascade_destruction() {
        let root: XmlItemRc = XmlItem::new("Root".to_string());
        {
            let node_1 = XmlItem::add_node(root.clone(), XmlItem::new("Root's One".to_string()));
            XmlItem::add_node(node_1.clone(), XmlItem::new("One's Two".to_string()));
        }
        XmlItem::clean(root.clone());

        let nodes: Vec<XmlItemRc> = XmlItem::get_nodes(root.clone());
        assert_eq!(nodes.len(), 0);	// Root sholdn't have children after clean();
    }

    #[test]
    fn nodes() {
        let root: XmlItemRc = XmlItem::new("Root".to_string());
        {
            let node_1 = XmlItem::add_node(root.clone(), XmlItem::new("Root's One".to_string()));
            XmlItem::add_node(node_1.clone(), XmlItem::new("One's Two".to_string()));
        }
        {
            assert_eq!(XmlItem::get_name(root.clone()), "Root".to_string());

            let roots_nodes = XmlItem::get_nodes(root.clone());
            assert_eq!(roots_nodes.len(), 1);

            assert_eq!(roots_nodes[0].borrow_mut().is_some(), true); // Does One exist as Root's child?
            assert_eq!(XmlItem::get_name(roots_nodes[0].clone()),
                       "Root's One".to_string());

            let ones_nodes = XmlItem::get_nodes(roots_nodes[0].clone());
            assert_eq!(ones_nodes.len(), 1);
            assert_eq!(ones_nodes[0].borrow_mut().is_some(), true); // Does Two exist as One's child?
            assert_eq!(XmlItem::get_name(ones_nodes[0].clone()),
                       "One's Two".to_string());

            let twos_bastards = XmlItem::get_nodes(ones_nodes[0].clone());
            assert_eq!(twos_bastards.len(), 0); // Two shoudn't have a child.
        }
    }

    #[test]
    fn parents() {
        let root: XmlItemRc = XmlItem::new("Root".to_string());
        let node_1 = XmlItem::add_node(root.clone(), XmlItem::new("Root's One".to_string()));
        let node_2 = XmlItem::add_node(node_1.clone(), XmlItem::new("One's Two".to_string()));

        assert_eq!(XmlItem::get_name(node_2.clone()), "One's Two".to_string());

        let node_2_parent = XmlItem::get_parent(node_2.clone());
        assert_eq!(XmlItem::get_name(node_2_parent.clone()),
                   "Root's One".to_string());

        let node_1_parent = XmlItem::get_parent(node_2_parent.clone());
        assert_eq!(XmlItem::get_name(node_1_parent.clone()), "Root".to_string());

        let god = XmlItem::get_parent(node_1_parent.clone());
        assert_eq!(god.borrow_mut().is_none(), true); // There is no God.
    }

}
