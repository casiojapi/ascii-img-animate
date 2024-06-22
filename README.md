# ascii image animator

this rust project will let convert an image to ASCII and animate it (in ascii)

## install

### clone the repository:

```bash
git clone https://github.com/yourusername/ascii_animator.git
```

### build the project:

```bash 
cd ascii_animator
cargo build --release
```


### run the bunary:
```bash
./target/release/ascii_animator
```


## usage:
```bash
ascii_animator [OPTIONS]
```

### options
    + -i, --image <IMAGE>: Path to the image file to animate. Default is "pepe.png".
    + -f, --fps <FPS>: Frames per second for the animation. Default is 20.
    + -e, --evil: Invert ASCII characters for a different visual effect. Default is false.

## example
animate a pepe the frog png (pepe.png), on 30 fps, inversed:
```bash
./target/release/ascii_animator --image pepe.png --fps 30 --evil
```
