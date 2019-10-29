use crate::errors::SavingError;
pub use crate::traits::PlotableStructure;

// Trait bounds
use core::fmt::Display;

/// See ``Process`` documentation for further use.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Comparison<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    pub(crate) data_set: Vec<crate::process::Process<I, J>>,
    pub(crate) options: crate::process::ProcessOptions,
}

impl<I, J> Comparison<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    pub fn new<K>(data_set: K) -> Comparison<I, J>
    where
        K: IntoIterator<Item = crate::process::Process<I, J>>,
    {
        let options = crate::process::ProcessOptions::default();
        let data_set = data_set
            .into_iter()
            .collect::<Vec<crate::process::Process<I, J>>>();
        Comparison { data_set, options }
    }

    pub fn set_title<S: Display>(mut self, title: S) -> Self {
        self.options.set_title(title.to_string());
        self
    }
    pub fn set_logx<N: Into<f64>>(mut self, logx: N) -> Self {
        self.options.set_logx(logx.into());
        self
    }
    pub fn set_logy<N: Into<f64>>(mut self, logy: N) -> Self {
        self.options.set_logy(logy.into());
        self
    }

    pub fn add<K>(&mut self, anothers: K)
    where
        K: IntoIterator<Item = crate::process::Process<I, J>>,
    {
        for process in anothers.into_iter() {
            self.data_set.push(process);
        }
    }
}

impl<I, J> crate::traits::PlotableStructure for Comparison<I, J>
where
    I: IntoIterator + Clone,
    I::Item: Display,
    J: IntoIterator + Clone,
    J::Item: Display,
{
    /// Saves the data under ``data`` directory, and writes a basic plot_script to be used after execution.
    ///
    /// # Remark
    ///
    /// It is inteded for when one only wants to save the data, and not call any plotting
    /// during the Rust program execution. Posterior plotting can easily be done with the
    /// quick template gnuplot script saved under ``plots`` directory.
    fn save<S: Display>(self, serie: &S) -> Result<(), SavingError> {
        for (counter, process) in self.data_set.into_iter().enumerate() {
            crate::process::Process::save(process, &format!("{}_{}", serie, counter))?
        }
        Ok(())
    }

    /// Plots the data by: saving it in hard-disk, writting a plot script for gnuplot and calling it.
    ///
    /// # Remark
    ///
    /// The plot will be executed asyncroniously and idependently of the Rust program.
    ///
    fn plot<S: Display>(self, serie: &S) -> Result<(), SavingError> {
        self.write_plot_script(serie)?;
        self.save(serie)?;

        let gnuplot_file = format!("{}.gnu", serie);

        let gnuplot_file = &format!("plots\\{}", gnuplot_file);
        std::process::Command::new("gnuplot")
            .arg(gnuplot_file)
            .spawn()?;
        Ok(())
    }

    /// Write simple gnuplot script for this type of data.
    ///
    fn write_plot_script<S: Display>(&self, serie: &S) -> Result<(), SavingError> {
        std::fs::create_dir_all("plots")?;
        let gnuplot_file = &format!("plots\\{}.gnu", serie);

        let mut gnuplot_script = String::new();
        gnuplot_script += "set key\n";
        if let Some(title) = &self.options.title {
            gnuplot_script += &format!("set title \"{}\"\n", title);
        }
        if let Some(logx) = &self.options.logx {
            if *logx <= 0.0 {
                gnuplot_script += "set logscale x\n";
            } else {
                gnuplot_script += &format!("set logscale x {}\n", logx);
            }
        }
        if let Some(logy) = &self.options.logy {
            if *logy <= 0.0 {
                gnuplot_script += "set logscale y\n";
            } else {
                gnuplot_script += &format!("set logscale y {}\n", logy);
            }
        }

        gnuplot_script += "plot ";
        for i in 0..self.data_set.len() {
            let legend = match &self.data_set[i].options.title {
                Some(leg) => String::from(leg),
                None => i.to_string(),
            };
            gnuplot_script += &format!(
                "\"data/{}_{}.txt\" using 1:2 with lines title \"{}\", ",
                serie, i, legend
            );
        }
        gnuplot_script += "\n";
        gnuplot_script += "pause -1\n";

        std::fs::write(&gnuplot_file, &gnuplot_script)?;

        Ok(())
    }
}