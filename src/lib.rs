mod content_parsers;

#[cfg(test)]
mod test {
    use super::*;
    use content_parsers::{java_parser, timestamp, verbosity};

    #[test]
    fn test_java_parse() {
        let log_line = "2020-03-11 07:19:21.542 [kafka-admin-client-thread | admin-adminClient] [] NetworkClient [WARN] [AdminClient clientId=admin-adminClient] Error connecting to node ltx1-app2113.stg.linkedin.com:16637 (id: 2113 rack: 405)";
        let parsed_log = match java_parser::parse_java(&log_line) {
            Ok(log) => log,
            Err(err) => panic!(err),
        };
        assert_eq!(parsed_log.timestamp, timestamp::Timestamp::new("2020", "3", "11", "7", "19", "21", "542"));
        assert_eq!(parsed_log.thread, "kafka-admin-client-thread | admin-adminClient");
        assert_eq!(parsed_log.class, "NetworkClient");
        assert_eq!(parsed_log.verbosity, verbosity::Verbosity::Warn);
        assert_eq!(parsed_log.content, "Error connecting to node ltx1-app2113.stg.linkedin.com:16637 (id: 2113 rack: 405)");
    }
}