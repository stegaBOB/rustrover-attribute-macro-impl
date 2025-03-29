MRE for https://youtrack.jetbrains.com/issue/RUST-17417/Attribute-macros-that-modify-self-in-impl-blocks-break-code-completion

With RustRover, if you have an attribute macro that acts on an impl block and modifies the item being impl'd on, the
code completion for fields and methods suggest on the original item, not the modified one from the macro. Rust-analyzer
currently handles this properly.

In [src/lib.rs](src/lib.rs), the `impl Foo` block actually outputs an impl on Bar due to the `impl_for` attribute macro.
While this attribute macro is very contrived, there are real use cases for something like this. In our case, we have
separate proc macros that generate complex structs that are abstracted away. In order to easily add impl blocks on top,
our attribute macros replace the simple struct the user provides with the actual complex signature under the hood.
