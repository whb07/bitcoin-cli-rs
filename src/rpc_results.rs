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
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "lowercase")]
    #[allow(non_camel_case_types)]
    enum BIP9Status {
        Defined,
        Started,
        Locked_In,
        Active,
        Failed,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct BIP9Stats {
        period: usize,
        threshold: usize,
        elapsed: usize,
        count: usize,
        possible: bool,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "lowercase")]
    enum SoftForkType {
        Buried,
        BIP9,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BIP9StartedInfo {
        status: BIP9Status,
        bit: u8,
        start_time: u64,
        timeout: u64,
        since: u64,
        statistics: BIP9Stats,
        active: bool,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BIP9ActiveInfo {
        status: BIP9Status,
        start_time: u64,
        timeout: u64,
        since: u64,
        height: u64,
        active: bool,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BIP9OthersInfo {
        status: BIP9Status,
        start_time: u64,
        timeout: u64,
        since: u64,
        active: bool,
        r#type: SoftForkType,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SoftForkInfo {
        height: usize,
        active: bool,
        r#type: SoftForkType,
    }

    
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "lowercase")]
    pub enum BIP9Info {
        BIP9ActiveInfo(BIP9ActiveInfo),
        BIP9OthersInfo(BIP9OthersInfo),
        BIP9StartedInfo(BIP9StartedInfo),
    }
    // #[derive(Serialize, Deserialize, Debug)]
    // #[serde(rename_all = "lowercase", untagged)]
    // pub enum SoftFork {
    //     Fork(SoftForkInfo),
    //     BIP9(BIP9Info),
    // }
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    // softforks: HashMap<softforks::SoftForks, softforks::SoftFork>,
    warnings: String,
    #[serde(flatten)]
    pruned_info: HashMap<PrunedInfoKey, PrunedInfoValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RPCResult {
    result: BlockchainInfo,
}

pub fn hello(data: &str) -> RPCResult {
    serde_json::from_str(data).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u8,
}

#[cfg(test)]
mod tests {
    use super::{BlockchainInfo, Chain, RPCResult, PrunedInfoValue, PrunedInfoKey, softforks};
    use serde_json::json;

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
    // #[test]
    // fn proper_result() {
        // let data = r#"{
        //     "result": {
        //       "chain": "main",
        //       "blocks": 669547,
        //       "headers": 669547,
        //       "bestblockhash": "000000000000000000054b03be7c1544acc2ed8a4eb2035c8e416ff2a0a02921",
        //       "difficulty": 21434395961348.92,
        //       "mediantime": 1612705006,
        //       "verificationprogress": 0.99999682277192,
        //       "initialblockdownload": false,
        //       "chainwork": "0000000000000000000000000000000000000000191dbd2e0effa46508000d04",
        //       "size_on_disk": 370655486372,
        //       "pruned": true,
        //       "pruneheight": 666,
        //       "softforks": {
        //         "bip34": { "type": "buried", "active": true, "height": 227931 },
        //         "bip66": { "type": "buried", "active": true, "height": 363725 },
        //         "bip65": { "type": "buried", "active": true, "height": 388381 },
        //         "csv": { "type": "buried", "active": true, "height": 419328 },
        //         "segwit": { "type": "buried", "active": true, "height": 481824 },
        //         "bip9": { "type": "bip9": {
        //             "period": 12345,
        //             "threshold": 1234525,
        //             "elapsed": 55555523,
        //             "count": 235252535235,
        //             "possible": true,
        //         },
        //         "active": true, "height": 481824 }
        //       },
        //       "warnings": ""
        //     },
        //     "error": null,
        //     "id": "curltest"
        //   }"#;
    //     let resultoutput: RPCResult = serde_json::from_str(data).unwrap();
    //     assert_eq!("main", json!(resultoutput.result.chain));
    //     assert_eq!(370655486372, resultoutput.result.size_on_disk);
    //     assert_eq!( PrunedInfoValue::PruneHeight(666), resultoutput.result.pruned_info[&PrunedInfoKey::PruneHeight]);
    // }
    #[test]
    fn serialized_bip34() {
        let data = r#"{ softforks: { bip34: { type: \"buried\", active: true, height: 227931 } } }"#;
        // assert_eq!("main", json!(Chain::Main));
        // assert_eq!("test", json!(Chain::Test));
        // assert_eq!("signet", json!(Chain::Signet));
        let fork = serde_json::from_str::<softforks::SoftForks>(data).unwrap();
        // assert_eq!("buried", fork) ;
        // assert_eq!(softforks::SoftForks::Bip34(val), );
    }
}
