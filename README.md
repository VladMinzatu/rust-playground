# rust-playground

Small, self-contained Rust examples, one concept per file in `./examples`.
Files are numbered `CCNN_topic.rs` (chapter, then example) so they sort in learning order.

## Running an example

In VSCode, open the example file and press **Cmd+Shift+B** (runs the default task in `.vscode/tasks.json`),
or click the **▶ Run** link that rust-analyzer shows above `fn main()`.

From a terminal:
```
cargo run --example 0605_trait_objects_and_dynamic_dispatch
```
