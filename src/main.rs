use mavlink::ardupilotmega::MavMessage;
use mavlink::MavFrame;
use mavlink::MavHeader;

fn main() {
    let buf: &mut [u8; 255] = &mut [0; 255];

    let mavlink_message = mavlink_message();
    let mavlink_frame = new(mavlink_message);

    let _len = mavlink_frame.ser(buf);
    println!("{:#?}", mavlink_frame);

    let parsed_mavlink_frame =
        MavFrame::<mavlink::ardupilotmega::MavMessage>::deser(mavlink::MavlinkVersion::V2, buf)
            .unwrap();

    println!("{:#?}", parsed_mavlink_frame);
}

fn mavlink_message() -> mavlink::ardupilotmega::MavMessage {
    mavlink::ardupilotmega::MavMessage::LINK_NODE_STATUS(
        mavlink::ardupilotmega::LINK_NODE_STATUS_DATA {
            timestamp: 92197916,
            tx_rate: 53,
            rx_rate: 20,
            messages_sent: 47,
            messages_received: 21,
            messages_lost: 0,
            rx_parse_err: 0,
            tx_overflows: 0,
            rx_overflows: 0,
            tx_buf: 0,
            rx_buf: 0,
        },
    )
}

fn new(msg: MavMessage) -> MavFrame<MavMessage> {
    MavFrame {
        header: MavHeader {
            system_id: 1,
            component_id: 1,
            sequence: 84,
        },
        msg,
        protocol_version: mavlink::MavlinkVersion::V2,
    }
}
