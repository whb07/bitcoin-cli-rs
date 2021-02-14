use serde::{Deserialize, Serialize};
use std::{collections::HashMap, u64};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Chain {
    Main,
    Test,
    Signet,
    Regtest,
}

pub mod softforks {

    use super::{Deserialize, Serialize};
    use std::convert::TryFrom;
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    #[allow(non_camel_case_types)]
    pub enum Bip9Status {
        Defined,
        Started,
        Locked_In,
        Active,
        Failed,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct Statistics {
        pub  period: usize,
        pub  threshold: usize,
        pub  elapsed: usize,
        pub  count: usize,
        pub  possible: bool,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum SoftForkType {
        Buried,
        BIP9,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(try_from = "u8")]
    pub struct BlockVersionFieldBit(pub u8);

    impl TryFrom<u8> for BlockVersionFieldBit {
        type Error = &'static str;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            if value < 29 {
                Ok(BlockVersionFieldBit(value))
            } else {
                Err("value greater than 28 not valid")
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    pub struct StartedInfo {
        pub  status: Bip9Status,
        pub  bit: BlockVersionFieldBit,
        pub  start_time: u64,
        pub  timeout: u64,
        pub  since: u64,
        pub  statistics: Statistics,
        pub  active: bool,
        pub  r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ActiveInfo {
        pub status: Bip9Status,
        pub start_time: u64,
        pub timeout: u64,
        pub since: u64,
        pub height: u64,
        pub active: bool,
        pub r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct OthersInfo {
        pub  status: Bip9Status,
        pub  start_time: u64,
        pub  timeout: u64,
        pub  since: u64,
        pub  active: bool,
        pub  r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase", untagged)]
    pub enum BIP9Info {
        Active(ActiveInfo),
        Others(OthersInfo),
        Started(StartedInfo),
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[serde(rename_all = "lowercase")]
    pub enum SoftForkNames {
        Bip34,
        Bip65,
        Bip66,
        Csv,
        Segwit,
        TestDummy,
        TapRoo,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SoftForkInfo {
        height: usize,
        active: bool,
        r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase", untagged)]
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
        use super::{
            ActiveInfo, BIP9Info, Bip9Status, BlockVersionFieldBit, SoftForkInfo, SoftForkType,
            SoftForks, StartedInfo, Statistics,
        };
        use serde_test::{assert_ser_tokens, Token};

        #[test]
        fn bip9status() {
            let statuses = [
                "\"defined\"",
                "\"started\"",
                "\"locked_in\"",
                "\"active\"",
                "\"failed\"",
            ];
            for status in statuses.iter() {
                match status.as_ref() {
                    "\"defined\"" => assert_eq!(
                        &serde_json::to_string(&Bip9Status::Defined).unwrap(),
                        status
                    ),
                    "\"started\"" => assert_eq!(
                        &serde_json::to_string(&Bip9Status::Started).unwrap(),
                        status
                    ),
                    "\"locked_in\"" => assert_eq!(
                        &serde_json::to_string(&Bip9Status::Locked_In).unwrap(),
                        status
                    ),
                    "\"active\"" => {
                        assert_eq!(&serde_json::to_string(&Bip9Status::Active).unwrap(), status)
                    }
                    "\"failed\"" => {
                        assert_eq!(&serde_json::to_string(&Bip9Status::Failed).unwrap(), status)
                    }
                    _ => assert_eq!(&"invalid value at this point", status),
                }
            }
        }
        #[test]
        fn bip9stats() {
            let stats = Statistics {
                period: 32,
                threshold: 292032,
                elapsed: 32042,
                count: 99,
                possible: true,
            };
            assert_eq!("{\"period\":32,\"threshold\":292032,\"elapsed\":32042,\"count\":99,\"possible\":true}", serde_json::to_string(&stats).unwrap());
            assert_eq!(
                stats.count,
                serde_json::from_str::<Statistics>(&serde_json::to_string(&stats).unwrap())
                    .unwrap()
                    .count
            );
        }
        #[test]
        fn bip9activefork() {
            let data = "{\"status\":\"active\",\"start_time\":12345,\"timeout\":12345,\"since\":100,\"height\":481824,\"active\":true,\"type\":\"bip9\"}";
            let active_info = ActiveInfo {
                status: Bip9Status::Active,
                start_time: 12345,
                timeout: 12345,
                since: 100,
                height: 481824,
                active: true,
                r#type: SoftForkType::BIP9,
            };
            let info = BIP9Info::Active(active_info);
            let fork = SoftForks::TestDummy(info);
            assert_eq!(data, serde_json::to_string(&fork).unwrap());
        }

        #[test]
        fn bip9startedfork() {
            let data = "{\"status\":\"started\",\"bit\":19,\"start_time\":12345,\"timeout\":12345,\"since\":100,\
                \"statistics\":{\"period\":100,\"threshold\":250,\"elapsed\":12345,\"count\":99,\"possible\":false},\
                \"active\":false,\"type\":\"bip9\"}";
            let started_info = StartedInfo {
                status: Bip9Status::Started,
                bit: BlockVersionFieldBit(19),
                start_time: 12345,
                timeout: 12345,
                since: 100,
                statistics: Statistics {
                    period: 100,
                    threshold: 250,
                    elapsed: 12345,
                    count: 99,
                    possible: false,
                },
                active: false,
                r#type: SoftForkType::BIP9,
            };
            let info = BIP9Info::Started(started_info);
            let fork = SoftForks::TapRoot(info);
            assert_eq!(data, serde_json::to_string(&fork).unwrap());
        }

        #[test]
        fn bip34fork() {
            let data = "{\"height\":227931,\"active\":true,\"type\":\"buried\"}";
            let fork = SoftForks::Bip34(SoftForkInfo {
                r#type: SoftForkType::Buried,
                active: true,
                height: 227931,
            });
            assert_eq!(data, serde_json::to_string(&fork).unwrap());
        }

        #[test]
        fn bip9startedinfo() {
            let stats = Statistics {
                period: 32,
                threshold: 292032,
                elapsed: 32042,
                count: 99,
                possible: true,
            };
            let bip9 = StartedInfo {
                status: Bip9Status::Started,
                bit: BlockVersionFieldBit(17),
                start_time: 123456,
                timeout: 568820,
                active: true,
                since: 92402,
                statistics: stats,
                r#type: SoftForkType::BIP9,
            };
            let rebip9: StartedInfo =
                serde_json::from_str(&serde_json::to_string(&bip9).unwrap()).unwrap();
            assert_eq!(bip9, rebip9);
            assert_eq!(bip9.since, rebip9.since);
            assert_ser_tokens(
                &rebip9,
                &[
                    Token::Struct {
                        name: "StartedInfo",
                        len: 8,
                    },
                    Token::Str("status"),
                    Token::UnitVariant {
                        variant: "started",
                        name: "Bip9Status",
                    },
                    Token::Str("bit"),
                    Token::NewtypeStruct {
                        name: "BlockVersionFieldBit",
                    },
                    Token::U8(17),
                    Token::Str("start_time"),
                    Token::U64(123456),
                    Token::Str("timeout"),
                    Token::U64(568820),
                    Token::Str("since"),
                    Token::U64(92402),
                    Token::Str("statistics"),
                    Token::Struct {
                        name: "Statistics",
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
                    Token::UnitVariant {
                        name: "SoftForkType",
                        variant: "bip9",
                    },
                    Token::StructEnd,
                ],
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
enum PrunedInfoKey {
    PruneHeight,
    Automatic_Pruning,
    Prune_Target_Size,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
enum PrunedInfoValue {
    PruneHeight(u64),
    Automatic_Pruning(bool),
    Prune_Target_Size(u64),
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockchainInfo {
    chain: Chain,
    blocks: u64,
    headers: u64,
    bestblockhash: String,
    difficulty: f64,
    mediantime: u64,
    verificationprogress: f32,
    initialblockdownload: bool,
    chainwork: String,
    size_on_disk: u64,
    pruned: bool,
    softforks: HashMap<softforks::SoftForkNames, softforks::SoftForks>,
    warnings: String,
    #[serde(flatten)]
    pruned_info: HashMap<PrunedInfoKey, PrunedInfoValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RPCResult {
    result: BlockchainInfo,
}

#[cfg(test)]
mod tests {

    use super::{Chain, RPCResult, softforks};
    use serde_json::json;
    use softforks::{SoftForkType, StartedInfo};
    #[test]
    fn proper_serialize_chain() {
        assert_eq!("main", json!(Chain::Main));
        assert_eq!("test", json!(Chain::Test));
        assert_eq!("signet", json!(Chain::Signet));
        assert_eq!("regtest", json!(Chain::Regtest));
    }

    #[test]
    fn proper_deserialize_chain() {
        let y: Chain = serde_json::from_str(&serde_json::to_string(&Chain::Main).unwrap()).unwrap();
        assert_eq!("main", json!(y));
    }
    #[test]
    fn test_block_version_field_bit() {
        let data = "{\"result\":{\"chain\":\"main\",\"blocks\":670484,\"headers\":670484,\"bestblockhash\":\"00000000000000000005e98912baa79cdb190c7cfae6b07b963331e0a994af1c\",\
        \"difficulty\":21434395961348.92,\"mediantime\":1613255634,\"verificationprogress\":0.9999998819394618,\"initialblockdownload\":false,\
        \"chainwork\":\"0000000000000000000000000000000000000000196517c6d0bd6023c7b9ebff\",\"size_on_disk\":372048172533,\
        \"pruned\":false,\"softforks\":{\"bip34\":{\"type\":\"buried\",\"active\":true,\"height\":227931},\
        \"bip66\":{\"type\":\"buried\",\"active\":true,\"height\":363725},\"bip65\":{\"type\":\"buried\",\"active\":true,\"height\":388381},\
        \"csv\":{\"type\":\"buried\",\"active\":true,\"height\":419328},\"segwit\":{\"type\":\"buried\",\"active\":true,\"height\":481824}},\
        \"warnings\":\"\"},\"error\":null,\"id\":\"curltest\"}";
        let rpc_result: RPCResult = serde_json::from_str(&data).unwrap();
        assert_eq!(rpc_result.result.chain, Chain::Main);
        assert_eq!(
            rpc_result.result.bestblockhash,
            "00000000000000000005e98912baa79cdb190c7cfae6b07b963331e0a994af1c"
        );
    }

    // #[test]
    // fn test_chain_test_and_bip9_fork() {
    //     let data ="{\"result\":{\"chain\":\"test\",\"blocks\":669788,\"headers\":669788,\
    //     \"bestblockhash\":\"0000000000000000000877f2a7e071ab1a9c036f09dce48e8e72a79e2d10f948\",\"difficulty\":21434395961348.92,\
    //     \"mediantime\":1612831628,\"verificationprogress\":0.9999978907438913,\"initialblockdownload\":false,\
    //     \"chainwork\":\"00000000000000000000000000000000000000001930176b3488f68fcf0883d7\",\"size_on_disk\":371031289853,\"pruned\":false,\
    //     \"softforks\":{\"bip34\":{\"type\":\"buried\",\"active\":true,\"height\":227931},\"bip66\":{\"type\":\"buried\",\"active\":true,\"height\":363725},\
    //     \"bip65\":{\"type\":\"buried\",\"active\":true,\"height\":388381},\"csv\":{\"type\":\"buried\",\"active\":true,\"height\":419328},\
    //     \"segwit\":{\"type\":\"buried\",\"active\":true,\"height\":481824},\"warnings\":\"\"},\"error\":null,\"id\":\"curltest\"}";
    //     let rpc_result: RPCResult = serde_json::from_str(&data).unwrap();
    //     assert_eq!(rpc_result.result.chain, Chain::Test);
    //     // let testdummy = match rpc_result.result.softforks.get(&softforks::SoftForkNames::TestDummy).unwrap() {
    //     //     softforks::SoftForks::TestDummy(softforks::BIP9Info::Started(v)) => v,
    //     //     _ =>  panic!()
    //     // };
    //     // assert_eq!(
    //     //     testdummy,
    //     //     &StartedInfo{
    //     //         status: softforks::Bip9Status::Started,
    //     //         bit: softforks::BlockVersionFieldBit(28),
    //     //         start_time: 12345,
    //     //         timeout: 12345,
    //     //         since: 100,
    //     //         statistics: softforks::Statistics{period:1234, threshold:456,elapsed:999, count:12345, possible:true},
    //     //         active: true,
    //     //         r#type: softforks::SoftForkType::BIP9}
    //     // )
    // }
}
