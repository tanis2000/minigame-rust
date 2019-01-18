# Collection of useful snippets

## To set an Option field to None

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

## Difference between optional types

- Rc - multiple owners
- Cell - mutation even if it's immutable borrow
- Option - to value or not to value


## ECS-like from magnusi

```
#![allow(dead_code)]
use std::rc::Rc;

struct Scene
{
    entities: EntityList
}

impl Scene
{
    fn new() -> Self
    {
        Scene
        {
            entities: EntityList::new()
        }
    }
}

struct EntityList
{
    list: Vec<Entity>
}

impl EntityList
{
    fn new() -> Self
    {
        EntityList
        {
            list: Vec::new()
        }
    }
}

struct Entity
{
    components: ComponentList
}

impl Entity
{
    fn new() -> Self
    {
        Entity
        {
            components: ComponentList::new()
        }
    }
}

struct ComponentList
{
    list: Vec<Box<Component>>
}

impl ComponentList
{
    pub fn new() -> Self
    {
        ComponentList
        {
            list: Vec::new()
        }
    }
}

trait Component
{
    fn draw(&self);
    fn update(&self);
    
    //these supply common fields
    fn name(&self) -> &str;
    fn parent(&self) -> &Option<Entity>;
    fn size(&self) -> usize;
}

struct BaseComponent
{
    name: String,
    size: usize,
    parent: Rc<Option<Entity>>,
}

impl BaseComponent
{
    fn new() -> Self
    {
        BaseComponent
        {
            name: String::new(),
            size: 0,
            parent: Rc::new(None), //you would supply this in another function, perhaps use a builder pattern
        }
    }
}

impl Component for BaseComponent
{
    fn draw(&self) {}
    fn update(&self) {}
    
    fn name(&self) -> &str { self.name.as_ref() }
    fn parent(&self) -> &Option<Entity> { &*self.parent }
    fn size(&self) -> usize { self.size }
}

fn main(){}
```
