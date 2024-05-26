use crate::data::commands::CommandData;
use crate::data::parser::verify_le_u8;
use crate::data::{ParserResult, Span};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::number::complete::{le_u16, le_u32, le_u8};
use nom::sequence::tuple;
use nom_tracable::tracable_parser;

#[derive(Debug)]
pub struct BuildSquad {
    pub player_id: u8,
    pub pgbid: u32,
    pub source_identifier: u16,
}

impl BuildSquad {
    #[tracable_parser]
    pub fn parse_command(input: Span) -> ParserResult<CommandData> {
        map(
            tuple((
                take(2u32),
                Self::verify_action_type,
                le_u8,
                take(26u32),
                le_u16,
                take(3u32),
                le_u32,
            )),
            |(_, _, player_id, _, source_identifier, _, pgbid)| {
                CommandData::BuildSquad(BuildSquad {
                    player_id,
                    pgbid,
                    source_identifier,
                })
            },
        )(input)
    }

    fn verify_action_type(input: Span) -> ParserResult<u8> {
        verify_le_u8(3u8)(input)
    }
}
