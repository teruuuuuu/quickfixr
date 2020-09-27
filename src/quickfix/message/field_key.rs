#[macro_use]
use lazy_static::lazy_static;

pub const BEGIN_STRING_KEY: i32 = 8;
pub const BODY_LENGTH_KEY: i32 = 9;
pub const CHECKSUM_KEY: i32 = 10;
pub const CLORDID_KEY: i32 = 11;
pub const HANDLINST_KEY: i32 = 21;
pub const MSG_SEQ_NUM_KEY: i32 = 34;
pub const MSG_TYPE_KEY: i32 = 35;
pub const ORDER_QTY_KEY: i32 = 38;
pub const ORDER_TYPE_KEY: i32 = 40;
pub const SENDER_COMP_ID_KEY: i32 = 49;
pub const SENDING_TIME_KEY: i32 = 52;
pub const SIDE_KEY: i32 = 54;
pub const SYMBOL_KEY: i32 = 55;
pub const TARGET_CMP_ID_KEY: i32 = 56;
pub const TIME_IN_FORCE_KEY: i32 = 59;
pub const TRANSACT_TIME_KEY: i32 = 60;
pub const ENCRYPT_METHOD_KEY: i32 = 98;
pub const HEART_BEAT_INTERVAL_KEY: i32 = 108;
pub const RESET_SEQ_NUM_FLG_KEY: i32 = 141;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldKey(i32);

impl FieldKey {
    pub fn new(key: i32) -> FieldKey {
        FieldKey(key)
    }
    pub fn key_val(&self) -> i32 {
        self.0
    }

    pub fn to_string(&self) -> String {
        match self.0 {
            BEGIN_STRING_KEY => String::from("BeginString"),
            BODY_LENGTH_KEY => String::from("BodyLength"),
            CHECKSUM_KEY => String::from("Checksum"),
            CLORDID_KEY => String::from("ClOrdID"),
            HANDLINST_KEY => String::from("Handlnst"),
            MSG_SEQ_NUM_KEY => String::from("MsgSeqNum"),
            MSG_TYPE_KEY => String::from("MsgType"),
            ORDER_QTY_KEY => String::from("OrderQty"),
            ORDER_TYPE_KEY => String::from("OrderType"),
            SENDER_CMP_ID_KEY => String::from("SenderCmpId"),
            SENDING_TIME_KEY => String::from("SendingTime"),
            SIDE_KEY => String::from("Side"),
            SYMBOL_KEY => String::from("Symbol"),
            TARGET_CMP_ID_KEY => String::from("TargetCmpID"),
            TIME_IN_FORCE_KEY => String::from("TimeInForce"),
            TRANSACT_TIME_KEY => String::from("TransactTime"),
            ENCRYPT_METHOD_KEY => String::from("EncryptMethod"),
            HEART_BEAT_INTERVAL_KEY => String::from("HeartBeatInterval"),
            RESET_SEQ_NUM_FLG_KEY => String::from("ResetSeqNumFlg"),
            _ => String::from(""),
        }
    }
}

lazy_static! {
    pub static ref BEGIN_STRING: FieldKey = FieldKey(BEGIN_STRING_KEY);
    pub static ref BODY_LENGTH: FieldKey = FieldKey(BODY_LENGTH_KEY);
    pub static ref CHECKSUM: FieldKey = FieldKey(CHECKSUM_KEY);
    pub static ref CLORDID: FieldKey = FieldKey(CLORDID_KEY);
    pub static ref HANDLINST: FieldKey = FieldKey(HANDLINST_KEY);
    pub static ref MSG_SEQ_NUM: FieldKey = FieldKey(MSG_SEQ_NUM_KEY);
    pub static ref MSG_TYPE: FieldKey = FieldKey(MSG_TYPE_KEY);
    pub static ref ORDER_QTY: FieldKey = FieldKey(ORDER_QTY_KEY);
    pub static ref ORDER_TYPE: FieldKey = FieldKey(ORDER_TYPE_KEY);
    pub static ref SENDER_CMP_ID: FieldKey = FieldKey(SENDER_COMP_ID_KEY);
    pub static ref SENDING_TIME: FieldKey = FieldKey(SENDING_TIME_KEY);
    pub static ref SIDE: FieldKey = FieldKey(SIDE_KEY);
    pub static ref SYMBOL: FieldKey = FieldKey(SYMBOL_KEY);
    pub static ref TARGET_CMP_ID: FieldKey = FieldKey(TARGET_CMP_ID_KEY);
    pub static ref TIME_IN_FORCE: FieldKey = FieldKey(TIME_IN_FORCE_KEY);
    pub static ref TRANSACT_TIME: FieldKey = FieldKey(TRANSACT_TIME_KEY);
    pub static ref ENCRYPT_METHOD: FieldKey = FieldKey(ENCRYPT_METHOD_KEY);
    pub static ref HEART_BEAT_INTERVAL: FieldKey = FieldKey(HEART_BEAT_INTERVAL_KEY);
    pub static ref RESET_SEQ_NUM_FLG: FieldKey = FieldKey(RESET_SEQ_NUM_FLG_KEY);
}

#[test]
fn test_field_key() {
    assert_eq!(8, BEGIN_STRING.key_val());
    assert_eq!(9, BODY_LENGTH.key_val());
    assert_eq!(35, MSG_TYPE.key_val());
}
