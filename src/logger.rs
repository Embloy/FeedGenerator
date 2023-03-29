use crate::models::FeedLog;

pub(crate) fn add_to_feed_log(_log: FeedLog, _key: &str, _status: u32) {
    //key describes what the log resembles in the context of its source (so where it was produced)
    //status describes whether (and which) a log describes an erro.

    //TODO: Make db entry
}