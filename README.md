`cargo-example` is a easy-to-use example runner for cargo projects.

# About

`cargo-example` aims to provide a simple interface to running examples
of any valid rust crate. Currently there is no short way to do this.
In order to run examples you'll always have to manually clone or download
the repository of the crate whose examples you wish to run.
For newcomers to the rust language the `example` feature of cargo
might even remain completely unknown for quite some time.  
This crate aims to simplify this process. All the messy cloning
is abstracted away from the end user.

# Usage

## As a user

TBD. Test if cargo install works!

## In development

Just use `cargo run example ...args` where args are the arguments you wish
to give to the `cargo --example` command.

# Inspiration

I wouldn't have made this project if it wasn't for the excellent GitHub
Project "request-for-implementation" by dtolnay. Specifically
[this](https://github.com/dtolnay/request-for-implementation/issues/30)
issue. 

# Early stages

This entire project is still in its infancy. It might not work for you.
But you can help. Just open up a new GitHub issue if you find an error
or would like to request a feature or a change.