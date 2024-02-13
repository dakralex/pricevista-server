use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq)]
pub enum ImportSourceType {
    ApiOutput,
    Dossier,
    HeissePreise,
    Inflation43z,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq)]
pub enum ImportMarketType {
    AUTO,
    BILLA,
    BIPA,
    DM,
    HOFER,
    LIDL,
    MPREIS,
    MUELLER,
    PENNY,
    SPAR,
    UNIMARKT,
}
