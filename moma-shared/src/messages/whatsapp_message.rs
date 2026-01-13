use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
// pub struct WhatsAppMessage{
//     pub phone_number : String,
//     pub message : String
// }

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Sender {
    pub id: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub pushname: Option<String>,
    #[serde(rename = "type")]
    pub sender_type: Option<String>,
    pub is_contact_sync_completed: Option<u8>,
    pub text_status_last_update_time: Option<i64>,
    pub sync_to_addressbook: Option<bool>,
    pub formatted_name: Option<String>,
    pub is_me: Option<bool>,
    pub is_my_contact: Option<bool>,
    pub is_psa: Option<bool>,
    pub is_user: Option<bool>,
    pub is_wa_contact: Option<bool>
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaData {}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WppWhatsAppMessage {
    pub event: Option<String>,
    pub session: Option<String>,
    pub id: Option<String>,
    pub viewed: Option<bool>,
    pub body: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: Option<String>,
    pub t: Option<u64>,
    pub notify_name: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub ack: Option<u8>,
    pub invis: Option<bool>,
    pub is_new_msg: Option<bool>,
    pub star: Option<bool>,
    pub kic_notified: Option<bool>,
    pub recv_fresh: Option<bool>,
    pub is_from_template: Option<bool>,
    pub poll_invalidated: Option<bool>,
    pub is_sent_cag_poll_creation: Option<bool>,
    pub latest_edit_msg_key: Option<String>,
    pub latest_edit_sender_timestamp_ms: Option<String>,
    pub mentioned_jid_list: Option<Vec<String>>,
    pub group_mentions: Option<Vec<String>>,
    pub is_event_canceled: Option<bool>,
    pub event_invalidated: Option<bool>,
    pub is_vcard_over_mms_document: Option<bool>,
    pub is_forwarded: Option<bool>,
    pub has_reaction: Option<bool>,
    pub view_mode: Option<String>,
    pub product_header_image_rejected: Option<bool>,
    pub last_playback_progress: Option<u64>,
    pub is_dynamic_reply_buttons_msg: Option<bool>,
    pub is_carousel_card: Option<bool>,
    pub parent_msg_id: Option<String>,
    pub is_md_history_msg: Option<bool>,
    pub sticker_sent_ts: Option<u64>,
    pub is_avatar: Option<bool>,
    pub last_update_from_server_ts: Option<u64>,
    pub invoked_bot_wid: Option<String>,
    pub biz_bot_type: Option<String>,
    pub bot_response_target_id: Option<String>,
    pub bot_plugin_type: Option<String>,
    pub bot_plugin_reference_index: Option<u64>,
    pub bot_plugin_search_provider: Option<String>,
    pub bot_plugin_search_url: Option<String>,
    pub bot_plugin_search_query: Option<String>,
    pub bot_plugin_maybe_parent: Option<bool>,
    pub bot_reel_plugin_thumbnail_cdn_url: Option<String>,
    pub bot_msg_body_type: Option<String>,
    pub requires_direct_connection: Option<bool>,
    pub biz_content_placeholder_type: Option<String>,
    pub hosted_biz_enc_state_mismatch: Option<bool>,
    pub sender_or_recipient_account_type_hosted: Option<bool>,
    pub placeholder_created_when_account_is_hosted: Option<bool>,
    pub chat_id: Option<String>,
    pub from_me: Option<bool>,
    pub sender: Option<Sender>,
    pub timestamp: Option<u64>,
    pub content: Option<String>,
    pub is_group_msg: Option<bool>,
    pub media_data: Option<MediaData>,
}