#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&s| s)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|&m| m)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores().to_vec();
        scores.sort_unstable_by(|a,b| b.cmp(a));

        scores.truncate(3);

        scores
    }
}
