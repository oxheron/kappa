
Large amounts of data is a bit harder to deal with.
A few things can be done.

You can just do it how it would be done with strict eval (ofc this applies to overwrites only).
And store the edits somewhere (for lightly edited structure) 
For heavy edits the performance time is still roughly the same, and the memory usage is more 
But memory can be saved in other ways
This way any code that references edited structures can find the way back  
And obviously objects that are only used once can just be mutated, as no other function will depend on them 
