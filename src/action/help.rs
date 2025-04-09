pub fn help<W: std::io::Write>(mut writer: W) {
    // UNWRAP: If this panics, we are already exiting and in a non-recoverable state
    writeln!(
        writer,
        "Usage: {0} [ACTION] [args...]

A directory classifier

Commands:
  q <query>  Query the matching directory entries
  a <path>   Registers an access to the given path
  h          Show this help message

Examples:
  {0} q co/ru/z  Queries {0} for the matches of `co/ru/z` and their scores

Output:
  If there's a single match, just the match is returned:
    $ {0} q co/ru/z
    /home/user/code/rust/{0}

  If there are multiple matches, each will take a line preceded by their score:
    $ {0} q co/ru/z
    100 /home/user/code/rust/{0}/
    60 /homu/user/records/carl/utils/lazy",
        env!("CARGO_BIN_NAME"),
    )
    .unwrap();
}
