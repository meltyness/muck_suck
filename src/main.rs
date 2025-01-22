// use std::crypto::*; these crypto protocol
fn main() {
    println!("Hello, world!");
}

pub fn configure() -> msec_ent {

}

struct mac_pdu {
    // dst
    // src
    // pri
    // msdu
    // FCS
    // tag
}

struct msec_ent {
    // controlled ports listing
    //
    // 'common port' selection
    //
    // a mac address
    // map of secure channel identifiers to port identifiers
    //
    // kex_ent listing
    //
    // configuration regarding protection level used
    //   i.e., integrity enabled/disabled
    //   i.e., selected cipher suite (per port association?)
    //
    // peer listing
    //  authenticated and authorized peers, needed details or msec_ch
    //
    //
}

struct msec_ch { 
    // selected suite
    //  associated config info max SCs,SAKs
    //
    // rx channel
    //  two keys
    // tx channel
    //  a reference to one of the keys
    // 
}

impl msec_ch {
    pub fn swap_key(self: &mut Self) {
        // swap the tx channel key
    }
}

struct tx_req {

}

struct rx_noti {

}

struct ms_pdu {

}

impl msec_ent {
    pub fn decode_validate_pdu(data: &[u8]) -> ms_pdu {
        // uses some combination of validation and decryption
    }

    pub fn encode_protect_pdu(data: &ms_pdu) -> Vec<u8> {
        // uses encryption
    }
}

struct kex_ent {

}

impl kex_ent {
    pub fn layer_management(self: &mut Self, state: lm_state) {
    
    }
}
