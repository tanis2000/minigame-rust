# To set an Option field to None

```
#[derive(Debug)]
struct MyStruct {
    field: Option<u8>
}

fn main() {
    let mut my_struct = MyStruct { field: Some(1) };
    my_struct.field = None;
    // or: my_struct.field.take();
    
    println!("{:?}", my_struct)
}
```

```
#[derive(Debug)]
struct MyStruct {
    field: Option<u8>
}

impl MyStruct {
    fn set_field_to_none(&mut self) {
        self.field = None;
    }
}

fn main() {
    let mut my_struct = MyStruct { field: Some(1) };
    my_struct.set_field_to_none();    
    println!("{:?}", my_struct)
}
```

- Rc - multiple owners
- Cell - mutation even if it's immutable borrow
- Option - to value or not to value