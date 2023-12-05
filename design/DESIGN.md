# Assignment / Declaration Semantics 
A data type with heap data must be moved from (declared invalid) when setting something equal to it
Additionally the data in the type must be declared invalid and dealt with (deleted immediatly)
By default objects shallow copy between eachother or do something functionally equivalent
(because copies of data types with heap data are declared invalid this holds true for those objects)
For the moves you can just reuse the old data entirely because that object is invalid
For the fully shallow copies this might be done if it can, as an optimization
Clone operations exist to do a deep copy for objects with heap data 

2 types of objects: 
Owners 
Can be immutable and mutable (whatever that means)

Referances 
Immutable (whatever that means) in what you can do to them and reassignment 
(probably the latter, def the former)

# Variable Assignment and "Reassignment"

```
let i64 x = 64;
let x = x * x;
```

With data structures

```
struct Test {
    i64 test;
}

let Test = Test(1) 
let test.test = 3
```

Of course with the first syntax this is valid
```
let i64 x = 64;
let string x = "string";
```

But this is a compiler error 
```
let Test = Test(1)
let string test.test = "test"
```

Or indeed any type after the let keyword, even if that is the type of the data 

# General Function Structure

```
func add_to_all (i64[] args, ) -> () {
    len(args)
}
```

Lambda 

name = () -> {}
