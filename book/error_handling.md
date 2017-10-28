# Better Error Handling

So far whenever something goes wrong we've just returned a null pointer to 
indicate failure... This isn't overly ideal. Instead, it'd be nice if we could
get some context behind an error and possibly present a nice friendly message 
to the user.


> **TODO:** Talk about errno-style error handling and logs and stuff