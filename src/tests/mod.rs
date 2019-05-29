use super::AntPathMatcher;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
#[test]
fn it_works2() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn is_matchWithCaseSensitiveWithDefaultPathSeparator() {
    let pathMatcher = AntPathMatcher::new();

    // test exact matching
    assert_eq!(true, pathMatcher.is_match("test", "test"));
    assert_eq!(true, pathMatcher.is_match("/test", "/test"));
    assert_eq!(
        true,
        pathMatcher.is_match("http://example.org", "http://example.org")
    ); // SPR-14141
    assert_eq!(false, pathMatcher.is_match("/test.jpg", "test.jpg"));
    assert_eq!(false, pathMatcher.is_match("test", "/test"));
    assert_eq!(false, pathMatcher.is_match("/test", "test"));

    // test matching with ?'s
    assert_eq!(true, pathMatcher.is_match("t?st", "test"));
    assert_eq!(true, pathMatcher.is_match("??st", "test"));
    assert_eq!(true, pathMatcher.is_match("tes?", "test"));
    assert_eq!(true, pathMatcher.is_match("te??", "test"));
    assert_eq!(true, pathMatcher.is_match("?es?", "test"));
    assert_eq!(false, pathMatcher.is_match("tes?", "tes"));
    assert_eq!(false, pathMatcher.is_match("tes?", "testt"));
    assert_eq!(false, pathMatcher.is_match("tes?", "tsst"));

    // test matching with *'s
    assert_eq!(true, pathMatcher.is_match("*", "test"));
    assert_eq!(true, pathMatcher.is_match("test*", "test"));
    assert_eq!(true, pathMatcher.is_match("test*", "testTest"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test/Test"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test/t"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test/"));
    assert_eq!(true, pathMatcher.is_match("*test*", "AnothertestTest"));
    assert_eq!(true, pathMatcher.is_match("*test", "Anothertest"));
    assert_eq!(true, pathMatcher.is_match("*.*", "test."));
    assert_eq!(true, pathMatcher.is_match("*.*", "test.test"));
    assert_eq!(true, pathMatcher.is_match("*.*", "test.test.test"));
    assert_eq!(true, pathMatcher.is_match("test*aaa", "testblaaaa"));
    assert_eq!(false, pathMatcher.is_match("test*", "tst"));
    assert_eq!(false, pathMatcher.is_match("test*", "tsttest"));
    assert_eq!(false, pathMatcher.is_match("test*", "test/"));
    assert_eq!(false, pathMatcher.is_match("test*", "test/t"));
    assert_eq!(false, pathMatcher.is_match("test/*", "test"));
    assert_eq!(false, pathMatcher.is_match("*test*", "tsttst"));
    assert_eq!(false, pathMatcher.is_match("*test", "tsttst"));
    assert_eq!(false, pathMatcher.is_match("*.*", "tsttst"));
    assert_eq!(false, pathMatcher.is_match("test*aaa", "test"));
    assert_eq!(false, pathMatcher.is_match("test*aaa", "testblaaab"));

    // test matching with ?'s and /'s
    assert_eq!(true, pathMatcher.is_match("/?", "/a"));
    assert_eq!(true, pathMatcher.is_match("/?/a", "/a/a"));
    assert_eq!(true, pathMatcher.is_match("/a/?", "/a/b"));
    assert_eq!(true, pathMatcher.is_match("/??/a", "/aa/a"));
    assert_eq!(true, pathMatcher.is_match("/a/??", "/a/bb"));
    assert_eq!(true, pathMatcher.is_match("/?", "/a"));

    // test matching with **'s
    assert_eq!(true, pathMatcher.is_match("/**", "/testing/testing"));
    assert_eq!(true, pathMatcher.is_match("/*/**", "/testing/testing"));
    assert_eq!(true, pathMatcher.is_match("/**/*", "/testing/testing"));
    assert_eq!(
        true,
        pathMatcher.is_match("/bla/**/bla", "/bla/testing/testing/bla")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/bla/**/bla", "/bla/testing/testing/bla/bla")
    );
    assert_eq!(true, pathMatcher.is_match("/**/test", "/bla/bla/test"));
    assert_eq!(
        true,
        pathMatcher.is_match("/bla/**/**/bla", "/bla/bla/bla/bla/bla/bla")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/bla*bla/test", "/blaXXXbla/test")
    );
    assert_eq!(true, pathMatcher.is_match("/*bla/test", "/XXXbla/test"));
    assert_eq!(
        false,
        pathMatcher.is_match("/bla*bla/test", "/blaXXXbl/test")
    );
    assert_eq!(false, pathMatcher.is_match("/*bla/test", "XXXblab/test"));
    assert_eq!(false, pathMatcher.is_match("/*bla/test", "XXXbl/test"));

    assert_eq!(false, pathMatcher.is_match("/????", "/bala/bla"));
    assert_eq!(false, pathMatcher.is_match("/**/*bla", "/bla/bla/bla/bbb"));

    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*bla*/**/bla/**",
            "/XXXblaXXXX/testing/testing/bla/testing/testing/"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/*bla*/**/bla/*", "/XXXblaXXXX/testing/testing/bla/testing")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*bla*/**/bla/**",
            "/XXXblaXXXX/testing/testing/bla/testing/testing"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*bla*/**/bla/**",
            "/XXXblaXXXX/testing/testing/bla/testing/testing.jpg"
        )
    );

    assert_eq!(
        true,
        pathMatcher.is_match(
            "*bla*/**/bla/**",
            "XXXblaXXXX/testing/testing/bla/testing/testing/"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("*bla*/**/bla/*", "XXXblaXXXX/testing/testing/bla/testing")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "*bla*/**/bla/**",
            "XXXblaXXXX/testing/testing/bla/testing/testing"
        )
    );
    assert_eq!(
        false,
        pathMatcher.is_match(
            "*bla*/**/bla/*",
            "XXXblaXXXX/testing/testing/bla/testing/testing"
        )
    );

    assert_eq!(false, pathMatcher.is_match("/x/x/**/bla", "/x/x/x/"));

    assert_eq!(true, pathMatcher.is_match("/foo/bar/**", "/foo/bar"));

    assert_eq!(true, pathMatcher.is_match("", ""));

    assert_eq!(true, pathMatcher.is_match("/foo/bar/**", "/foo/bar"));
    assert_eq!(true, pathMatcher.is_match("/resource/1", "/resource/1"));
    assert_eq!(true, pathMatcher.is_match("/resource/*", "/resource/1"));
    assert_eq!(true, pathMatcher.is_match("/resource/*/", "/resource/1/"));
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/top-resource/*/resource/*/sub-resource/*",
            "/top-resource/1/resource/2/sub-resource/3"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/top-resource/*/resource/*/sub-resource/*",
            "/top-resource/999999/resource/8888888/sub-resource/77777777"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*/*/*/*/secret.html",
            "/this/is/protected/path/secret.html"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/*/*/*/*/*.html", "/this/is/protected/path/secret.html")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/*/*/*/*", "/this/is/protected/path")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/springframework/**/*.jsp",
            "org/springframework/web/views/hello.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/springframework/**/*.jsp",
            "org/springframework/web/default.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/springframework/**/*.jsp",
            "org/springframework/default.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/**/servlet/bla.jsp",
            "org/springframework/servlet/bla.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/**/servlet/bla.jsp",
            "org/springframework/testing/servlet/bla.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("org/**/servlet/bla.jsp", "org/servlet/bla.jsp")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "**/hello.jsp",
            "org/springframework/servlet/web/views/hello.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "**/**/hello.jsp",
            "org/springframework/servlet/web/views/hello.jsp"
        )
    );

    assert_eq!(false, pathMatcher.is_match("/foo/bar/**", "/foo /bar"));
    assert_eq!(
        false,
        pathMatcher.is_match("/foo/bar/**", "/foo          /bar")
    );
    assert_eq!(
        false,
        pathMatcher.is_match("/foo/bar/**", "/foo          /               bar")
    );
    assert_eq!(
        false,
        pathMatcher.is_match(
            "/foo/bar/**",
            "       /      foo          /               bar"
        )
    );
    assert_eq!(
        false,
        pathMatcher.is_match(
            "org/**/servlet/bla.jsp",
            "   org   /      servlet    /   bla   .   jsp"
        )
    );
}

#[test]
fn is_matchWithCustomSeparator() {
    let pathMatcher = AntPathMatcher::new().with_path_separator('.');

    assert_eq!(true, pathMatcher.is_match(".foo.bar.**", ".foo.bar"));
    assert_eq!(true, pathMatcher.is_match(".resource.1", ".resource.1"));
    assert_eq!(true, pathMatcher.is_match(".resource.*", ".resource.1"));
    assert_eq!(true, pathMatcher.is_match(".resource.*.", ".resource.1."));
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.springframework.**.*.jsp",
            "org.springframework.web.views.hello.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.springframework.**.*.jsp",
            "org.springframework.web.default.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.springframework.**.*.jsp",
            "org.springframework.default.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.**.servlet.bla.jsp",
            "org.springframework.servlet.bla.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.**.servlet.bla.jsp",
            "org.springframework.testing.servlet.bla.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("org.**.servlet.bla.jsp", "org.servlet.bla.jsp")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("http://example.org", "http://example.org")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "**.hello.jsp",
            "org.springframework.servlet.web.views.hello.jsp"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "**.**.hello.jsp",
            "org.springframework.servlet.web.views.hello.jsp"
        )
    );

    // test matching with ?'s and .'s
    assert_eq!(true, pathMatcher.is_match(".?", ".a"));
    assert_eq!(true, pathMatcher.is_match(".?.a", ".a.a"));
    assert_eq!(true, pathMatcher.is_match(".a.?", ".a.b"));
    assert_eq!(true, pathMatcher.is_match(".??.a", ".aa.a"));
    assert_eq!(true, pathMatcher.is_match(".a.??", ".a.bb"));
    assert_eq!(true, pathMatcher.is_match(".?", ".a"));

    // test matching with **'s
    assert_eq!(true, pathMatcher.is_match(".**", ".testing.testing"));
    assert_eq!(true, pathMatcher.is_match(".*.**", ".testing.testing"));
    assert_eq!(true, pathMatcher.is_match(".**.*", ".testing.testing"));
    assert_eq!(
        true,
        pathMatcher.is_match(".bla.**.bla", ".bla.testing.testing.bla")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(".bla.**.bla", ".bla.testing.testing.bla.bla")
    );
    assert_eq!(true, pathMatcher.is_match(".**.test", ".bla.bla.test"));
    assert_eq!(
        true,
        pathMatcher.is_match(".bla.**.**.bla", ".bla.bla.bla.bla.bla.bla")
    );
    assert_eq!(
        false,
        pathMatcher.is_match(".bla*bla.test", ".blaXXXbl.test")
    );
    assert_eq!(false, pathMatcher.is_match(".*bla.test", "XXXblab.test"));
    assert_eq!(false, pathMatcher.is_match(".*bla.test", "XXXbl.test"));
}

#[test]
fn is_matchWithIgnoreCase() {
    let pathMatcher = AntPathMatcher::new().with_ignore_case();

    assert_eq!(true, pathMatcher.is_match("/foo/bar/**", "/FoO/baR"));
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/springframework/**/*.jsp",
            "ORG/SpringFramework/web/views/hello.JSP"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("org/**/servlet/bla.jsp", "Org/SERVLET/bla.jsp")
    );
    assert_eq!(true, pathMatcher.is_match("/?", "/A"));
    assert_eq!(true, pathMatcher.is_match("/?/a", "/a/A"));
    assert_eq!(true, pathMatcher.is_match("/a/??", "/a/Bb"));
    assert_eq!(true, pathMatcher.is_match("/?", "/a"));
    assert_eq!(true, pathMatcher.is_match("/**", "/testing/teSting"));
    assert_eq!(true, pathMatcher.is_match("/*/**", "/testing/testing"));
    assert_eq!(true, pathMatcher.is_match("/**/*", "/tEsting/testinG"));
    assert_eq!(
        true,
        pathMatcher.is_match("http://example.org", "HtTp://exAmple.org")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("HTTP://EXAMPLE.ORG", "HtTp://exAmple.org")
    );
}

#[test]
fn is_matchWithIgnoreCaseWithCustomPathSeparator() {
    let pathMatcher = AntPathMatcher::new()
        .with_ignore_case()
        .with_path_separator('.');

    assert_eq!(true, pathMatcher.is_match(".foo.bar.**", ".FoO.baR"));
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.springframework.**.*.jsp",
            "ORG.SpringFramework.web.views.hello.JSP"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("org.**.servlet.bla.jsp", "Org.SERVLET.bla.jsp")
    );
    assert_eq!(true, pathMatcher.is_match(".?", ".A"));
    assert_eq!(true, pathMatcher.is_match(".?.a", ".a.A"));
    assert_eq!(true, pathMatcher.is_match(".a.??", ".a.Bb"));
    assert_eq!(true, pathMatcher.is_match(".?", ".a"));
    assert_eq!(true, pathMatcher.is_match(".**", ".testing.teSting"));
    assert_eq!(true, pathMatcher.is_match(".*.**", ".testing.testing"));
    assert_eq!(true, pathMatcher.is_match(".**.*", ".tEsting.testinG"));
    assert_eq!(
        true,
        pathMatcher.is_match("http:..example.org", "HtTp:..exAmple.org")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("HTTP:..EXAMPLE.ORG", "HtTp:..exAmple.org")
    );
}

#[test]
fn is_matchWithMatchStart() {
    let pathMatcher = AntPathMatcher::new().with_match_start();

    // test exact matching
    assert_eq!(true, pathMatcher.is_match("test", "test"));
    assert_eq!(true, pathMatcher.is_match("/test", "/test"));
    assert_eq!(false, pathMatcher.is_match("/test.jpg", "test.jpg"));
    assert_eq!(false, pathMatcher.is_match("test", "/test"));
    assert_eq!(false, pathMatcher.is_match("/test", "test"));

    // test matching with ?'s
    assert_eq!(true, pathMatcher.is_match("t?st", "test"));
    assert_eq!(true, pathMatcher.is_match("??st", "test"));
    assert_eq!(true, pathMatcher.is_match("tes?", "test"));
    assert_eq!(true, pathMatcher.is_match("te??", "test"));
    assert_eq!(true, pathMatcher.is_match("?es?", "test"));
    assert_eq!(false, pathMatcher.is_match("tes?", "tes"));
    assert_eq!(false, pathMatcher.is_match("tes?", "testt"));
    assert_eq!(false, pathMatcher.is_match("tes?", "tsst"));

    // test matching with *'s
    assert_eq!(true, pathMatcher.is_match("*", "test"));
    assert_eq!(true, pathMatcher.is_match("test*", "test"));
    assert_eq!(true, pathMatcher.is_match("test*", "testTest"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test/Test"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test/t"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test/"));
    assert_eq!(true, pathMatcher.is_match("*test*", "AnothertestTest"));
    assert_eq!(true, pathMatcher.is_match("*test", "Anothertest"));
    assert_eq!(true, pathMatcher.is_match("*.*", "test."));
    assert_eq!(true, pathMatcher.is_match("*.*", "test.test"));
    assert_eq!(true, pathMatcher.is_match("*.*", "test.test.test"));
    assert_eq!(true, pathMatcher.is_match("test*aaa", "testblaaaa"));
    assert_eq!(false, pathMatcher.is_match("test*", "tst"));
    assert_eq!(false, pathMatcher.is_match("test*", "test/"));
    assert_eq!(false, pathMatcher.is_match("test*", "tsttest"));
    assert_eq!(false, pathMatcher.is_match("test*", "test/"));
    assert_eq!(false, pathMatcher.is_match("test*", "test/t"));
    assert_eq!(true, pathMatcher.is_match("test/*", "test"));
    assert_eq!(true, pathMatcher.is_match("test/t*.txt", "test"));
    assert_eq!(false, pathMatcher.is_match("*test*", "tsttst"));
    assert_eq!(false, pathMatcher.is_match("*test", "tsttst"));
    assert_eq!(false, pathMatcher.is_match("*.*", "tsttst"));
    assert_eq!(false, pathMatcher.is_match("test*aaa", "test"));
    assert_eq!(false, pathMatcher.is_match("test*aaa", "testblaaab"));

    // test matching with ?'s and /'s
    assert_eq!(true, pathMatcher.is_match("/?", "/a"));
    assert_eq!(true, pathMatcher.is_match("/?/a", "/a/a"));
    assert_eq!(true, pathMatcher.is_match("/a/?", "/a/b"));
    assert_eq!(true, pathMatcher.is_match("/??/a", "/aa/a"));
    assert_eq!(true, pathMatcher.is_match("/a/??", "/a/bb"));
    assert_eq!(true, pathMatcher.is_match("/?", "/a"));

    // test matching with **'s
    assert_eq!(true, pathMatcher.is_match("/**", "/testing/testing"));
    assert_eq!(true, pathMatcher.is_match("/*/**", "/testing/testing"));
    assert_eq!(true, pathMatcher.is_match("/**/*", "/testing/testing"));
    assert_eq!(true, pathMatcher.is_match("test*/**", "test/"));
    assert_eq!(true, pathMatcher.is_match("test*/**", "test/t"));
    assert_eq!(
        true,
        pathMatcher.is_match("/bla/**/bla", "/bla/testing/testing/bla")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/bla/**/bla", "/bla/testing/testing/bla/bla")
    );
    assert_eq!(true, pathMatcher.is_match("/**/test", "/bla/bla/test"));
    assert_eq!(
        true,
        pathMatcher.is_match("/bla/**/**/bla", "/bla/bla/bla/bla/bla/bla")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/bla*bla/test", "/blaXXXbla/test")
    );
    assert_eq!(true, pathMatcher.is_match("/*bla/test", "/XXXbla/test"));
    assert_eq!(
        false,
        pathMatcher.is_match("/bla*bla/test", "/blaXXXbl/test")
    );
    assert_eq!(false, pathMatcher.is_match("/*bla/test", "XXXblab/test"));
    assert_eq!(false, pathMatcher.is_match("/*bla/test", "XXXbl/test"));

    assert_eq!(false, pathMatcher.is_match("/????", "/bala/bla"));
    assert_eq!(true, pathMatcher.is_match("/**/*bla", "/bla/bla/bla/bbb"));

    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*bla*/**/bla/**",
            "/XXXblaXXXX/testing/testing/bla/testing/testing/"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/*bla*/**/bla/*", "/XXXblaXXXX/testing/testing/bla/testing")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*bla*/**/bla/**",
            "/XXXblaXXXX/testing/testing/bla/testing/testing"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/*bla*/**/bla/**",
            "/XXXblaXXXX/testing/testing/bla/testing/testing.jpg"
        )
    );

    assert_eq!(
        true,
        pathMatcher.is_match(
            "*bla*/**/bla/**",
            "XXXblaXXXX/testing/testing/bla/testing/testing/"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("*bla*/**/bla/*", "XXXblaXXXX/testing/testing/bla/testing")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "*bla*/**/bla/**",
            "XXXblaXXXX/testing/testing/bla/testing/testing"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "*bla*/**/bla/*",
            "XXXblaXXXX/testing/testing/bla/testing/testing"
        )
    );

    assert_eq!(true, pathMatcher.is_match("/x/x/**/bla", "/x/x/x/"));

    assert_eq!(true, pathMatcher.is_match("", ""));
}

#[test]
fn is_matchWithTrimTokens() {
    let pathMatcher = AntPathMatcher::new().with_trim_tokens();

    assert_eq!(true, pathMatcher.is_match("/foo/bar/**", "/foo /bar"));
    assert_eq!(
        true,
        pathMatcher.is_match("/foo/bar/**", "/foo          /bar")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/foo/bar/**", "/foo          /               bar")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "/foo/bar/**",
            "       /      foo          /               bar"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match("/**/*", "/      testing     /     testing   ")
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org/**/servlet/bla.jsp",
            "   org   /      servlet    /   bla   .   jsp"
        )
    );
}

#[test]
fn is_matchWithIgnoreCaseWithCustomPathSeparatorWithTrimTokens() {
    let pathMatcher = AntPathMatcher::new()
        .with_ignore_case()
        .with_trim_tokens()
        .with_path_separator('.');

    assert_eq!(true, pathMatcher.is_match(".foo.bar.**", ".FoO.baR"));
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.springframework.**.*.jsp",
            "ORG.  SpringFramework.web.views.hello . JSP"
        )
    );
    assert_eq!(
        true,
        pathMatcher.is_match(
            "org.**.servlet.bla.jsp",
            "Org        .SERVLET     .     bla.jsp"
        )
    );
    assert_eq!(true, pathMatcher.is_match(".?", ".       A"));
    assert_eq!(true, pathMatcher.is_match(".?.a", ".a.A"));
    assert_eq!(
        true,
        pathMatcher.is_match(".a.??", ".       a      .     B       b")
    );
    assert_eq!(true, pathMatcher.is_match(".?", ".a"));
    assert_eq!(
        true,
        pathMatcher.is_match(".**", ".testing       .   teSting")
    );
    assert_eq!(true, pathMatcher.is_match(".*.**", ".testing.testing"));
    assert_eq!(
        true,
        pathMatcher.is_match(".**.*", "    .         tEsting .    testinG")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("http:..example.org", " H t T p : . . exAmple . org")
    );
    assert_eq!(
        true,
        pathMatcher.is_match("HTTP:..EXAMPLE.ORG", "HtTp:..exAmple.org")
    );
}
