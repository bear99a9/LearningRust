# LearningRust

##Stack Vs Heap

Stack is small memory. 

So data to be big for the stack to hold the heap will take it. 

Data on the stack is easier to find than on the heap. 

You use pointers to get it from the Heap.

##Data Types

Scalar => Numbers, characters and Booleans
Rust uses a 4 Byte size allowing for unicode-32 table or 4294967296 total chars

Compound => Arrays, Tuples
Array holds multiple data of the same type. 
Tuples hold multiple of different types
They are fast but both are fixed size

Strings

String     
Vector u8 data
Mutable
Stored in the heap

&str
Vector u8 data
Immuatale
Can be stored on the heap, stack or embedded in the compiled code

