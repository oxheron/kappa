// High level features


# How is this different that haskell, elixir, erlang, etc.

Optimizer - built to make some of the more imperative features feasible
This includes heap based data (pointers, arrays)
"Feels more like" an imperative language, but has all of the benifits of pf



# Immutablity
Variables are immutable

```
x: i64 = 64

// For this obviously x is a new object 
x: String = String(x)

// More complicated copy 
y: String = "hello, "
y: = y.append("world")
```

This is harder with large data (see [Optimizations](arch/OPTIMIZATIONS.md))

So every new variable must act like a deep copy
Of course much of this can be optimized away

# Functions 

This can be declared anywhere
All arguments are technically considered "free parameters"
This means they can be based on the environment and return a function that has the rest of the arguments

name = (args) [rvals] 
{
   code 
} 
