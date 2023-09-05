pub fn read(word: u16) -> (u16, u16, u16, u16) {
    ((word & 0xf000) >> 12, (word & 0x0f00) >> 8, (word & 0x00f0) >> 4, word & 0x000f)
}
