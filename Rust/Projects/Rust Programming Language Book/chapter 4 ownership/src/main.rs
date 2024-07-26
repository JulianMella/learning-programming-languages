/*

There are different ways that programming languages manage their memory in

1. Garbage collection

Garbage collection is an automatized management method implemented in programming languages
where the language handles it without any form of modifications done by the user.

The pros are that it is error free and faster write time, but the cons are that
there is no control over memory, slower and unpredictable runtime performance and larger program size

2. Manual memory management (MMM)

MMM is when the coder itself has to allocate memory for the program manually by writing it down.
The pros are that there is better control over memory than Garbage collection, faster runtime and smaller program size, but the cons are
that it is very error prone because the coder can easily write something incorrectly and it takes longer to figure out logic and
essentially has slower write time.

3. Ownership model

Rust is a memory safe language where we don't use manual memory management. This is done by doing a bunch of compile time checks to make sure memory is done in a safe way.

The pros are control over memory, error free (unless unsafe rust is utilized), faster runtime and smaller program size. The cons are slower write time and a learning curve.

---

Rust makes certain decisions on whether something will be stored on the stack or on the heap.

During runtime, the program has access to the stack and the heap.

--- Properties of the stack ---

The stack is fixed in size and cannot grow or shrink during runtime.
It stores stack frames which are created for every funciton that executes.
Stack frames stores the local variables of the function being executed.
The size of the stack frame is calculated at compile time.
Variables only live as long as the stack frame lives (Until it goes out of scope).


--- Properties of heap ---

Can grow or shrink during runtime.
Data stored can be dynamic in size, large amounts of data.
We control the lifetime of the data.

A clear example of something that can be stored in either the stack or the heap is a string and a string literal.

A string literal has a predefined size of how many characters it includes and since it is immutable, it fits right into the stack because it will not grow or shrink.

String on the other hand which can be dynamic in size and cannot be stored directly on the stack and therefore allocates memory for it on the heap.
The pointer to the String variable will be stored in the stack.

Storing on the Stack is faster than the Heap since when something is stored in the Heap, the program takes time on finding a memory area where the variable (or something else) can be stored.

Access time is also slower since you first need to go the pointer in the stack and from there go to the heap.

*/

fn main() {}
