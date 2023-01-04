# has_fields

A macro to check whether some fields are `Some` or not    

## Usage

```rust
use has_fields::has_fields;

struct MyForm {
    id: Option<u32>,
    name: Option<String>,
    email: Option<String>
}

fn main() {
    // Parse your form here
    let form = MyForm {
        id: None,
        name: Some("name".to_string()),
        email: Some("email@example.com".to_string())
    }
    
    // Validate it
    if let Err(missing_fields) = has_fields(form, "name", "email") {
        println!("Missing fields: {:?}", missing_fields);
    } else {
        println!("Validation Successful");
    }
}
```

