use std::time::Duration;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0, space0},
    combinator::{eof, map_res, opt},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Lyric {
    pub timestamp: Duration,
    pub text: String,
}

fn parse_timestamp(input: &str) -> IResult<&str, Duration> {
    let (input, minutes) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, seconds) = map_res(digit1, |s: &str| s.parse::<u64>())(input)?;
    let (input, milliseconds) = opt(preceded(
        tag("."),
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    let total_milliseconds = minutes * 60 * 1000 + seconds * 1000 + milliseconds.unwrap_or(0);
    let timestamp = Duration::from_millis(total_milliseconds);
    Ok((input, timestamp))
}

fn parse_lyric_line(input: &str) -> IResult<&str, Lyric> {
    let (input, (timestamp, text)) = pair(
        delimited(tag("["), parse_timestamp, tag("]")),
        preceded(space0, take_until("\n")),
    )(input)?;

    Ok((
        input,
        Lyric {
            timestamp,
            text: text.trim().to_string(),
        },
    ))
}

pub fn parse_lyrics(input: &str) -> IResult<&str, Vec<Lyric>> {
    let lyric_line = terminated(parse_lyric_line, multispace0);
    let (input, lyrics) = many0(lyric_line)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, lyrics))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp("01:23"), Ok(("", Duration::from_secs(83))));
        assert_eq!(
            parse_timestamp("01:23.456"),
            Ok(("", Duration::from_millis(83456)))
        );
    }

    #[test]
    fn test_parse_lyric_line() {
        assert_eq!(
            parse_lyric_line("[01:23] Hello, world!\n"),
            Ok((
                "\n",
                Lyric {
                    timestamp: Duration::from_secs(83),
                    text: "Hello, world!".to_string()
                }
            ))
        );
    }

    #[test]
    fn test_parse_lyrics() {
        let input = "\
            [00:05] First line
            [00:10] Second line
            ";
        let expected = vec![
            Lyric {
                timestamp: Duration::from_secs(5),
                text: "First line".to_string(),
            },
            Lyric {
                timestamp: Duration::from_secs(10),
                text: "Second line".to_string(),
            },
        ];
        assert_eq!(parse_lyrics(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_lyrics_with_milliseconds() {
        let input = "\
            [00:05.500] First line
            [00:10.250] Second line
            ";
        let expected = vec![
            Lyric {
                timestamp: Duration::from_millis(5500),
                text: "First line".to_string(),
            },
            Lyric {
                timestamp: Duration::from_millis(10250),
                text: "Second line".to_string(),
            },
        ];
        assert_eq!(parse_lyrics(input), Ok(("", expected)));
    }
}
