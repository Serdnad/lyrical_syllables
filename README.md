# What is it?
At its core, this is a Rust library that counts the syllables in text. It's been built upon to process input either through the command line or in a file and output a pretty, formatted version of the input text with syllable info.

# How does it work?
The process of counting syllables is done by putting each word through a number of tests, where each tests checks the word against a certain rule using regex and updates the syllable count appropriately.

Each "rule" is not a rule of English grammar, but rather generalizations of trends I found to hold in practice. Each rule is also (reasonably) well documented in the code, with examples provided.

# How accurate is it?
The main library is rather accurate, but likely not enough so be compared against as a reference for important uses. As its tested more and more, new exceptions arise, and those are added as test cases (in tests.rs).
By this process, the library has improved to the point where in a moderate piece of text (e. g. song lyrics), it'll frequently count most words correctly, if not all.

Described in another way, there are currently no known exception words for which the library fails. However, it's not too improbable that you find some after a decent number of test runs.

If you do happen to find words for which the library fails (and are still reading), reporting it as an issue would be greatly appreciated (:

# Examples
The output from running the program on the text in a file called lyrics.txt. Notice the -f flag for reading from a file instead of using the command arg.

For help on how to use the interface, run ```./lyrical --help```.
<img center="true" src="/screenshots/screenshot1.png" width="50%">

# To Do
[] Add cmd option to output to a file.
[] More tests!
[] Publish as a Crate? (no intention to, unless I receive requests to.)
