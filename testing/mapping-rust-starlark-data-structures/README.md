# Mapping Rust Starlark Data Structures

This repository documents the process of mapping Rust and Starlark data structures using the Rust Starlark crate. It presents two main approaches for initializing Rust data structures within Starlark.

## Approach 1: Using Rust functions to initialize a Rust data structure

This approach involves exposing the necessary functions from the Rust side to create and interact with the struct objects in Starlark. The following data structures are discussed:

- `Flow` struct: Represents a flow in the workflow, with properties `hook_type`, `flow_type`, `task_name`, and `depend_on`. Each `Flow` instance represents a single step in the workflow.
- `WorkFlow` struct: Represents the overall workflow. It contains a `RefCell<Vec<Flow>>` which stores a collection of `Flow` instances. It also implements an `add` method to add new flows to the collection.

The `starlark_workflow` function is a Starlark module that defines the `create_flow` function. This function takes parameters for `hook_type`, `flow_type`, `task_name`, and `depend_on`, and adds a new `Flow` to the workflow store.

## Approach 2: Creating new objects in Rust and injecting them into Starlark

The second approach for mapping Rust and Starlark data structures involves creating a new object of the struct in Rust and then injecting it into Starlark. However, it is important to note that directly creating an object for a Rust data structure from within Starlark, without using a constructor function, is nearly impossible. This limitation arises from the design differences between Rust and Starlark.

Starlark, as a restricted language focused on simplicity and safety, does not support direct interaction with arbitrary data types like Rust structs. Additionally, Rust's strict control over object lifetimes and memory safety makes it challenging for other languages, such as Starlark, to directly manipulate Rust objects.

As a result, the conventionally accepted approach is to expose the necessary functions from the Rust side, enabling the creation and interaction with these `struct` objects in Starlark.