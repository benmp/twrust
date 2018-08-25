#![allow(dead_code)]

use std::str;

const PRIVMSG: &'static str = "PRIVMSG";
const BADGES: &'static str = "@badges=";

#[derive(Debug)]
struct ParseResult {
    privmsg: String,
    badgestr: String,
    badges: Vec<Badge>,
}

struct ParseResultBuilder {
    ogmsg: String,
    tokens: Vec<String>,
    privmsg: String,
    badgestr: String,
    badges: Vec<Badge>,
}

#[derive(Debug, PartialEq)]
struct Badge {
    name: BadgeType,
    variant: u8,
}

#[derive(Debug, PartialEq)]
enum BadgeType {
    Moderator,
    Subscriber,
    Turbo,
}

impl str::FromStr for BadgeType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "moderator" => Ok(BadgeType::Moderator),
            "subscriber" => Ok(BadgeType::Subscriber),
            "turbo" => Ok(BadgeType::Turbo),
            _ => Err("not a valid value"),
        }
    }
}

impl ParseResultBuilder {

    fn new(msg: &str) -> ParseResultBuilder {
        ParseResultBuilder {
            ogmsg: String::from(msg),
            tokens: String::from(msg)
                .split(" ")
                .map(|x| String::from(x.trim()))
                .collect(),
            privmsg: String::from(""),
            badgestr: String::from(""),
            badges: Vec::new(),
        }
    }

    fn parse_privmsg(mut self) -> ParseResultBuilder {
        let mut found: bool = false;

        for (index, token) in self.tokens.iter().enumerate() {
            if found {
                self.privmsg.push_str(token);
                if index < self.tokens.len() - 1 {
                    self.privmsg.push_str(" ");
                }
            }

            if token == PRIVMSG {
                found = true;
            }
        }

        self
    }

    fn parse_badges(mut self) -> ParseResultBuilder {
        if self.tokens.len() > 0 && self.tokens[0].starts_with(BADGES) {
            self.badgestr = self.tokens[0].clone();

            let badges: Vec<&str> = self.badgestr.trim_left_matches(BADGES).split(";color=").collect();

            for badge in badges[0].split(",") {
                if badge.starts_with("color=") {
                    break;
                }
                let kvp: Vec<&str> = badge.split("/").collect();
                self.badges.push(Badge {
                    name: kvp[0].parse().expect("unkown new subscriber type"),
                    variant: kvp[1].parse::<u8>().expect("badge variant is unexpected"),
                });
            }
        }

        self
    }

    fn build(self) -> ParseResult {
        ParseResult {
            privmsg: self.privmsg,
            badges: self.badges,
            badgestr: self.badgestr
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_privmsg() {
        assert_eq!(
            String::from("#tsm_myth :@saucymike01 obv not banned"),
            ParseResultBuilder::new(MSG).parse_privmsg().build().privmsg
        );
    }

    #[test]
    fn parse_badge_enum() {
        let badge_type: BadgeType = "moderator".parse().unwrap();
        assert_eq!(BadgeType::Moderator, badge_type);
    }

    #[test]
    fn parse_badges() {
        assert_eq!(
            vec![
                Badge {
                    name: BadgeType::Moderator,
                    variant: 1,
                },
                Badge {
                    name: BadgeType::Subscriber,
                    variant: 6,
                },
                Badge {
                    name: BadgeType::Turbo,
                    variant: 1,
                },
            ],
            ParseResultBuilder::new(MSG).parse_badges().badges
        );
    }

    const MSG: &'static str = "@badges=moderator/1,subscriber/6,turbo/1;color=#CC008D;display-name=KushyLife;emotes=;id=4fc69423-4845-483d-93e5-c193be554965;mod=1;room-id=110690086;subscriber=1;tmi-sent-ts=1531789918256;turbo=1;user-id=37734410;user-type=mod :kushylife!kushylife@kushylife.tmi.twitch.tv PRIVMSG #tsm_myth :@saucymike01 obv not banned";

}
