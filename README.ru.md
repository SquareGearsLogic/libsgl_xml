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
use libsgl_xml::XmlItem;
use libsgl_xml::XmlDom;

fn main() {
	// Читаем xml файл
    let mut result = XmlDom::open("test.xml".to_string());

    match &mut result {
        &mut Ok(ref mut root) => {
			// Распечатка
			println!("{}", XmlItem::as_string(root.clone()));
			// Добавим новый элемент
			{
				let mut child = XmlItem::add_child(root.clone(), "Child".to_string());
				XmlItem::set_attribute(child.clone(), "ID".to_string(), "42".to_string()
			}
			// Сохраняем обратно в xml
			XmlDom::save_file(root.clone(), "result.xml".to_string());
			// Чистим
			XmlItem::clean(root.clone());
        }
        &mut Err(ref mut val) => {
			// Поддержка сообщений об ошибках
            println!("Error is \"{}\"", val);
        }
    };
}
```

Лицензия
-----------
Apache License Version 2.0, January 2004
http://www.apache.org/licenses/