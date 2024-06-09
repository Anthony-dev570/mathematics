#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum MapSegment {
    Beginning,
    Segment,
    End
}

pub trait MapIter<T> {
    fn map_iter<O, F>(&self, mapper: F) -> Vec<O> where F: Fn((&T, MapSegment)) -> O;
}

impl<T> MapIter<T> for Vec<T> {
    fn map_iter<O, F>(&self, mapper: F) -> Vec<O> where F: Fn((&T, MapSegment)) -> O{
        let mut out = vec![];
        for i in 0..self.len() {
            let segment = match i == 0 {
                true => MapSegment::Beginning,
                false => match i == self.len() - 1 {
                    true => MapSegment::End,
                    false => MapSegment::Segment
                }
            };
            out.push(mapper((&self[i], segment)));
        }
        out
    }
}