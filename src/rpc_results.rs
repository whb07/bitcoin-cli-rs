use serde::{Deserialize, Serialize};
use std::{collections::HashMap, u64};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Chain {
    Main,
    Test,
    Signet,
    Regtest,
}

pub mod softforks {

    use super::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    #[allow(non_camel_case_types)]
    enum BIP9Status {
        Defined,
        Started,
        Locked_In,
        Active,
        Failed,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct BIP9Stats {
        period: usize,
        threshold: usize,
        elapsed: usize,
        count: usize,
        possible: bool,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    enum SoftForkType {
        Buried,
        BIP9,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct BIP9StartedInfo {
        status: BIP9Status,
        bit: u8,
        start_time: u64,
        timeout: u64,
        since: u64,
        statistics: BIP9Stats,
        active: bool,
        r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BIP9ActiveInfo {
        status: BIP9Status,
        start_time: u64,
        timeout: u64,
        since: u64,
        height: u64,
        active: bool,
        r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BIP9OthersInfo {
        status: BIP9Status,
        start_time: u64,
        timeout: u64,
        since: u64,
        active: bool,
        r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SoftForkInfo {
        height: usize,
        active: bool,
        r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum BIP9Info {
        BIP9ActiveInfo(BIP9ActiveInfo),
        BIP9OthersInfo(BIP9OthersInfo),
        BIP9StartedInfo(BIP9StartedInfo),
    }
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum SoftForks {
        Bip34(SoftForkInfo),
        Bip65(SoftForkInfo),
        Bip66(SoftForkInfo),
        Csv(SoftForkInfo),
        Segwit(SoftForkInfo),
        TestDummy(BIP9Info),
        TapRoot(BIP9Info),
    }

    #[cfg(test)]
    mod tests {
        use super::{BIP9StartedInfo, BIP9Stats, BIP9Status, SoftForkType};
        use serde_test::{assert_ser_tokens, Token};

        #[test]
        fn bip9status() {
            let statuses = ["\"defined\"", "\"started\"", "\"locked_in\"", "\"active\"", "\"failed\""];
            for status in statuses.iter(){
                match status.as_ref() {
                    "\"defined\"" => assert_eq!(&serde_json::to_string(&BIP9Status::Defined).unwrap(), status),
                    "\"started\"" => assert_eq!(&serde_json::to_string(&BIP9Status::Started).unwrap(), status),
                    "\"locked_in\"" => assert_eq!(&serde_json::to_string(&BIP9Status::Locked_In).unwrap(), status),
                    "\"active\"" => assert_eq!(&serde_json::to_string(&BIP9Status::Active).unwrap(), status),
                    "\"failed\"" => assert_eq!(&serde_json::to_string(&BIP9Status::Failed).unwrap(), status),
                    _ => assert_eq!(&"invalid value at this point", status),
                }
            }
        }
        #[test]
        fn bip9stats() {
            let stats = BIP9Stats {
                period: 32,
                threshold: 292032,
                elapsed: 32042,
                count: 99,
                possible: true,
            };
            assert_eq!("{\"period\":32,\"threshold\":292032,\"elapsed\":32042,\"count\":99,\"possible\":true}", serde_json::to_string(&stats).unwrap());
            assert_eq!(
                stats.count,
                serde_json::from_str::<BIP9Stats>(&serde_json::to_string(&stats).unwrap())
                    .unwrap()
                    .count
            );
        }
        #[test]
        fn bip9startedinfo() {
            let stats = BIP9Stats {
                period: 32,
                threshold: 292032,
                elapsed: 32042,
                count: 99,
                possible: true,
            };
            let bip9 = BIP9StartedInfo {
                status: BIP9Status::Started,
                bit: 32,
                start_time: 123456,
                timeout: 568820,
                active: true,
                since: 92402,
                statistics: stats,
                r#type: SoftForkType::BIP9
            };
            let rebip9: BIP9StartedInfo =
                serde_json::from_str(&serde_json::to_string(&bip9).unwrap()).unwrap();
            assert_eq!(bip9, rebip9);
            assert_eq!(bip9.since, rebip9.since);
            assert_ser_tokens(
                &rebip9,
                &[
                    Token::Struct {
                        name: "BIP9StartedInfo",
                        len: 8,
                    },
                    Token::Str("status"),
                    Token::UnitVariant {
                        variant: "started",
                        name: "BIP9Status",
                    },
                    Token::Str("bit"),
                    Token::U8(32),
                    Token::Str("start_time"),
                    Token::U64(123456),
                    Token::Str("timeout"),
                    Token::U64(568820),
                    Token::Str("since"),
                    Token::U64(92402),
                    Token::Str("statistics"),
                    Token::Struct {
                        name: "BIP9Stats",
                        len: 5,
                    },
                    Token::Str("period"),
                    Token::U64(32),
                    Token::Str("threshold"),
                    Token::U64(292032),
                    Token::Str("elapsed"),
                    Token::U64(32042),
                    Token::Str("count"),
                    Token::U64(99),
                    Token::Str("possible"),
                    Token::Bool(true),
                    Token::StructEnd,
                    Token::Str("active"),
                    Token::Bool(true),
                    Token::Str("type"),
                    Token::UnitVariant {name:"SoftForkType", variant:"bip9"},
                    Token::StructEnd,
                ],
            );
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
// #[serde(rename_all = "lowercase")]
// #[allow(non_camel_case_types)]
// enum PrunedInfoKey {
//     PruneHeight,
//     Automatic_Pruning,
//     Prune_Target_Size,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// #[serde(untagged)]
// #[allow(non_camel_case_types)]
// enum PrunedInfoValue {
//     PruneHeight(u64),
//     Automatic_Pruning(bool),
//     Prune_Target_Size(u64),
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct BlockchainInfo {
//     chain: Chain,
//     blocks: u64,
//     headers: u64,
//     bestblockhash: String,
//     difficulty: f64,
//     mediantime: u64,
//     verificationprogress: f32,
//     initialblockdownload: bool,
//     chainwork: String,
//     size_on_disk: u64,
//     pruned: bool,
//     // softforks: HashMap<softforks::SoftForks, softforks::SoftFork>,
//     warnings: String,
//     #[serde(flatten)]
//     pruned_info: HashMap<PrunedInfoKey, PrunedInfoValue>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct RPCResult {
//     result: BlockchainInfo,
// }

// pub fn hello(data: &str) -> RPCResult {
//     serde_json::from_str(data).unwrap()
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Person {
//     name: String,
//     age: u8,
// }

// #[cfg(test)]
// mod tests {
//     use super::{softforks, BlockchainInfo, Chain, PrunedInfoKey, PrunedInfoValue, RPCResult};
//     use serde_json::json;

//     #[test]
//     fn proper_serialize_chain() {
//         assert_eq!("main", json!(Chain::Main));
//         assert_eq!("test", json!(Chain::Test));
//         assert_eq!("signet", json!(Chain::Signet));
//         assert_eq!("regtest", json!(Chain::Regtest));
//     }

//     #[test]
//     fn proper_deserialize_chain() {
//         let y: Chain = serde_json::from_str(&serde_json::to_string(&Chain::Main).unwrap()).unwrap();
//         assert_eq!("main", json!(y));
//     }
//     // #[test]
//     // fn proper_result() {
//     // let data = r#"{
//     //     "result": {
//     //       "chain": "main",
//     //       "blocks": 669547,
//     //       "headers": 669547,
//     //       "bestblockhash": "000000000000000000054b03be7c1544acc2ed8a4eb2035c8e416ff2a0a02921",
//     //       "difficulty": 21434395961348.92,
//     //       "mediantime": 1612705006,
//     //       "verificationprogress": 0.99999682277192,
//     //       "initialblockdownload": false,
//     //       "chainwork": "0000000000000000000000000000000000000000191dbd2e0effa46508000d04",
//     //       "size_on_disk": 370655486372,
//     //       "pruned": true,
//     //       "pruneheight": 666,
//     //       "softforks": {
//     //         "bip34": { "type": "buried", "active": true, "height": 227931 },
//     //         "bip66": { "type": "buried", "active": true, "height": 363725 },
//     //         "bip65": { "type": "buried", "active": true, "height": 388381 },
//     //         "csv": { "type": "buried", "active": true, "height": 419328 },
//     //         "segwit": { "type": "buried", "active": true, "height": 481824 },
//     //         "bip9": { "type": "bip9": {
//     //             "period": 12345,
//     //             "threshold": 1234525,
//     //             "elapsed": 55555523,
//     //             "count": 235252535235,
//     //             "possible": true,
//     //         },
//     //         "active": true, "height": 481824 }
//     //       },
//     //       "warnings": ""
//     //     },
//     //     "error": null,
//     //     "id": "curltest"
//     //   }"#;
//     //     let resultoutput: RPCResult = serde_json::from_str(data).unwrap();
//     //     assert_eq!("main", json!(resultoutput.result.chain));
//     //     assert_eq!(370655486372, resultoutput.result.size_on_disk);
//     //     assert_eq!( PrunedInfoValue::PruneHeight(666), resultoutput.result.pruned_info[&PrunedInfoKey::PruneHeight]);
//     // }
//     // #[test]
//     // fn serialized_bip34() {
//     //     let data =
//     //         r#"{ softforks: { bip34: { type: \"buried\", active: true, height: 227931 } } }"#;
//     //     // assert_eq!("main", json!(Chain::Main));
//     //     // assert_eq!("test", json!(Chain::Test));
//     //     // assert_eq!("signet", json!(Chain::Signet));
//     //     let fork = serde_json::from_str::<softforks::SoftForks>(data).unwrap();
//     //     // assert_eq!("buried", fork) ;
//     //     // assert_eq!(softforks::SoftForks::Bip34(val), );
//     // }
// }
