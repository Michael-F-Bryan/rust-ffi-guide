# Better Error Handling

So far whenever something goes wrong we've just returned a null pointer to 
indicate failure... This isn't overly ideal. Instead, it'd be nice if we could
get some context behind an error and possibly present a nice friendly message 
to the user.

A very powerful error handling mechanism in C-style programs (technically this 
is one because our FFI bindings export a C interface) is modelled on `errno`.

This employs a thread-local variable which holds the most recent error as well 
as some convenience functions for getting/clearing this variable. The theory is
if a function fails then it should return an "obviously invalid" value, this is
commonly `-1` when returning an integer or `null` when returning a pointer. The
user can then check for this and consult the most recent error for more 
information. Of course that means all fallible operations *must* update the most
recent error if they fail.

While it isn't as elegant as Rust's monad-style `Result<T, E>` with `?` and the
various combinators, it actually turns out to be a pretty solid error handling
technique in practice.

> **Note:** It is **highly recommended** to have a skim through libgit2's 
> [error handling docs][libgit]. The error handling mechanism we'll be using 
> takes a lot of inspiration from `libgit2`.


[libgit]: (https://github.com/libgit2/libgit2/blob/master/docs/error-handling.md)
