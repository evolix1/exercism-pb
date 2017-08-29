extern crate crypto_square;

use crypto_square::encrypt;

#[test]
fn test_empty_last_line() {
    let input = "congratulate";
    let expected = "crl oaa ntt gue";
    assert_eq!(encrypt(input), expected);
}

#[test]
#[ignore]
fn test_empty_input() {
    assert_eq!(encrypt(""), "");
}


#[test]
#[ignore]
fn test_encrypt_also_decrypts_square() {
    // note that you only get the exact input back if:
    // 1. no punctuation
    // 2. even spacing
    // 3. all lowercase
    // 4. square input
    let example = "lime anda coco anut";
    assert_eq!(example, &encrypt(&encrypt(example)));
}

#[test]
#[ignore]
fn test_example() {
    let input = "If man was meant to stay on the ground, god would have given us roots.";
    let expected = "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn sseoau";
    assert_eq!(encrypt(input), expected);
}

#[test]
#[ignore]
fn test_spaces_are_reorganized() {
    let input = "     a  b     e      t             ";
    let expected = "ae bt";
    assert_eq!(encrypt(input), expected);
}

#[test]
#[ignore]
fn test_everything_becomes_lowercase() {
    let input = "cAsE";
    let expected = "cs ae";
    assert_eq!(encrypt(input), expected);
}

#[test]
#[ignore]
fn test_ignore_non_ascii_chars() {
    let input = "She got her education, then got rich programming: 👩‍🎓🎓👩‍💻💰";
    let expected = "setnhm hrigpm eeoori gdnton outrgg tchir haeca";
    assert_eq!(encrypt(input), expected);
}

#[test]
#[ignore]
fn test_long() {
    let input = r#"
We choose to go to the moon.

We choose to go to the moon in this decade and do the other things,
not because they are easy, but because they are hard, because that
goal will serve to organize and measure the best of our energies and
skills, because that challenge is one that we are willing to accept,
one we are unwilling to postpone, and one which we intend to win,
and the others, too.

-- John F. Kennedy, 12 September 1962
        "#;
    let expected = "wohaseagabnttenwan eoennaronedcwpghnf cnmdoseaisshettidk hwodtyhlztkaaoocte oeoobbaweoilrnphhn ocnteuriaflleeowen shihctdlnolewwseoe eoneabbldusnietitd totoueesmrbglapnhy oshtscceeeeelrotes geiheaaranciienere otsetuuvseasnuensp todrhsseuruognadtt ogeteeetrgsntwntoe tochyttoeieeoidoom htaiahhotettalowjb eodnrearhshhclnioe mtegeytgeaaacienhr";
    assert_eq!(encrypt(input), expected);
}
