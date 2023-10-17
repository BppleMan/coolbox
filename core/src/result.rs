use color_eyre::Report;
use color_eyre::Result;

pub type LLResult<T, E = Report> = Result<T, E>;
