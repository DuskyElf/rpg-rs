# Terminal Based Rpg Game (Engine)
### In early stages of development

### Quick Start
```shell
$ git clone git@github.com:DuskyElf/rpg-rs.git
$ cd rpg-rs
```

### Sample Program
```
"Hello, World! This is just so awesome!!!"
?0 "What is your name?"
"Which Branch?" {
    "first" => {
        ?1 "What is your age?"
        "Hi $0, you are $1 years old!"
    }

    "second" => {
        "Are you 18+ $0?" {
            "yes" => { "Whooo, you can vote" }
            "no" => { "Sorry, you can't vote" }
        }
    }
}
```

### Quick Run
```shell
$ cargo r --release test.rpg
```

### Features that are currently implemented
- Variables
- Asking question into a variable (identifier)
- Branching System
