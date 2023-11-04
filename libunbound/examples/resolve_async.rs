use libunbound::*;
use hickory_proto::rr::record_type::RecordType;
use hickory_proto::rr::DNSClass;

#[tokio::main]
async fn main() {
    let ctx = Context::new().expect("Context to be created");
    let ctx = ctx.into_async().unwrap();

    let result = ctx
        .resolve("_25._tcp.do.havedane.net", RecordType::TLSA, DNSClass::IN)
        .await
        .unwrap();
    println!("{result:#?}");

    let result = ctx
        .resolve("do.havedane.net", RecordType::MX, DNSClass::IN)
        .await
        .unwrap();
    println!("{result:#?}");

    println!("I am done");
}
