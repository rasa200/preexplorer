# preexplorer
Easy plotter and saver of simple data. Handy tool for development stage or small computational projects. 



Handy way to save data, have a quick view and an initial [gnuplot](http://www.gnuplot.info/) script to plot it. 
if you are looking for a pure Rust plotter, check out [plotters](https://crates.io/crates/plotters).



Rust is meant for computations and plotting is usually an iterative process, done best in tools like [gnuplot](http://www.gnuplot.info/). That is way we separate both steps: compute in Rust, save the data, find the best plot with [gnuplot](http://www.gnuplot.info/). 

# Purpose

Do you have a costly process in Rust and want to save the data for postprocessing?
Would you like to still have a basic glance to check it and leave fine-tuning of the plot for later?
This is the crate for you!

# Main features

- Easy plotting
- Easy comparison
- Extensible options
- Implementable traits?

# Examples

For more, see the folder [examples](https://github.com/rasa200/preexplorer/tree/master/examples). 

# Warning

For now, this crate is Windows only, because of extensions in file name. Help wanted.

# Installation

- Download and [install gnuplot](http://www.gnuplot.info/download.html), a command line engine for plotting. (Note that the gnuplot project has nothing to do with GNU).
- Add ``preexplorer = "0.1"`` to your ``Cargo.toml`` file under ``[dependencies]``.
- I suggest to simply go with ``use preexplorer::prelude::*;`` in your binary, and then use the short-name ``pre::...`` and the ``preexplore`` method directly in iterators and tuple of iterators.



# To do 

- pub traits should be externally implementable
  - Expose base_plot_script
    - It is behind the Configuration 
      - Shall we expose the whole thing? No
        - Document well that all configurations are taken into account
        - Document how to add more settings
    - At least examples for plot script in Plotable Trait documentation
  - Document 
    - Trait structure
      - Configurable + -> Saveable + -> Plotable
    - Saveable
      - raw_data
    - Plotable
      - plot_script
      - opening_plot_script
      - ending_plot_script
      - Make a personalized plot_script writable
        - Add a macro for it?
- New Structs for error plotting
  - SequenceWithError 
  - ProcessWithError
- More customization
  - Audio
    - audify()
    - sonify()
- Document project
  - Add bingenes crate as an example of implementing traits

# Questions

Why the following code does not work?

```rust
MarkovChain::new(init_state, &transition)
    .take(max_steps)
	.preexplore()
```



# Q & A

1. Why do the structs not implement Eq traits?
   Because we use f64
2. Why do the structs not implement PartialOrd traits?
   Because we use HashMap
3. Why do we not have two Configurations? SaveConfiguration and PlotConfiguration?
   To make life simpler for those outside the crate
4. Why processes must be the same structs to be compared? 
   Because of Rust explicit typing: comparisons need to save an explicit type. 
5. Are there n-dimensional variants?
   No, it is out of scope. Please implement your own plot script for that. You can do so easily based in the Data struct. 

# Disclaimer

There is no connection with the gnuplot project.
