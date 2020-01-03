use chrono::prelude::*;
use icalendar::{Calendar, Class, Component, Event, EventStatus, Todo};
#[cfg(feature="parser")] use icalendar::parser;
use pretty_assertions::assert_eq;

const EXPECTED_CAL_CONTENT: &str = "\
BEGIN:VCALENDAR\r
VERSION:2.0\r
PRODID:ICALENDAR-RS\r
CALSCALE:GREGORIAN\r
BEGIN:VEVENT\r
CLASS:CONFIDENTIAL\r
DESCRIPTION:Description\r
DTEND:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DTSTART:20140708T071011Z\r
LOCATION:Somewhere\r
PRIORITY:10\r
STATUS:TENTATIVE\r
SUMMARY:summary\r
UID:euid\r
END:VEVENT\r
BEGIN:VTODO\r
COMPLETED:20140709T091011Z\r
DTSTAMP:20190307T181159\r
DUE:20140708T091011\r
PERCENT-COMPLETE:95\r
SUMMARY:A Todo\r
UID:todouid\r
END:VTODO\r
END:VCALENDAR\r
";

#[test]
fn test_calendar_to_string() {
    let mut calendar = Calendar::new();
    let cest_date = FixedOffset::east(2 * 3600)
        .ymd(2014, 7, 8)
        .and_hms(9, 10, 11);
    let utc_date = Utc.ymd(2014, 7, 9).and_hms(9, 10, 11);
    let event = Event::new()
        .status(EventStatus::Tentative)
        .starts(cest_date.with_timezone(&Utc))
        .ends(utc_date)
        .priority(11) // converted to 10
        .summary("summary")
        .description("Description")
        .location("Somewhere")
        .uid("euid")
        .class(Class::Confidential)
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(event);
    let todo = Todo::new()
        .percent_complete(95)
        .due(cest_date.naive_local())
        .completed(utc_date)
        .summary("A Todo")
        .uid("123e4567-e89b-12d3-a456-426655440000")
        .add_property("DTSTAMP", "20190307T181159")
        .done();
    calendar.push(todo);
    assert_eq!(calendar.to_string(), EXPECTED_CAL_CONTENT);
}

#[test]
#[cfg(feature="parser")]
fn test_string_to_calendar() {
    let cal = parser::parse(EXPECTED_CAL_CONTENT).unwrap();
    println!("{:#?}", cal);
}


#[test]
#[cfg(feature="parser")]
fn stringify_equality() {
    let cal1 = parser::parse(EXPECTED_CAL_CONTENT).unwrap();
    let cal2 = parser::parse(&cal1.to_string()).unwrap();

    assert_eq!(cal1.to_string(), cal2.to_string());
}

#[test]
#[cfg(feature="parser")]
fn parse_equality() {
    let cal1 = parser::parse(EXPECTED_CAL_CONTENT).unwrap();

    assert_eq!(cal1.to_string(), EXPECTED_CAL_CONTENT);
}

