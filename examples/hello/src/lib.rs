// Copyright (c) 2019 jmjoy
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{
    arrays::ZArray,
    classes::{DynamicClass, Visibility},
    functions::Argument,
    ini::{Ini, Policy},
    modules::{Module, ModuleContext},
    objects::Object,
    php_get_module,
    values::ZVal,
};

fn module_init(_args: ModuleContext) -> bool {
    true
}

fn say_hello(arguments: &mut [ZVal]) -> phper::Result<String> {
    let name = arguments[0].as_string_value()?;
    Ok(format!("Hello, {}!\n", name))
}

fn throw_exception(_: &mut [ZVal]) -> phper::Result<()> {
    Err(phper::Error::other("I am sorry"))
}

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // register module ini
    Ini::add("hello.enable", false, Policy::All);
    Ini::add("hello.num", 100, Policy::All);
    Ini::add("hello.ratio", 1.5, Policy::All);
    Ini::add("hello.description", "hello world.".to_owned(), Policy::All);

    // register hook functions
    module.on_module_init(module_init);
    module.on_module_shutdown(|_| true);
    module.on_request_init(|_| true);
    module.on_request_shutdown(|_| true);

    // register functions
    module.add_function("hello_say_hello", say_hello, vec![Argument::by_val("name")]);
    module.add_function("hello_throw_exception", throw_exception, vec![]);
    module.add_function(
        "hello_get_all_ini",
        |_: &mut [ZVal]| {
            let mut arr = ZArray::new();

            let hello_enable = ZVal::from(Ini::get::<bool>("hello.enable"));
            arr.insert("hello.enable", hello_enable);

            let hello_description = ZVal::from(Ini::get::<String>("hello.description"));
            arr.insert("hello.description", hello_description);

            arr
        },
        vec![],
    );

    // register classes
    let mut foo_class = DynamicClass::new("FooClass");
    foo_class.add_property("foo", Visibility::Private, 100);
    foo_class.add_method(
        "getFoo",
        Visibility::Public,
        |this: &mut Object<()>, _: &mut [ZVal]| {
            let prop = this.get_property("foo");
            Ok::<_, phper::Error>(prop.as_string_value()?)
        },
        vec![],
    );
    foo_class.add_method(
        "setFoo",
        Visibility::Public,
        |this: &mut Object<()>, arguments: &mut [ZVal]| -> phper::Result<()> {
            this.set_property("foo", ZVal::from(arguments[0].as_string_value()?));
            Ok(())
        },
        vec![Argument::by_val("foo")],
    );
    module.add_class(foo_class);

    module
}
