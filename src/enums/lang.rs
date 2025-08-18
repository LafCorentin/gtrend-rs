//! Represent all langage supported by google.   

use serde::Serialize;
use strum_macros::Display;

#[derive(Debug, Display, Serialize, Clone, Copy)]
#[strum(serialize_all = "kebab_case")]
pub enum Lang {
    AF,
    AR,
    AZ,
    BE,
    BG,
    BN,
    CA,
    CS,
    CY,
    DA,
    DE,
    EL,
    EN,
    EO,
    ES,
    ET,
    EU,
    FA,
    FI,
    FR,
    GA,
    GL,
    GU,
    HI,
    HR,
    HT,
    HU,
    ID,
    IS,
    IT,
    IW,
    JA,
    KA,
    KN,
    KO,
    LA,
    LT,
    LV,
    MK,
    MS,
    MT,
    NL,
    NO,
    PL,
    PT,
    RO,
    RU,
    SK,
    SL,
    SQ,
    SR,
    SV,
    SW,
    TA,
    TE,
    TH,
    TL,
    TR,
    UK,
    UR,
    VI,
    YI,
    #[allow(non_camel_case_types)]
    #[strum(serialize = "zh-CN")]
    #[serde(rename = "zh-CN")]
    ZH_CN,
    #[allow(non_camel_case_types)]
    #[strum(serialize = "zh-TW")]
    #[serde(rename = "zh-TW")]
    ZH_TW,
}
