use sae_library_logging::*;

#[test]
fn log4j_test() {
    let log = LoggingBuilder::default().add_log4j();
    log.error("Goes to stderr and file",Option::None);
    log.warn("Goes to stderr and file");
    log.info("Goes to stderr and file");
    log.debug("Goes to file only");
    log.trace("Goes to file only",Option::None);
}
