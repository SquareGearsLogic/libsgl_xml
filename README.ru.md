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

use std::path::Path;

use libsgl_xml::{XmlItem, XmlItemRc};
use libsgl_xml::XmlDom;

fn main() {
    // Load from xml file
    match XmlDom::open(Path::new("./tests/test.xml")) {
        Ok(root) => {
            // Распечатка.
            println!("test.xml начинается с \"{}\" элемента :\n{}\n----------",
                     XmlItem::get_name(root.clone()),
                     XmlItem::as_string(root.clone()));

            // Берём массив нодов корневого элемента и выделяем первый из них.
            // Все ноды, включая корневой - подсчитываемые ссылки, 
			// так что для передачи их надо просто clone().
            let first_node_of_root: XmlItemRc = XmlItem::get_nodes(root.clone())[0].clone();

            // Прикрепляем к "first_node_of_root" новый нод с атрибутом.
            let new_node = XmlItem::add_node(first_node_of_root.clone(),
                                             XmlItem::new("YetAnotherNode".to_string()));
            XmlItem::set_attribute(new_node.clone(), "ID".to_string(), "42".to_string());

            // Сохраняем результат в другой XML файл.
            if let Err(val) = XmlDom::save_file(root.clone(), Path::new("./tests/result.xml")) {
                // Поддержка сообщений об ошибках.
                println!("Ошибка: \"{}\"", val);
            } else {
                println!("Результат сохранён в result.xml :\n{}\n----------",
                         XmlItem::as_string(root.clone()));
            }

            // Можно почистить память вручную.
            XmlItem::clean(root.clone());
        }
        Err(val) => {
            // Поддержка сообщений об ошибках.
            println!("Ошибка: \"{}\"", val);
        }
    };
}
```
Вывод:
```xml
test.xml начинается с "root" элемента :
<root a="b" c="d">
	<node_1.1>
		<node_2.1/>
	</node_1.1>
	<node_1.2>
		<node_2.2/>
	</node_1.2>
</root>
----------
Результат сохранён в result.xml :
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