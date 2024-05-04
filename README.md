# Parser for our_journey videogame

This parser takes a type of file .st (story) that takes a file
and gets the Story, Characters and Dialog in Rust structs. The
file structure must be like this
```
## <Name of Story>
<description>
## Characters
narrator: {
    <narrator1>,
    <narrator2>,
    ...
}
players: {
    <player1>,
    <player2>,
    <player3>,
    ...
}
env: {
    <object1>,
    <object2>,
    <object3>,
    ...
}
## Story
<charA>: Dialog
<charB>: Dialog
<charC>: Dialog
<charD>: Dialog
<charA>: Dialog
...
```
This parser is for a story that is linear, I want to resemble more
of a movie than a choose your our adventure game, as this is intended
to tell a story that the option you have is to keep going or go back
to understand why things happen.
