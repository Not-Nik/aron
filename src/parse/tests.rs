// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[cfg(test)]
mod tests {
    use crate::parse::encodings::matches;
    use crate::parse::lexer::{Lexer, Token};

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("# test comment\n.test_directive test_dir_arg\na_label:\nsome other stuff 0x00, 22".to_string());

        assert_eq!(lexer.read().unwrap().as_str(), "\n");
        assert_eq!(lexer.read().unwrap().as_str(), ".test_directive");
        assert_eq!(lexer.read().unwrap().as_str(), "test_dir_arg");
        assert_eq!(lexer.read().unwrap().as_str(), "\n");
        assert_eq!(lexer.read().unwrap().as_str(), "a_label");
        assert_eq!(lexer.read().unwrap().as_str(), ":");
        assert_eq!(lexer.read().unwrap().as_str(), "\n");
        assert_eq!(lexer.read().unwrap().as_str(), "some");
        assert_eq!(lexer.read().unwrap().as_str(), "other");
        assert_eq!(lexer.read().unwrap().as_str(), "stuff");
        assert_eq!(lexer.read().unwrap().as_str(), "0x00");
        assert_eq!(lexer.read().unwrap().as_str(), ",");
        assert_eq!(lexer.read().unwrap().as_str(), "22");
    }

    #[test]
    fn test_assembler() {
        let instr = matches(&vec![Token::new("push"), Token::new("rbp")]);

        assert!(instr.is_ok());
        let instr = instr.unwrap();
        assert_eq!(instr.get_bytes(), &vec![0x55u8]);

        let instr = matches(&vec![Token::new("rdseed"), Token::new("eax")]);

        assert!(instr.is_ok());
        let instr = instr.unwrap();
        assert_eq!(instr.get_bytes(), &vec![0x0F, 0xC7, 0xF8]);
    }
}

