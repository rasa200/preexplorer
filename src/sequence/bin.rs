//! Most basic explorable structure: a sequence of values.
//!
//! # Remarks
//!
//! With the ``prelude`` module, we can easily convert ``IntoIterator``s
//! into ``Sequence`` for ease of use. The same can be achieved with the
//! ``new`` method.
//!
//! # Examples
//!
//! Quick plot.
//! ```no_run
//! use preexplorer::prelude::*;
//! (0..10).preexplore().plot("my_identifier").unwrap();
//! ```
//!
//! Compare ``Sequence``s.
//! ```no_run
//! use preexplorer::prelude::*;
//! pre::Sequences::new(vec![
//!     (0..10).preexplore(),
//!     (0..10).preexplore(),
//!     ])
//!     .plot("my_identifier").unwrap();
//! ```

// Traits
// use core::ops::Add;
pub use crate::traits::{Configurable, Plotable, Saveable};
use core::fmt::Display;

// /// Compare various ``Sequence``s.
// pub mod comparison;

// pub use comparison::Sequences;

/// Sequence of values.
#[derive(Debug, PartialEq, Clone)]
pub struct SequenceBin<T>
where
    T: Display + Clone,
{
    data: Vec<Vec<T>>,
    config: crate::configuration::Configuration,
}

impl<T> SequenceBin<T>
where
    T: Display + Clone,
{
    /// Create a new ``SequenceBin``.
    ///
    /// # Examples
    ///
    /// From a complicated computation.
    /// ```
    /// use preexplorer::prelude::*;
    /// let data = (0..10).map(|i| i * i + 1);
    /// let seq = pre::SequenceBin::new(data);
    /// ```
    pub fn new<I, J>(data: I) -> SequenceBin<T>
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T>,
    {
        let data: Vec<Vec<T>> = data.into_iter().map(|j| j.into_iter().collect()).collect();
        let config = crate::configuration::Configuration::default();

        SequenceBin { data, config }
    }
}

// impl<T> Add for SequenceBin<T>  
// where
//     T: Display + Clone,
// {
//     type Output = crate::SequenceBins<T>;

//     fn add(self, other: crate::SequenceBin<T>) -> crate::SequenceBins<T> { 
//         let mut cmp = self.into();
//         cmp += other;
//         cmp
//     }
// }

impl<T> Configurable for SequenceBin<T>
where
    T: Display + Clone,
{
    fn configuration_mut(&mut self) -> &mut crate::configuration::Configuration {
        &mut self.config
    }
    fn configuration(&self) -> &crate::configuration::Configuration {
        &self.config
    }
}

impl<T> Saveable for SequenceBin<T>
where
    T: Display + Clone,
{
    fn plotable_data(&self) -> String {
        let mut plotable_data = String::new();

        for (counter, values) in self.data.clone().into_iter().enumerate() {
            for value in values {
                plotable_data.push_str(&format!("{}\t{}\n", counter, value));
            }
            // Separate datasets
            plotable_data.push_str("\n\n");
        }

        plotable_data
    }
}

impl<T> Plotable for SequenceBin<T>
where
    T: Display + Clone,
{
    fn plot_script(&self) -> String {
        let mut gnuplot_script = self.opening_plot_script();

        // let dashtype = match self.dashtype() {
        //     Some(dashtype) => dashtype,
        //     None => 1,
        // };
        gnuplot_script += &format!("\
renormalize = 2
do for [i=0:{}] {{
    # Computing some values
    set table $_
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    renormalize = (renormalize < 2 * GPVAL_Y_MAX) ? 2 * GPVAL_Y_MAX : renormalize
    # Plotting a greater domain
    set table '{}'.'_partial_plot'.i
    x_min = (GPVAL_X_MIN < GPVAL_X_MIN - 5 * GPVAL_KDENSITY_BANDWIDTH)? GPVAL_X_MIN : GPVAL_X_MIN - 5 * GPVAL_KDENSITY_BANDWIDTH
    x_max = (GPVAL_X_MAX > GPVAL_X_MAX + 5 * GPVAL_KDENSITY_BANDWIDTH)? GPVAL_X_MAX : GPVAL_X_MAX + 5 * GPVAL_KDENSITY_BANDWIDTH
    set xrange [x_min:x_max]
    plot {:?} index i using 2:(1) smooth kdensity
    unset table
    # Clean the plotting
    unset xrange
    unset yrange
}}

# Plotting the violins
# Right side
plot for [i=0:{}] '{}'.'_partial_plot'.i using (i + $2/renormalize):1 with filledcurve x=i linecolor i
# Left side
replot for [i=0:{}] '{}'.'_partial_plot'.i using (i - $2/renormalize):1 with filledcurve x=i linecolor i
",
            self.data.len() - 1,
            self.data_path(),
            self.data_path().display(),
            self.data_path(),
            self.data.len() - 1,
            self.data_path().display(),
            self.data.len() - 1,
            self.data_path().display(),
        );
        gnuplot_script += &self.ending_plot_script();

        gnuplot_script
    }
}

///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_style() {
        let data = (0..2).map(|i| -> Vec<u64> {
            (0..4).map(|j| j + i).collect()
        });
        let mut seq = SequenceBin::new(data);
        seq.set_style("points");

        assert_eq!(
            &crate::configuration::plot::style::Style::Points,
            seq.style()
        );
    }
}
