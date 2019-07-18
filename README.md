# crabsay
crabsay is a version of the unix utility cowsay that displays an ASCII representaion of Ferris, the Rust mascot instead of the usual cow. crabsay implements two key features of cowsay: accepting input from stdin, piped in from other unix utilities, and also accepting text provided as arguments.
## Examples
### Message piped in from stdin
```bash
bash-3.2$ fortune | crabsay
------------------------------------------
| Knebel's Law:                           
| 	It is now proved beyond doubt that smok
| ing is one of the leading               
| 	causes of statistics.                  
------------------------------------------
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
```
### Message supplied as arguments

```bash
bash-3.2$ crabsay Hello World
------------------------------------------
| Hello World                             
------------------------------------------
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
```
## Getting Started
1. Clone Repository
```bash
git clone [GIT_URL]
```
2. Build Binary
```bash
cargo build --release
```
3. Run Command
```
bash-3.2$ fortune | ./target/release/crabsay
------------------------------------------
| The Seventh Commandments for Technicians
| :                                       
| 	Work thou not on energized equipment, f
| or if thou dost, thy fellow             
| 	workers will surely buy beers for thy w
| idow and console her in other           
| 	ways.                                  
------------------------------------------
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
```

## Future Improvements
- [ ] Implement Proper Text Wrapping, right border
- [ ] Add variable width
- [ ] Use clap to handle command line parsing
- [ ] Add proper error handling
- [ ] Add other configuration
- [ ] Clean up and distribute a release

## References
Inspired by https://github.com/mgattozzi/ferris-says

I loved the concept and decided to try to write crabsay as a learning experience