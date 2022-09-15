use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationMessage {
    #[serde(rename="FromUserName")]
    pub source: String,
    #[serde(rename="ToUserName")]
    pub target: String,
    #[serde(rename="CreateTime")]
    pub create_time: i64,
    #[serde(rename="MsgId")]
    pub id: i64,
    #[serde(rename="Location_X")]
    pub location_x: f64,
    #[serde(rename="Location_Y")]
    pub location_y: f64,
    #[serde(rename="Scale")]
    pub scale: usize,
    #[serde(rename="Label")]
    pub label: String,
}

#[cfg(test)]
mod tests {
    use crate::XmlMessageParser;
    use super::LocationMessage;

    #[test]
    fn test_from_xml() {
        let xml = "<xml>\
        <ToUserName><![CDATA[toUser]]></ToUserName>\
        <FromUserName><![CDATA[fromUser]]></FromUserName>\
        <CreateTime>1348831860</CreateTime>\
        <MsgType><![CDATA[location]]></MsgType>\
        <Location_X>23.134521</Location_X>\
        <Location_Y>113.358803</Location_Y>
        <Scale>20</Scale>\
        <Label><![CDATA[位置信息]]></Label>
        <MsgId>1234567890123456</MsgId>\
        </xml>";
        let msg = LocationMessage::from_xml(xml).unwrap();

        assert_eq!("fromUser", &msg.source);
        assert_eq!("toUser", &msg.target);
        assert_eq!(1234567890123456, msg.id);
        assert_eq!(23, msg.location_x as usize);
        assert_eq!(113, msg.location_y as usize);
        assert_eq!(20, msg.scale);
        assert_eq!("位置信息", &msg.label);
    }
}
