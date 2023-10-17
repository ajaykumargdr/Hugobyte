# **Creating and Invoking OpenWhisk Action Written in Rust**

The following sections will guide you through the process of creating and invoking a single Rust action. It will also demonstrate how to bundle multiple Rust files and their dependencies.

## Dependencies

- **OpenWhisk:** OpenWhisk is a platform for building cloud applications using serverless functions. It provides a rich programming model for creating serverless APIs from functions, composing functions into serverless workflows, and connecting events to functions using rules and triggers.

- **OpenWhisk CLI:** The OpenWhisk Command-line Interface (CLI) is a unified tool that allows users to interact with OpenWhisk services consistently. For more information, visit [OpenWhisk-cli](https://github.com/apache/Openwhisk-cli#running-the-wsk-cli).

## hello-person-example-OpenWhisk-action

This is an example code for an OpenWhisk action written in Rust. Rust actions are mainly composed of a `main` function that accepts a JSON `serde Value` as input and returns a `Result` that includes a JSON `serde Value`. In this example, we will pass a `serde Value` containing the data `"name: <any-name>"` in JSON format. If the value contains a name, this action will write `"Hello! <given-name>"`. Otherwise, it will return a `serde Error` containing the error message `"person not found!"`.

## How to test?

If you want to test the `hello-person-example-OpenWhisk-action` code before deploying it, you can use the following command to test it

```bash
cargo test
```

## Deploy the action

This action requires external dependencies, so we need to provide a zip file that includes our source code and the Cargo file containing all these dependencies. The source file that contains the entry point `main(args: Value)` must be named `lib.rs`.

**Create a zip file using the following command**
```bash
zip -r helloPerson.zip Cargo.toml src
```

**Deploy the action using the following OpenWhisk CLI command**
```bash
wsk action create helloPerson --kind rust:1.34 helloPerson.zip
```

## Invoke the action

invoke the action using the following command
```bash
wsk action invoke --result helloPerson --param name <any-name>
```