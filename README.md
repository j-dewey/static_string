# static_string
Create a string pool environment to allow for any String to become a &'static str

## Motivation
[&str]s are very useful since they can be copied for free. Unfortunately, any object which wants to hold one must deal with lifetimes and the associated coloring. [String] can be used instead, but then the object cannot be copied.

[&'static str]s can go uncolored, so they are almost always preferable to the non static forms. This requires making the [str] at compile time or leaking data.

## Solution
Moving the ownership of [str] from [&str] and [String] to some static object would effectively make [str] static. This library implements a reference counted string pool to accomplish this.
