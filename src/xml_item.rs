pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use std::collections::HashMap;

pub type XmlItemRc = Rc<RefCell<Option<XmlItem>>>;

pub struct XmlItem {
    pub name: String,
    pub attributes: HashMap<String, String>,

    // Parent and child are None by default.
    pub children: Vec<XmlItemRc>,
    pub parent: XmlItemRc,
}

impl XmlItem {
    pub fn new(name: String) -> XmlItemRc {
        Rc::new(RefCell::new(Some(XmlItem {
            parent: Self::get_empty_item(),
            children: vec![],
            name: name,
            attributes: HashMap::new(),
        })))
    }

    // Adds child to a given parent and given parent to a new child, so both know each other.
    pub fn add_child(parent: XmlItemRc, child: XmlItemRc) -> XmlItemRc {

        if let &mut Some(ref mut xml_item) = &mut *child.borrow_mut() {
            xml_item.parent = parent.clone();
        }

        if let &mut Some(ref mut xml_item) = &mut *parent.borrow_mut() {
            xml_item.children.push(child.clone());
        }

        child
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

    pub fn get_child(rc: XmlItemRc) -> Vec<XmlItemRc> {
        match &mut *rc.borrow_mut() {
            &mut Some(ref mut xml_item) => {
                // TODO: How vector of Rc behaves?
                let mut result: Vec<XmlItemRc> = vec![];
                for child in &xml_item.children {
                    result.push(child.clone());
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

    pub fn get_empty_item() -> XmlItemRc {
        Rc::new(RefCell::new(None))
    }

    fn delete_children(&mut self) {
        for child in &self.children {
            if let &mut Some(ref mut xml_item) = &mut *child.borrow_mut() {
                xml_item.delete_children();
            }
        }
        self.children = vec![];
    }

    pub fn clean(rc: XmlItemRc) {
        if let &mut Some(ref mut test) = &mut *rc.borrow_mut() {
            test.delete_children();
        }
    }

    fn to_string(&mut self, offset: usize) -> String {
        let mut result = format!("{}<{}", Self::get_tabs(&offset), self.name.clone());
        for (key, value) in &self.attributes {
            result = format!("{} {}=\"{}\"", result, key, value);
        }
        if self.children.is_empty() {
            result = format!("{}/>", result);
            return result;
        }
        result = format!("{}>", result);

        for child in &self.children {
            if let &mut Some(ref mut xml_item) = &mut *child.borrow_mut() {
                let child_result = xml_item.to_string(offset + 1);
                result = format!("{}\n{}", result, child_result);
            }
        }

        result = format!("{}\n{}</{}>", result, Self::get_tabs(&offset), self.name.clone());
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
impl Drop for XmlItem {
    fn drop(&mut self) {
        println!("{} is dropped", self.name);
    }
}

/// //////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use {XmlItem, XmlItemRc};

    #[test]
    fn cascade_destruction() {
        let chain_one: XmlItemRc = XmlItem::new("One".to_string());
        {
            let chain_two = XmlItem::add_child(chain_one.clone(), "Two".to_string());
            XmlItem::add_child(chain_two.clone(), "Three".to_string());
        }
        XmlItem::clean(chain_one.clone());

        let child: XmlItemRc = XmlItem::get_child(chain_one.clone());
        assert_eq!(child.borrow_mut().is_some(), false);	// One sholdn't have children after clean();
    }

    #[test]
    fn children() {
        let chain_one: XmlItemRc = XmlItem::new("One".to_string());
        {
            let chain_two = XmlItem::add_child(chain_one.clone(), "Two".to_string());
            XmlItem::add_child(chain_two.clone(), "Three".to_string());
        }
        {
            assert_eq!(XmlItem::get_name(chain_one.clone()), "One".to_string());

            let chain_two = XmlItem::get_child(chain_one.clone());
            assert_eq!(chain_two.borrow_mut().is_some(), true); // Does Two exist as One's child?
            assert_eq!(XmlItem::get_name(chain_two.clone()), "Two".to_string());

            let chain_three = XmlItem::get_child(chain_two.clone());
            assert_eq!(chain_three.borrow_mut().is_some(), true); // Does Three exist as Two's child?
            assert_eq!(XmlItem::get_name(chain_three.clone()), "Three".to_string());

            let bastard = XmlItem::get_child(chain_three.clone());
            assert_eq!(bastard.borrow_mut().is_some(), false); // Three shoudn't have a child.
        }
    }

    #[test]
    fn parents() {
        let chain_one: XmlItemRc = XmlItem::new("One".to_string());
        {
            let chain_two = XmlItem::add_child(chain_one.clone(), "Two".to_string());
            XmlItem::add_child(chain_two.clone(), "Three".to_string());
        }
        {
            let god = XmlItem::get_parent(chain_one.clone());
            assert_eq!(god.borrow_mut().is_some(), false); // There is no God.

            let chain_two = XmlItem::get_child(chain_one.clone());
            let first_parent = XmlItem::get_parent(chain_two.clone());
            assert_eq!(first_parent.borrow_mut().is_some(), true); // Does Two knows it's parent?
            assert_eq!(XmlItem::get_name(first_parent.clone()), "One".to_string());

            let chain_three = XmlItem::get_child(chain_two.clone());
            let second_parent = XmlItem::get_parent(chain_three.clone());
            assert_eq!(second_parent.borrow_mut().is_some(), true); // Does Three knows it's parent?
            assert_eq!(XmlItem::get_name(second_parent.clone()), "Two".to_string());

        }
    }

}
