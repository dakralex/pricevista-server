use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq)]
pub enum ImportSourceType {
    ApiOutput,
    Dossier,
    HeissePreise,
}
