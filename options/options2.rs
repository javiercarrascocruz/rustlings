// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        /* if "let" destructures optional_target into "Some(word)",
         * evaluate the block */
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        /* while "let" destructures optional_integers (flatten is needed to destructure the pop
         * operation, which adds anoter layer of Option<T>) into "Some(word)", evaluate the block */
        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
