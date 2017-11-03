# More Complex Requests

Now that we've got the basics working and things like error handling and 
asynchrony are dealt with, lets flesh out this application. 

At the moment the entire application is composed of a button where the handler
will fire off HTTP requests in the background. The GUI doesn't even display the 
response body, instead printing it to the parent console.

This chapter will deal with:

- Mutating heap-allocated Rust objects (the request headers)
- The separation between UI state and business logic state 
- Best practices when building a larger GUI program

> **TODO:** Complete this chapter, making a basic UI which looks something like
> [this](https://resttesttest.com/) and successfully pings 
> [httpbin](https://httpbin.org/).