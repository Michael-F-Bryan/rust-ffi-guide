# Better Error Handling

So far whenever something goes wrong we've just returned a null pointer to 
indicate failure... This isn't overly ideal. Instead, it'd be nice if we could
get some context behind an error and possibly present a nice friendly message 
to the user.


> **TODO:** Talk about errno-style error handling, logs, `catch_unwind()` and 
> all that good stuff.

Useful links:

- [libgit2's error handling docs](https://github.com/libgit2/libgit2/blob/master/docs/error-handling.md)