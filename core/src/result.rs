use color_eyre::Report;
use color_eyre::Result;

pub type CBResult<T, E = Report> = Result<T, E>;
