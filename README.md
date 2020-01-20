# Graph Visualizer

## About
This program, in its current state, takes an array of numerical data and plots a line graph that can be scrolled through like a timeline.  The graph can be extended to handle theoretically any amount of data, but can only plot so many dots at a time, hence the need for a means of scrolling through the data, with a time delta, or offset.

Ultimately, I would like to be able to pipe data into this program as well, as a means of visualizing output in real time. Applications include making resource monitoring fun, etc.

![The program in action][screenie]

## Compiling
Built with Rust 1.40.

Just run 
```
cargo build
``` 
and/or
```
cargo run
```
to compile/run the program.

## Testing
Currently, there are no Rust-y test functions, just a randomly generated vector of ints which are plotted, and the graph is scrubbed through every 250ms or so to show off the offset features.
It's on my to-do list.

## Author(s)
* **Tyler Stoney** - *Initial work*
Feel free to use this.  Kudos if you give me credit, a wag of the finger and a 'tsk tsk' from me if you don't.

[screenie]: https://raw.githubusercontent.com/tston529/Graph_Vis/master/images/screenshot_1_20_20.png
