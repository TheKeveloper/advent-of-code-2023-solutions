pub trait Solution {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> u32;
}
