libsgl_xml
======
[![Read in English](http://www.printableworldflags.com/icon-flags/24/United%20Kingdom.png)](https://github.com/SquareGearsLogic/libsgl_xml) [![Read in Russian](http://www.printableworldflags.com/icon-flags/24/Russian%20Federation.png)](https://github.com/SquareGearsLogic/libsgl_xml/blob/master/README.ru.md)  
![#](https://travis-ci.org/SquareGearsLogic/libsgl_xml.svg?branch=master)

Библиотека Rust представляющая XML в виде просматриваемого DOM

> Логика та же, [что для C++](https://github.com/SquareGearsLogic/SglXml.git)

Сборка
-----------

```
[dependencies]
libsgl_xml = { path = "libsgl_xml", version = "*"}
```

Использование
-----------

```rust
extern crate libsgl_xml;

use libsgl_xml::{XmlItem, XmlItemRc};
use libsgl_xml::XmlDom;

fn main() {
    // Load from xml file
    match XmlDom::open("test.xml".to_string()) {
        Ok(root) => {
            // Print it
            println!("test.xml starts with \"{}\" element :\n{}\n----------",
                     XmlItem::get_name(root.clone()),
                     XmlItem::as_string(root.clone()));

            // Get array of Root's sub-nodes and selet first item.
            // All nodes, including first one, are Counted References, so simply clone() them everywhere.
            let first_node_of_root: XmlItemRc = XmlItem::get_nodes(root.clone())[0].clone();

            // Attach another node with attribute to "first_node_of_root"
            let new_node = XmlItem::add_node(first_node_of_root.clone(),
                                             XmlItem::new("YetAnotherNode".to_string()));
            XmlItem::set_attribute(new_node.clone(), "ID".to_string(), "42".to_string());

            // Save it result to another xml file
            if let Err(val) = XmlDom::save_file(root.clone(), "result.xml".to_string()) {
                // Support error messages
                println!("Error: \"{}\"", val);
            } else {
                println!("Saved result.xml :\n{}\n----------",
                         XmlItem::as_string(root.clone()));
            }

            // You may clean memory manually
            XmlItem::clean(root.clone());
        }
        Err(val) => {
            // Support error messages
            println!("Error: \"{}\"", val);
        }
    };
}
```
Output:
```xml
test.xml starts with "root" element :
<root a="b" c="d">
	<node_1.1>
		<node_2.1/>
	</node_1.1>
	<node_1.2>
		<node_2.2/>
	</node_1.2>
</root>
----------
Saved result.xml :
<root a="b" c="d">
	<node_1.1>
		<node_2.1/>
		<YetAnotherNode ID="42"/>
	</node_1.1>
	<node_1.2>
		<node_2.2/>
	</node_1.2>
</root>
----------
```

Лицензия
-----------
Apache License Version 2.0, January 2004
http://www.apache.org/licenses/