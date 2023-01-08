# has_fields

Some macros helpful for processing forms with optional fields.

## Usage

For instance, if you got a form like this:
```rust
let form = MyForm {
    id: 1,
    name: Some("name".to_string()),
    email: Some("email@example.com".to_string()),
    phone: None,
}
```

Here are some macros that might help you:

1. `has_fields::has_fields!`: Check if a struct has some fields. Returns a boolean.

    ```rust
    has_fields!(form, "name", "email") // true
    ```

2. `has_fields::require_fields`: Check if a struct has some fields. Returns a `Result<(), Vec<&'static str>>`.

    ```rust
    require_fields!(form, "name", "email") // Ok(())
    require_fields!(form, "name", "email", "phone") // Err(vec!["phone"])
    ```

Moreover, you can derive `HasFields` trait for your struct, and use these methods:

1. `num_fields`: Get the number of `Some(...)` or non-optional fields in a struct.

    ```rust
    form.num_fields() // 2
    ```

## License

The Unlicense

## Contributing

If you have any ideas, feel free to open an issue or a PR.