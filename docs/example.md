# HOW TO FUCKING CREATE A MOTHERFUCKING COMMAND AND WHAT THE FUCK ARE WE FUCKING DOING

Hello young padawan, the following document is a small tutorial on what why and how.


## .env
as you already know, we need a `.env`

create a new file in the same level as the `.env.example`

call it `.env`

copy paste the text inside

this in particular

```.env
TOKEN=
PREFIX="!"
```

add the token in there, you can also change the prefix if you dont like that one

## main.rs

this is a shit hole, this is me on one afternoon of programming something that has horrible documentation.
you dont have to understand a lot from here, this is just what makes the bot run.

things important from here.


```rust
8  mod commands;
9  use crate::commands::dice::*;
10 use crate::commands::example::*;
```

when you create a new command (ill explain later in this doc) you have to import that command here:
1. `use crate` means that we are using a Module
2. `commands` is the module that we are using
3. `dice` or `example` is the modules we want to use
4. `*` means that it will bring everything from the module

why is me explaing this?

so you can make your own basic command iwill elaborate further but there is one small thing we need to address

*adding the command*

```rust
38 let commands = vec![roll(), example_command()];
```

this is the line were we will add the commands, as your eyes can see

this are functions, cause when we say `roll()` it means that is a function

what the fuck is a function?

nice question

a function is a set of procedures that can handle input and output

what the fuck is input and output?

nice question again

input means that we can add values to de function, in the case of roll

even if you dont see that it gets values like `r o l l ( " 2 d 20 " )` it actually needs a value to work

that is something the bot works out for you, when we see the bot you will understand this better

output means that it will return something, in this case `roll()` returns a message

now, how do we add a command there?

let's say you want to add a command called `rasoon` and it will display all the social links of rasoon

in here, we will first

```rust
8  mod commands;
9  use crate::commands::dice::*;
10 use crate::commands::example::*;
11 use crate::commands::social_links::rasoon; <- Here is the new command
```
and then
```rust
38 let commands = vec![roll(), example_command(), rasoon()]; <- Here is the new command
```

that's all you need to do to add a command

you got this

## social_links.rs
this is the ultimate test

this is what you have trained for


--WIP--
