# ty - thank you

Please note: This is a beginners project!

A small tool to say thank you. `ty` is a command line tool, that takes a tool 
name as input and sends it to our thank-you server. Every once in a while, we 
try to figure out who the authors of the thanked tools are and then try to 
deliver the collected gratitude for their work. Optionally, it takes a message.


## Usage
```
ty rustc
```

With a message:

```
ty rustc -m "The error message really helped me out, Cheers!"
```


If you just want to thank the last completed command, this is alias will do the 
trick. 

```
alias ty='ty `history -p \!:0`'
```

```
$ cargo build --release 
    Compiling ...
    Finished release [optimized] target(s) in 12.04s
$ ty
```

Here, the `ty` becomes `ty cargo` since history expands !:0 to the first token 
of the most recent command.


## Why?

First, it's a nice thing to say thank you from time to time for all the great 
work one is using every day. This hopefully makes it a little bit more 
convenient.

Also, this seems to be a nicely scoped project to get into rust and its 
ecosystem. Like cli tooling, building a simple webserver, how to organize a 
project, deploy it, and of course use the language itself. 
