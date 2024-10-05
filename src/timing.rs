use std::iter;
use std::time::Instant;
use std::fmt;

pub struct Timing {
    marks: Vec<(Instant, &'static str)>
}

impl Timing {
    pub fn new() -> Self {
        let start = (Instant::now(), "Start");
        Timing { marks: vec![start] }
    }
    
    pub fn mark(&mut self, name: &'static str) {
        let mark = (Instant::now(), name);
        self.marks.push(mark);
    }
}

impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = self.marks[self.marks.len() - 1].0 - self.marks[0].0;
        
        let intervals: Vec<_> = self.marks.iter()
            .zip(self.marks.iter().skip(1))
            .map(|((from, _), (to, name))| (*to - *from, format!("  {name}")))
            .chain(iter::once((total, "Total".into())))
            .collect();
        
        let width = intervals.iter().map(|(_, name)| name.len()).max().unwrap();
        let lines: String = intervals.iter()
            .map(|(duration, name)| format!("\n{:width$} {:10.3} s", name, duration.as_secs_f64()))
            .collect();
        
        write!(f, "{}", lines)
    }
}
