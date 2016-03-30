libsgl_xml
======
[![Read in English](http://www.printableworldflags.com/icon-flags/24/United%20Kingdom.png)](https://github.com/SquareGearsLogic/libsgl_xml) [![Read in Russian](http://www.printableworldflags.com/icon-flags/24/Russian%20Federation.png)](https://github.com/SquareGearsLogic/libsgl_xml/blob/master/README.ru.md)
![#](https://travis-ci.org/SquareGearsLogic/libsgl_xml.svg?branch=master)

Rust library that represents XML as browsable DOM

> It has same logic, as [C++ version](https://github.com/SquareGearsLogic/SglXml.git)

Building
-----------

```
[dependencies]
libsgl_xml = { path = "libsgl_xml", version = "*"}
```

Using
-----------

```rust
extern crate libsgl_xml;
use libsgl_xml::XmlItem;
use libsgl_xml::XmlDom;

fn main() {
	// Load from xml file
    let mut result = XmlDom::open("test.xml".to_string());

    match &mut result {
        &mut Ok(ref mut root) => {
			// Print it
			println!("{}", XmlItem::as_string(root.clone()));
			// Add another child node
			{
				let mut child = XmlItem::add_child(root.clone(), "Child".to_string());
				XmlItem::set_attribute(child.clone(), "ID".to_string(), "42".to_string()
			}
			// Save it back to xml
			XmlDom::save_file(root.clone(), "result.xml".to_string());
			// Clean up
			XmlItem::clean(root.clone());
        }
        &mut Err(ref mut val) => {
			// Support error messages
            println!("Error is \"{}\"", val);
        }
    };
}
```

License
-----------
Apache License Version 2.0, January 2004
http://www.apache.org/licenses/