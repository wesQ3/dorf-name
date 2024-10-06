# dorf-name
_Generate NPC names from Dwarf Fotress language files_

# Synopsis
```
$ dorf-name -c 10
Id Invisiblelabored
Ustir Bodicewhip
Zat Heavencombating
Tosid Wallanimal
Geshud Mobcave
Teling Princessflier
Ustuth Pillarfocused
Togal Skinharmonized
Gusil Vaultdrilling
```

`dorf-name` reads the Dwarf Fortress language files and generates a character name. Use it for your tabletop game or to make fun server names! You can also look up the properties of a DF word.

```
dorf-name -w earth
Word {
    root: "EARTH",
    noun: Some(
        Noun {
            singular: "earth",
            plural: "earths",
            usages: [
                FrontCompoundNounSing,
                RearCompoundNounSing,
                TheCompoundNounSing,
                TheNounSing,
                OfNounSing,
                RearCompoundNounPlur,
            ],
        },
    ),
    verb: None,
    adj: Some(
        Adjective {
            adj: "earthen",
            usages: [
                AdjDist2,
                FrontCompoundAdj,
                RearCompoundAdj,
                TheCompoundAdj,
            ],
        },
    ),
    prefix: None,
    translations: {
        "DWARF": "ber",
        "HUMAN": "etru",
        "GOBLIN": "usluk",
        "ELF": "saba",
    },
    symbols: [
        "NATURE",
        "EARTH",
    ],
}
```

# Usage
```
$ dorf-name -h
Generate character names from DF language files

Usage: dorf-name [OPTIONS]

Options:
  -c, --count <INT>  How many names to generate
  -w, --word <STR>   Dump a word structure
  -h, --help         Print help
  -V, --version      Print version
```

# About
This is my first Rust program and I made it primarily to learn. _Nish nisûn!_

⛏️ _Abod ber!_ ⛏️
