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

This is harder with large data (see [optimizations](arch/OPTIMIZATIONS.md))

So every new variable must act like a deep copy
Of course much of this can be optimized away

# Assignment / Declaration Semantics 
Scope of data is still compiler determined
If something 


# Values  

```
let i64 x = 64;
let x = x * x;
```



# General Function Structure

```
func add_to_all (i64[] args, ) -> () {
    len(args)
}
```

Lambda 

name = () -> {}
