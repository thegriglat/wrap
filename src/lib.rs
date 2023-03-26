pub struct Wrapper {
    pos: usize,
    max_length: usize,
    buffer_pos: usize,
}

impl Wrapper {
    pub fn new(max_length: usize) -> Wrapper {
        Wrapper {
            pos: 0,
            max_length,
            buffer_pos: 0,
        }
    }

    pub fn read<F>(&mut self, buf: &str, callback: F)
    where
        F: Fn(&str),
    {
        let new_lines = (self.pos + buf.len()) / self.max_length;
        for _ in 0..new_lines {
            let line = &buf[self.buffer_pos..self.buffer_pos + self.max_length];
            callback(line);
            self.buffer_pos += self.max_length;
        }
        callback(&buf[self.buffer_pos..]);
        self.pos = (self.pos + buf.len()) % self.max_length;
    }
}
