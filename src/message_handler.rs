use crate::message::{ProveSpecMessage, ProverMessage};

pub fn handle(message: ProverMessage) {
    println!("subscribe handle is : id{:?} ", message);
}

// pub async fn handle_prover_spec_msg<'a>(msg : ProveSpecMessage<'a>) {
//     println!("handle pool go message{:?}", msg);


//     println!("handle prover spec msg: id{:?} msg{:?} ", prover_id, prov_msg);
// }

