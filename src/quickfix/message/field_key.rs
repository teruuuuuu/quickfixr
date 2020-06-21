#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldKey(i32);

impl FieldKey {
    pub fn new(key: i32) -> FieldKey {
        FieldKey(key)
    }
    pub fn begin_string() -> FieldKey {
        FieldKey(8)
    }
    pub fn body_length() -> FieldKey {
        FieldKey(9)
    }
    pub fn cl_ord_id() -> FieldKey {
        FieldKey(11)
    }
    pub fn handl_inst() -> FieldKey {
        FieldKey(21)
    }
    pub fn msg_seq_num() -> FieldKey {
        FieldKey(34)
    }
    pub fn msg_type() -> FieldKey {
        FieldKey(35)
    }
    pub fn order_qty() -> FieldKey {
        FieldKey(38)
    }
    pub fn order_type() -> FieldKey {
        FieldKey(40)
    }
    pub fn sender_cmp_id() -> FieldKey {
        FieldKey(49)
    }
    pub fn sending_time() -> FieldKey {
        FieldKey(52)
    }
    pub fn side() -> FieldKey {
        FieldKey(54)
    }
    pub fn symbol() -> FieldKey {
        FieldKey(55)
    }
    pub fn target_cmp_id() -> FieldKey {
        FieldKey(56)
    }
    pub fn time_in_force() -> FieldKey {
        FieldKey(59)
    }
    pub fn transact_time() -> FieldKey {
        FieldKey(60)
    }

    pub fn encrypt_method() -> FieldKey {
        FieldKey(98)
    }
    pub fn heart_beat_interval() -> FieldKey {
        FieldKey(108)
    }
    pub fn checksum() -> FieldKey {
        FieldKey(10)
    }

    pub fn key_val(&self) -> i32 {
        self.0
    }

    pub fn to_string(&self) -> String {
        match self {
            FieldKey(8) => String::from("BeginString"),
            FieldKey(9) => String::from("BodyLength"),
            FieldKey(10) => String::from("Checksum"),
            FieldKey(11) => String::from("ClOrdID"),
            FieldKey(21) => String::from("ClOrdID"),
            FieldKey(34) => String::from("MsgSeqNum"),
            FieldKey(35) => String::from("MsgType"),
            FieldKey(38) => String::from("OrderQty"),
            FieldKey(40) => String::from("OrderType"),
            FieldKey(49) => String::from("SenderCmpId"),
            FieldKey(52) => String::from("SendingTime"),
            FieldKey(54) => String::from("Side"),
            FieldKey(55) => String::from("Symbol"),
            FieldKey(56) => String::from("TargetCmpID"),
            FieldKey(59) => String::from("TimeInForce"),
            FieldKey(60) => String::from("TransactTime"),
            FieldKey(98) => String::from("EncryptMethod"),
            FieldKey(108) => String::from("HeartBeatInterval"),
            _ => String::from(""),
        }
    }
}

#[test]
fn test_field_key() {
    assert_eq!(8, FieldKey::begin_string().key_val());
    assert_eq!(9, FieldKey::body_length().key_val());
    assert_eq!(35, FieldKey::msg_type().key_val());
}
