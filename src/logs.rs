use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest, DescribeLogStreamsRequest,
    GetLogEventsRequest, LogStream, OutputLogEvent,
};
use std::default::Default;

pub async fn describe_log_groups(log_group_name_prefix: Option<String>) -> Vec<String> {
    let client = CloudWatchLogsClient::new(Region::ApNortheast1);

    let mut desc_log_groups_req: DescribeLogGroupsRequest = Default::default();
    desc_log_groups_req.log_group_name_prefix = log_group_name_prefix;
    let desc_log_groups_res = client
        .describe_log_groups(desc_log_groups_req)
        .await
        .unwrap();

    let log_groups = desc_log_groups_res.log_groups.unwrap();
    log_groups
        .into_iter()
        .map(|x| x.log_group_name.unwrap())
        .collect()
}

pub async fn describe_log_streams(log_group_name: String) -> Vec<LogStream> {
    let client = CloudWatchLogsClient::new(Region::ApNortheast1);

    let mut desc_log_streams_req: DescribeLogStreamsRequest = Default::default();
    desc_log_streams_req.log_group_name = log_group_name;
    let desc_log_streams_res = client
        .describe_log_streams(desc_log_streams_req)
        .await
        .unwrap();

    desc_log_streams_res.log_streams.unwrap()
}

pub async fn get_log_events(
    log_group_name: String,
    log_stream_name: String,
) -> Vec<OutputLogEvent> {
    let client = CloudWatchLogsClient::new(Region::ApNortheast1);

    let mut get_log_events_req: GetLogEventsRequest = Default::default();
    get_log_events_req.log_group_name = log_group_name;
    get_log_events_req.log_stream_name = log_stream_name;
    let get_log_events_res = client.get_log_events(get_log_events_req).await.unwrap();

    get_log_events_res.events.unwrap()
}
