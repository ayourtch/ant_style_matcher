# Ant-style Path Matcher
Consise, recursive  & efficient path matcher implementation for an Ant-style path pattern matching algorithm. 

It is mostly a mechanical translation into Rust of AntPathMatcherArrays class from
https://github.com/azagniotov/ant-style-path-matcher

The matcher matches URLs using the following rules:
* `?` matches one character
* `*` matches zero or more characters
* `**` matches zero or more **_directories_** in a path

### _Dependencies_

None.

### _Examples_

* `com/t?st.jsp` - matches `com/test.jsp` but also `com/tast.jsp` or `com/txst.jsp`
* `com/*.jsp` - matches all `.jsp` files in the `com` directory
* `com/**/test.jsp` - matches all `test.jsp` files underneath the `com` path
* `org/springframework/**/*.jsp` - matches all `.jsp` files underneath the `org/springframework` path
* `org/**/servlet/bla.jsp` - matches `org/springframework/servlet/bla.jsp` but also `org/springframework/testing/servlet/bla.jsp` and `org/servlet/bla.jsp`

### _Complexity_
The matching algorithm of [AntPathMatcherArrays](../master/src/main/java/io/github/azagniotov/matcher/AntPathMatcherArrays.java) uses a `O(N)` space complexity, since the algorithm does not create
substrings (unlike [AntPathMatcher](../master/src/main/java/io/github/azagniotov/matcher/AntPathMatcher.java)) and recurses by moving pointers on the original char arrays

### _Credits_
* The algorithm in this implementation has been borrowed from https://github.com/azagniotov/ant-style-path-matcher
* Part of this README description has been kindly borrowed from Spring's `AntPathMatcher`
* The path matcher configuration options have been inspired by Spring's `AntPathMatcher`

### _License_
MIT
