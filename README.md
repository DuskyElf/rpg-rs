# Terminal Based Rpg Game (Engine)
### In it's early stages of development

### Quick Start
```shell
$ git clone git@github.com:DuskyElf/rpg-rs.git
$ cd rpg-rs
$ git checkout tags/v0.0.1-alpha
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

## Rpg lang Reference

### Messages
Currently, it clears the terminal and animates the message as it being typed, then stops for the user to proceed.
```
"Example message"
```
![image](https://user-images.githubusercontent.com/91879372/235681566-37732814-5ccd-48c9-941b-7da36991492b.png)

### Questions
Currently, it clears the terminal and animtes the question as it being typed, then stops for the user to type in the answer till a newline. Then saves the answer in the variable index provided (`0` in the following example).
```
?0 "What's your name?"
```
![image](https://user-images.githubusercontent.com/91879372/235684202-d37e6c12-1f52-4921-b28a-4d1f2585ee80.png)

### Variables
As stated above, [questions](#questions) save the answer in the variable index provided. Those values could be accessed via the index as `$<index>` inside a quotes.
```
?0 "What's your name?"
"Hi $0!"
```
![image](https://user-images.githubusercontent.com/91879372/235685837-661fe884-c7a5-4dea-91cf-41f4d0aa942c.png)
|
![image](https://user-images.githubusercontent.com/91879372/235686117-244a41f1-2710-42b0-b241-77cfd76bfd3b.png)

### Branches
Currently, it's able to ask a question then show the possible options to select, on the basis of which it branches the code flow.
```
"Select an option -" {
    "First" => {
        "You selected the first branch!"
    }
    "Second" => {
        "You selected the second branch!"
    }
}
```
![image](https://user-images.githubusercontent.com/91879372/235689591-1f79e7f5-7e13-41cc-8200-970bbd06be32.png)

