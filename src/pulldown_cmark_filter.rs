use std::collections::VecDeque;
use pulldown_cmark::Event;

#[derive(Debug)]
pub struct Filter<'a, I> {
    iter: I,
    queue: VecDeque<Event<'a>>,
}

impl<'a, I> Filter<'a, I>
where
    I: Iterator<Item=Event<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter,
            queue: VecDeque::new(),
        }
    }
}

impl<'a, I> Iterator for Filter<'a, I>
where
    I: Iterator<Item=Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
