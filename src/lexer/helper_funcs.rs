pub fn is_nonzero_digit(ch: char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&ch)
}
pub fn is_digit(ch: char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&ch)
}
pub fn is_nondigit(ch: char) -> bool {
    [
        '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
    .contains(&ch)
}
