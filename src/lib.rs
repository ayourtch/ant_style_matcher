#[cfg(test)]
mod tests;

/*
 * mostly a mechanical translation of https://github.com/azagniotov/ant-style-path-matcher
 */

const ASTERISK: char = '*';
const QUESTION: char = '?';
const BLANK: char = ' ';

pub struct AntPathMatcher {
    path_separator: char,
    ignore_case: bool,
    match_start: bool,
    trim_tokens: bool,
}

impl AntPathMatcher {
    pub fn new() -> Self {
        Self::create('/', false, false, false)
    }
    pub fn with_path_separator(&self, sep: char) -> Self {
        Self {
            path_separator: sep,
            ..*self
        }
    }
    pub fn with_ignore_case(&self) -> Self {
        Self {
            ignore_case: true,
            ..*self
        }
    }
    pub fn with_match_start(&self) -> Self {
        Self {
            match_start: true,
            ..*self
        }
    }
    pub fn with_trim_tokens(&self) -> Self {
        Self {
            trim_tokens: true,
            ..*self
        }
    }
    fn create(
        path_separator: char,
        ignore_case: bool,
        match_start: bool,
        trim_tokens: bool,
    ) -> Self {
        Self {
            path_separator,
            ignore_case,
            match_start,
            trim_tokens,
        }
    }

    pub fn is_match(&self, pattern: &str, path: &str) -> bool {
        let pattern_v: Vec<char> = pattern.chars().collect();
        let path_v: Vec<char> = path.chars().collect();
        self.is_match_vec(&pattern_v, 0, &path_v, 0)
    }

    fn is_match_vec(
        &self,
        pattern: &Vec<char>,
        pattern_ptr: usize,
        path: &Vec<char>,
        path_ptr: usize,
    ) -> bool {
        if self.empty(pattern, pattern_ptr) {
            return self.empty(path, path_ptr);
        } else if self.empty(path, path_ptr) && pattern[pattern_ptr] == self.path_separator {
            if self.match_start {
                return true;
            } else if self.length_of(pattern, 2, pattern_ptr)
                && pattern[pattern_ptr + 1] == ASTERISK
            {
                return false;
            }
            return self.is_match_vec(pattern, pattern_ptr + 1, path, path_ptr);
        }

        let pattern_start = pattern[pattern_ptr];
        if pattern_start == ASTERISK {
            if self.length_of(pattern, 1, pattern_ptr) {
                return self.empty(path, path_ptr)
                    || (path[path_ptr] != self.path_separator
                        && self.is_match_vec(pattern, pattern_ptr, path, path_ptr + 1));
            } else if self.double_asterisk_match(pattern, pattern_ptr, path, path_ptr) {
                return true;
            }

            let mut start = path_ptr;
            while start < path.len() {
                if self.is_match_vec(pattern, pattern_ptr + 1, path, start) {
                    return true;
                }
                start = start + 1;
            }

            return self.is_match_vec(pattern, pattern_ptr + 1, path, start);
        }

        let pointer = self.skip_blanks(path, path_ptr);

        (pointer < path.len())
            && (self.equal(path[pointer], pattern_start) || pattern_start == QUESTION)
            && self.is_match_vec(pattern, pattern_ptr + 1, path, pointer + 1)
    }

    fn double_asterisk_match(
        &self,
        pattern: &Vec<char>,
        pattern_ptr: usize,
        path: &Vec<char>,
        path_ptr: usize,
    ) -> bool {
        if pattern[pattern_ptr + 1] != ASTERISK {
            return false;
        } else if pattern.len() - pattern_ptr > 2 {
            return self.is_match_vec(pattern, pattern_ptr + 3, path, path_ptr);
        }
        return false;
    }

    fn skip_blanks(&self, path: &Vec<char>, path_ptr: usize) -> usize {
        let mut pointer = path_ptr;
        if self.trim_tokens {
            while pointer < path.len() && path[pointer] == BLANK {
                pointer = pointer + 1;
            }
        }
        return pointer;
    }

    fn equal(&self, path_char: char, pattern_char: char) -> bool {
        if self.ignore_case {
            path_char.to_lowercase().to_string() == pattern_char.to_lowercase().to_string()
        } else {
            path_char == pattern_char
        }
    }

    fn empty(&self, characters: &Vec<char>, pointer: usize) -> bool {
        characters.len() == pointer
    }

    fn length_of(&self, characters: &Vec<char>, length: usize, pointer: usize) -> bool {
        characters.len() - pointer == length
    }
}
