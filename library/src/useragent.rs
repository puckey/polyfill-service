
#[must_use] pub fn useragent(ua: &str) -> [String; 4] {
    let family = "Other".to_owned();
    let major = "0".to_owned();
    let minor = "0".to_owned();
    let patch = "0".to_owned();
    if let Some(result) = crate::regex_cache::cached_regex(r"Opera\/9\.80 \(.+(Opera Mini)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                Into::<&str>::into(r).to_string()
            },
            None => {
                "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Opera\/9\.80 \(.+(Opera Mini)\/(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                Into::<&str>::into(r).to_string()
            },
            None => {
                "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/525\.18(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "3".to_owned();
        let minor="1".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/528\.18(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "4".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/531\.21(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "4".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/532\.9(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "4".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/532\+").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "5".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/533\.17(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "5".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/534\.12(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "5".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/534\.46(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "5".to_owned();
        let minor="1".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/536\.26(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "6".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/537\.51(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "7".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/600\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "8".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/601\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "9".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/601\.5(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "9".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/602\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "10".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/602\.2(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "10".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/602\.3(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "10".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/602\.4(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "10".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/603\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "10".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/603\.2(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "10".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/604\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "11".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/604\.2(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "11".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/604\.3(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "11".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/604\.5(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "11".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/605\.1(?:\.\d+|) \(KHTML, like Gecko\) Version\/(\d+)\.?(\d+)?\.?(\d+)?.+?Mobile\/\w+\s(Safari)").captures(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                Into::<&str>::into(r).to_string()
            },
            None => {
                "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    // The OS version may have a patch component ("OS 17_4_1") — accept it,
    // otherwise in-app webviews on patch-level iOS releases fall through to
    // the WebKit-version fallback table, which tops out around iOS 11.
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+OS (\d+)_(\d+)(?:_\d+)? like Mac OS X\) AppleWebKit\/605\.1(?:\.\d+|) \(KHTML, like Gecko\) Mobile\/\w+").captures(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/605\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "11".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/606\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "12".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/607\.1(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "12".to_owned();
        let minor="1".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+AppleWebKit\/608\.2(?:\.\d+|)").is_match(ua) {
        let family = "Mobile Safari/WKWebView".to_owned();
        let major = "13".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MQQBrowser\/Mini)(?:(\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = "QQ Browser Mini".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MQQBrowser)(?:\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = "QQ Browser Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(QQBrowser)(?:\/(\d+)(?:\.(\d+)\.(\d+)(?:\.(\d+)|)|)|)").captures(ua) {
        let family = "QQ Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(ESPN)[%20| ]+Radio\/(\d+)\.(\d+)\.(\d+) CFNetwork").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Antenna)\/(\d+) CFNetwork").captures(ua) {
        let family = "AntennaPod".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(TopPodcasts)Pro\/(\d+) CFNetwork").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MusicDownloader)Lite\/(\d+)\.(\d+)\.(\d+) CFNetwork").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(.*)-iPad\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)(?:\.(\d+)|) CFNetwork").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(.*)-iPhone\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)(?:\.(\d+)|) CFNetwork").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(.*)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)(?:\.(\d+)|) CFNetwork").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(espn\.go)").is_match(ua) {
        let family = "ESPN".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(espnradio\.com)").is_match(ua) {
        let family = "ESPN".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"ESPN APP$").is_match(ua) {
        let family = "ESPN".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(audioboom\.com)").is_match(ua) {
        let family = "AudioBoom".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r" (Rivo) RHYTHM").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(CFNetwork)(?:\/(\d+)\.(\d+)(?:\.(\d+)|)|)").captures(ua) {
        let family = "CFNetwork".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Pingdom\.com_bot_version_)(\d+)\.(\d+)").captures(ua) {
        let family = "PingdomBot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(PingdomTMS)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "PingdomBot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r" (PTST)\/(\d+)(?:\.(\d+)|)$").captures(ua) {
        let family = "WebPageTest.org bot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"X11; (Datanyze); Linux").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(NewRelicPinger)\/(\d+)\.(\d+)").captures(ua) {
        let family = "NewRelicPingerBot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Tableau)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Tableau".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Salesforce)(?:.)\/(\d+)\.(\d?)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(\(StatusCake\))").is_match(ua) {
        let family = "StatusCakeBot".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(facebookexternalhit)\/(\d+)\.(\d+)").captures(ua) {
        let family = "FacebookBot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Google.*\/\+\/web\/snippet").is_match(ua) {
        let family = "GooglePlusBot".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"via ggpht\.com GoogleImageProxy").is_match(ua) {
        let family = "GmailImageProxy".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"YahooMailProxy; https:\/\/help\.yahoo\.com\/kb\/yahoo-mail-proxy-SLN28749\.html").is_match(ua) {
        let family = "YahooMailProxy".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Twitterbot)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Twitterbot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\/((?:Ant-|)Nutch|[A-z]+[Bb]ot|[A-z]+[Ss]pider|Axtaris|fetchurl|Isara|ShopSalad|Tailsweep)[ \-](\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\b(008|Altresium|Argus|BaiduMobaider|BoardReader|DNSGroup|DataparkSearch|EDI|Goodzer|Grub|INGRID|Infohelfer|LinkedInBot|LOOQ|Nutch|OgScrper|PathDefender|Peew|PostPost|Steeler|Twitterbot|VSE|WebCrunch|WebZIP|Y!J-BR[A-Z]|YahooSeeker|envolk|sproose|wminer)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MSIE) (\d+)\.(\d+)([a-z]\d|[a-z]|);.* MSIECrawler").captures(ua) {
        let family = "MSIECrawler".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(DAVdroid)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Google-HTTP-Java-Client|Apache-HttpClient|Go-http-client|scalaj-http|http%20client|Python-urllib|HttpMonitor|TLSProber|WinHTTP|JNLP|okhttp|aihttp|reqwest|axios|unirest-(?:java|python|ruby|nodejs|php|net))(?:[ /](\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Pinterest(?:bot|))\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)[;\s(]+\+https:\/\/www.pinterest.com\/bot.html").captures(ua) {
        let family = "Pinterestbot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(CSimpleSpider|Cityreview Robot|CrawlDaddy|CrawlFire|Finderbots|Index crawler|Job Roboter|KiwiStatus Spider|Lijit Crawler|QuerySeekerSpider|ScollSpider|Trends Crawler|USyd-NLP-Spider|SiteCat Webbot|BotName\/\$BotVersion|123metaspider-Bot|1470\.net crawler|50\.nu|8bo Crawler Bot|Aboundex|Accoona-[A-z]{1,30}-Agent|AdsBot-Google(?:-[a-z]{1,30}|)|altavista|AppEngine-Google|archive.{0,30}\.org_bot|archiver|Ask Jeeves|[Bb]ai[Dd]u[Ss]pider(?:-[A-Za-z]{1,30})(?:-[A-Za-z]{1,30}|)|bingbot|BingPreview|blitzbot|BlogBridge|Bloglovin|BoardReader Blog Indexer|BoardReader Favicon Fetcher|boitho.com-dc|BotSeer|BUbiNG|\b\w{0,30}favicon\w{0,30}\b|\bYeti(?:-[a-z]{1,30}|)|Catchpoint(?: bot|)|[Cc]harlotte|Checklinks|clumboot|Comodo HTTP\(S\) Crawler|Comodo-Webinspector-Crawler|ConveraCrawler|CRAWL-E|CrawlConvera|Daumoa(?:-feedfetcher|)|Feed Seeker Bot|Feedbin|findlinks|Flamingo_SearchEngine|FollowSite Bot|furlbot|Genieo|gigabot|GomezAgent|gonzo1|(?:[a-zA-Z]{1,30}-|)Googlebot(?:-[a-zA-Z]{1,30}|)|Google SketchUp|grub-client|gsa-crawler|heritrix|HiddenMarket|holmes|HooWWWer|htdig|ia_archiver|ICC-Crawler|Icarus6j|ichiro(?:\/mobile|)|IconSurf|IlTrovatore(?:-Setaccio|)|InfuzApp|Innovazion Crawler|InternetArchive|IP2[a-z]{1,30}Bot|jbot\b|KaloogaBot|Kraken|Kurzor|larbin|LEIA|LesnikBot|Linguee Bot|LinkAider|LinkedInBot|Lite Bot|Llaut|lycos|Mail\.RU_Bot|masscan|masidani_bot|Mediapartners-Google|Microsoft .{0,30} Bot|mogimogi|mozDex|MJ12bot|msnbot(?:-media {0,2}|)|msrbot|Mtps Feed Aggregation System|netresearch|Netvibes|NewsGator[^/]{0,30}|^NING|Nutch[^/]{0,30}|Nymesis|ObjectsSearch|OgScrper|Orbiter|OOZBOT|PagePeeker|PagesInventory|PaxleFramework|Peeplo Screenshot Bot|PlantyNet_WebRobot|Pompos|Qwantify|Read%20Later|Reaper|RedCarpet|Retreiver|Riddler|Rival IQ|scooter|Scrapy|Scrubby|searchsight|seekbot|semanticdiscovery|SemrushBot|Simpy|SimplePie|SEOstats|SimpleRSS|SiteCon|Slackbot-LinkExpanding|Slack-ImgProxy|Slurp|snappy|Speedy Spider|Squrl Java|Stringer|TheUsefulbot|ThumbShotsBot|Thumbshots\.ru|Tiny Tiny RSS|Twitterbot|WhatsApp|URL2PNG|Vagabondo|VoilaBot|^vortex|Votay bot|^voyager|WASALive.Bot|Web-sniffer|WebThumb|WeSEE:[A-z]{1,30}|WhatWeb|WIRE|WordPress|Wotbox|www\.almaden\.ibm\.com|Xenu(?:.s|) Link Sleuth|Xerka [A-z]{1,30}Bot|yacy(?:bot|)|YahooSeeker|Yahoo! Slurp|Yandex\w{1,30}|YodaoBot(?:-[A-z]{1,30}|)|YottaaMonitor|Yowedo|^Zao|^Zao-Crawler|ZeBot_www\.ze\.bz|ZooShot|ZyBorg)(?:[ /]v?(\d+)(?:\.(\d+)(?:\.(\d+)|)|)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\b(Boto3?|JetS3t|aws-(?:cli|sdk-(?:cpp|go|java|nodejs|ruby2?|dotnet-(?:\d{1,2}|core)))|s3fs)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\[(FBAN\/MessengerForiOS|FB_IAB\/MESSENGER);FBAV\/(\d+)(?:\.(\d+)(?:\.(\d+)|)|)").captures(ua) {
        let family = "Facebook Messenger".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\[FB.*;(FBAV)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = "Facebook".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"\[FB.*;").is_match(ua) {
        let family = "Facebook".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(?:\/[A-Za-z0-9\.]+|) {0,5}([A-Za-z0-9 \-_\!\[\]:]{0,50}(?:[Aa]rchiver|[Ii]ndexer|[Ss]craper|[Bb]ot|[Ss]pider|[Cc]rawl[a-z]{0,50}))[/ ](\d+)(?:\.(\d+)(?:\.(\d+)|)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"((?:[A-Za-z][A-Za-z0-9 -]{0,50}|)[^C][^Uu][Bb]ot)\b(?:(?:[ /]| v)(\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"((?:[A-z0-9]{1,50}|[A-z\-]{1,50} ?|)(?: the |)(?:[Ss][Pp][Ii][Dd][Ee][Rr]|[Ss]crape|[Cc][Rr][Aa][Ww][Ll])[A-z0-9]{0,50})(?:(?:[ /]| v)(\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(HbbTV)\/(\d+)\.(\d+)\.(\d+) \(").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Chimera|SeaMonkey|Camino|Waterfox)\/(\d+)\.(\d+)\.?([ab]?\d+[a-z]*|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(SailfishBrowser)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Sailfish Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\[(Pinterest)\/[^\]]+\]").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Pinterest)(?: for Android(?: Tablet|)|)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Mozilla.*Mobile.*(Instagram).(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Mozilla.*Mobile.*(Flipboard).(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Mozilla.*Mobile.*(Flipboard-Briefing).(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Mozilla.*Mobile.*(Onefootball)\/Android.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Snapchat)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+) Basilisk\/(\d+)").captures(ua) {
        let family = "Basilisk".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(PaleMoon)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Pale Moon".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Fennec)\/(\d+)\.(\d+)\.?([ab]?\d+[a-z]*)").captures(ua) {
        let family = "Firefox Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Fennec)\/(\d+)\.(\d+)(pre)").captures(ua) {
        let family = "Firefox Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Fennec)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Firefox Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(?:Mobile|Tablet);.*(Firefox)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Firefox Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Namoroka|Shiretoko|Minefield)\/(\d+)\.(\d+)\.(\d+(?:pre|))").captures(ua) {
        let family = "Firefox ($1)".replace("$1", result.get(1).unwrap().into());
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+)(a\d+[a-z]*)").captures(ua) {
        let family = "Firefox Alpha".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+)(b\d+[a-z]*)").captures(ua) {
        let family = "Firefox Beta".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)-(?:\d+\.\d+|)\/(\d+)\.(\d+)(a\d+[a-z]*)").captures(ua) {
        let family = "Firefox Alpha".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)-(?:\d+\.\d+|)\/(\d+)\.(\d+)(b\d+[a-z]*)").captures(ua) {
        let family = "Firefox Beta".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Namoroka|Shiretoko|Minefield)\/(\d+)\.(\d+)([ab]\d+[a-z]*|)").captures(ua) {
        let family = "Firefox ($1)".replace("$1", result.get(1).unwrap().into());
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox).*Tablet browser (\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "MicroB".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MozillaDeveloperPreview)\/(\d+)\.(\d+)([ab]\d+[a-z]*|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(FxiOS)\/(\d+)\.(\d+)(\.(\d+)|)(\.(\d+)|)").captures(ua) {
        let family = "Firefox iOS".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Flock)\/(\d+)\.(\d+)(b\d+?)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(RockMelt)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Navigator)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Netscape".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Navigator)\/(\d+)\.(\d+)([ab]\d+)").captures(ua) {
        let family = "Netscape".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Netscape6)\/(\d+)\.(\d+)\.?([ab]?\d+|)").captures(ua) {
        let family = "Netscape".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MyIBrow)\/(\d+)\.(\d+)").captures(ua) {
        let family = "My Internet Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(UC? ?Browser|UCWEB|U3)[ /]?(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "UC Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Opera Tablet).*Version\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Opera Mini)(?:\/att|)\/?(\d+|)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Opera)\/.+Opera Mobi.+Version\/(\d+)\.(\d+)").captures(ua) {
        let family = "Opera Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Opera)\/(\d+)\.(\d+).+Opera Mobi").captures(ua) {
        let family = "Opera Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Opera Mobi.+(Opera)(?:\/|\s+)(\d+)\.(\d+)").captures(ua) {
        let family = "Opera Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Opera Mobi").is_match(ua) {
        let family = "Opera Mobile".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Opera)\/9.80.*Version\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(?:Mobile Safari).*(OPR)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Opera Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(?:Chrome).*(OPR)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Opera".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Coast)\/(\d+).(\d+).(\d+)").captures(ua) {
        let family = "Opera Coast".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(OPiOS)\/(\d+).(\d+).(\d+)").captures(ua) {
        let family = "Opera Mini".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Chrome\/.+( MMS)\/(\d+).(\d+).(\d+)").captures(ua) {
        let family = "Opera Neon".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(hpw|web)OS\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "webOS Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"(luakit)").is_match(ua) {
        let family = "LuaKit".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Snowshoe)\/(\d+)\.(\d+).(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Gecko\/\d+ (Lightning)\/(\d+)\.(\d+)\.?((?:[ab]?\d+[a-z]*)|(?:\d*))").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+)\.(\d+(?:pre|)) \(Swiftfox\)").captures(ua) {
        let family = "Swiftfox".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+)([ab]\d+[a-z]*|) \(Swiftfox\)").captures(ua) {
        let family = "Swiftfox".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(rekonq)\/(\d+)\.(\d+)(?:\.(\d+)|) Safari").captures(ua) {
        let family = "Rekonq".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"rekonq").is_match(ua) {
        let family = "Rekonq".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(conkeror|Conkeror)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Conkeror".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(konqueror)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Konqueror".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(WeTab)-Browser").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Comodo_Dragon)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Comodo Dragon".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Symphony) (\d+).(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"PLAYSTATION 3.+WebKit").is_match(ua) {
        let family = "NetFront NX".to_owned();
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"PLAYSTATION 3").is_match(ua) {
        let family = "NetFront".to_owned();
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"(PlayStation Portable)").is_match(ua) {
        let family = "NetFront".to_owned();
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"(PlayStation Vita)").is_match(ua) {
        let family = "NetFront NX".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"AppleWebKit.+ (NX)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "NetFront NX".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"(Nintendo 3DS)").is_match(ua) {
        let family = "NetFront NX".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Silk)\/(\d+)\.(\d+)(?:\.([0-9\-]+)|)").captures(ua) {
        let family = "Amazon Silk".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Puffin)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Windows Phone .*(Edge)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Edge Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(SamsungBrowser)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Samsung Internet".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(SznProhlizec)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Seznam prohlížeč".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(coc_coc_browser)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Coc Coc".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(baidubrowser)[/\s](\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = "Baidu Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(FlyFlow)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Baidu Explorer".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MxBrowser)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Maxthon".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Crosswalk)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Line)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "LINE".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MiuiBrowser)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "MiuiBrowser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Mint Browser)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Mint Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Mozilla.+Android.+(GSA)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Google".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Version\/.+(Chrome)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Chrome Mobile WebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"; wv\).+(Chrome)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Chrome Mobile WebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(CrMo)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Chrome Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(CriOS)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Chrome Mobile iOS".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Chrome)\/(\d+)\.(\d+)\.(\d+)\.(\d+) Mobile(?:[ /]|$)").captures(ua) {
        let family = "Chrome Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r" Mobile .*(Chrome)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Chrome Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(chromeframe)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Chrome Frame".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(SLP Browser)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Tizen Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(SE 2\.X) MetaSr (\d+)\.(\d+)").captures(ua) {
        let family = "Sogou Explorer".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MQQBrowser\/Mini)(?:(\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = "QQ Browser Mini".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MQQBrowser)(?:\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = "QQ Browser Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(QQBrowser)(?:\/(\d+)(?:\.(\d+)\.(\d+)(?:\.(\d+)|)|)|)").captures(ua) {
        let family = "QQ Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Rackspace Monitoring)\/(\d+)\.(\d+)").captures(ua) {
        let family = "RackspaceBot".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(PyAMF)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(YaBrowser)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Yandex Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Chrome)\/(\d+)\.(\d+)\.(\d+).* MRCHROME").captures(ua) {
        let family = "Mail.ru Chromium Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(AOL) (\d+)\.(\d+); AOLBuild (\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(PodCruncher|Downcast)[ /]?(\d+)(?:\.(\d+)|)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r" (BoxNotes)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Whale)\/(\d+)\.(\d+)\.(\d+)\.(\d+) Mobile(?:[ /]|$)").captures(ua) {
        let family = "Whale".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Whale)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Whale".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Ghost)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Slack_SSB)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Slack Desktop Client".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(HipChat)\/?(\d+|)").captures(ua) {
        let family = "HipChat Desktop Client".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\b(MobileIron|FireWeb|Jasmine|ANTGalio|Midori|Fresco|Lobo|PaleMoon|Maxthon|Lynx|OmniWeb|Dillo|Camino|Demeter|Fluid|Fennec|Epiphany|Shiira|Sunrise|Spotify|Flock|Netscape|Lunascape|WebPilot|NetFront|Netfront|Konqueror|SeaMonkey|Kazehakase|Vienna|Iceape|Iceweasel|IceWeasel|Iron|K-Meleon|Sleipnir|Galeon|GranParadiso|Opera Mini|iCab|NetNewsWire|ThunderBrowse|Iris|UP\.Browser|Bunjalloo|Google Earth|Raven for Mac|Openwave|MacOutlook|Electron|OktaMobile)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Microsoft Office Outlook 12\.\d+\.\d+|MSOffice 12").is_match(ua) {
        let family = "Outlook".to_owned();
        let major = "2007".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Microsoft Outlook 14\.\d+\.\d+|MSOffice 14").is_match(ua) {
        let family = "Outlook".to_owned();
        let major = "2010".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Microsoft Outlook 15\.\d+\.\d+").is_match(ua) {
        let family = "Outlook".to_owned();
        let major = "2013".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Microsoft Outlook (?:Mail )?16\.\d+\.\d+|MSOffice 16").is_match(ua) {
        let family = "Outlook".to_owned();
        let major = "2016".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"Microsoft Office (Word) 2014").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"Outlook-Express\/7\.0.*").is_match(ua) {
        let family = "Windows Live Mail".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Airmail) (\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Thunderbird)\/(\d+)\.(\d+)(?:\.(\d+(?:pre|))|)").captures(ua) {
        let family = "Thunderbird".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Postbox)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Postbox".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Barca(?:Pro)?)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Barca".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Lotus-Notes)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Lotus Notes".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Vivaldi)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Edge?)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = "Edge".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(brave)\/(\d+)\.(\d+)\.(\d+) Chrome").captures(ua) {
        let family = "Brave".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Chrome)\/(\d+)\.(\d+)\.(\d+)[\d.]* Iron[^/]").captures(ua) {
        let family = "Iron".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\b(Dolphin)(?: |HDCN\/|\/INT\-)(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(HeadlessChrome)(?:\/(\d+)\.(\d+)\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Evolution)\/(\d+)\.(\d+)\.(\d+\.\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(RCM CardDAV plugin)\/(\d+)\.(\d+)\.(\d+(?:-dev|))").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(bingbot|Bolt|AdobeAIR|Jasmine|IceCat|Skyfire|Midori|Maxthon|Lynx|Arora|IBrowse|Dillo|Camino|Shiira|Fennec|Phoenix|Flock|Netscape|Lunascape|Epiphany|WebPilot|Opera Mini|Opera|NetFront|Netfront|Konqueror|Googlebot|SeaMonkey|Kazehakase|Vienna|Iceape|Iceweasel|IceWeasel|Iron|K-Meleon|Sleipnir|Galeon|GranParadiso|iCab|iTunes|MacAppStore|NetNewsWire|Space Bison|Stainless|Orca|Dolfin|BOLT|Minimo|Tizen Browser|Polaris|Abrowser|Planetweb|ICE Browser|mDolphin|qutebrowser|Otter|QupZilla|MailBar|kmail2|YahooMobileMail|ExchangeWebServices|ExchangeServicesClient|Dragon|Outlook-iOS-Android)\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Chromium|Chrome)\/(\d+)\.(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(IEMobile)[ /](\d+)\.(\d+)").captures(ua) {
        let family = "IE Mobile".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(BacaBerita App)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(bPod|Pocket Casts|Player FM)$").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(AlexaMediaPlayer|VLC)\/(\d+)\.(\d+)\.([^.\s]+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(AntennaPod|WMPlayer|Zune|Podkicker|Radio|ExoPlayerDemo|Overcast|PocketTunes|NSPlayer|okhttp|DoggCatcher|QuickNews|QuickTime|Peapod|Podcasts|GoldenPod|VLC|Spotify|Miro|MediaGo|Juice|iPodder|gPodder|Banshee)\/(\d+)\.(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Peapod|Liferea)\/([^.\s]+)\.([^.\s]+|)\.?([^.\s]+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(bPod|Player FM) BMID\/(\S+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Podcast ?Addict)\/v(\d+) ").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"^(Podcast ?Addict) ").is_match(ua) {
        let family = "PodcastAddict".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Replay) AV").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(VOX) Music Player").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(CITA) RSS Aggregator\/(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Pocket Casts)$").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Player FM)$").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(LG Player|Doppler|FancyMusic|MediaMonkey|Clementine) (\d+)\.(\d+)\.?([^.\s]+|)\.?([^.\s]+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(philpodder)\/(\d+)\.(\d+)\.?([^.\s]+|)\.?([^.\s]+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Player FM|Pocket Casts|DoggCatcher|Spotify|MediaMonkey|MediaGo|BashPodder)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(QuickTime)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Kinoma)(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Fancy) Cloud Music (\d+)\.(\d+)").captures(ua) {
        let family = "FancyMusic".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"EspnDownloadManager").is_match(ua) {
        let family = "ESPN".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(ESPN) Radio (\d+)\.(\d+)(?:\.(\d+)|) ?(?:rv:(\d+)|) ").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(podracer|jPodder) v ?(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(ZDM)\/(\d+)\.(\d+)[; ]?").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Zune|BeyondPod) (\d+)(?:\.(\d+)|)[\);]").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(WMPlayer)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"^(Lavf)").is_match(ua) {
        let family = "WMPlayer".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(RSSRadio)[ /]?(\d+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(RSS_Radio) (\d+)\.(\d+)").captures(ua) {
        let family = "RSSRadio".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Podkicker) \S+\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Podkicker".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(HTC) Streaming Player \S+ \/ \S+ \/ \S+ \/ (\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Stitcher)\/iOS").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Stitcher)\/Android").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(VLC) .*version (\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r" (VLC) for").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(vlc)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "VLC".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(foobar)\S+\/([^.\s]+)\.([^.\s]+|)\.?([^.\s]+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Clementine)\S+ ([^.\s]+)\.([^.\s]+|)\.?([^.\s]+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(amarok)\/([^.\s]+)\.([^.\s]+|)\.?([^.\s]+|)").captures(ua) {
        let family = "Amarok".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Custom)-Feed Reader").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iRider|Crazy Browser|SkipStone|iCab|Lunascape|Sleipnir|Maemo Browser) (\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iCab|Lunascape|Opera|Android|Jasmine|Polaris|Microsoft SkyDriveSync|The Bat!) (\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Kindle)\/(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Android) Donut").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "1".to_owned();
        let minor="2".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Android) Eclair").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "2".to_owned();
        let minor="1".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Android) Froyo").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "2".to_owned();
        let minor="2".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Android) Gingerbread").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "2".to_owned();
        let minor="3".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Android) Honeycomb").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "3".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MSIE) (\d+)\.(\d+).*XBLWP7").captures(ua) {
        let family = "IE Large Screen".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Nextcloud)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(mirall)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(ownCloud-android)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Owncloud".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(OC)\/(\d+)\.(\d+)\.(\d+)\.(\d+) \(Skype for Business\)").captures(ua) {
        let family = "Skype".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Obigo)InternetBrowser").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Obigo)\-Browser").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Obigo|OBIGO)[^\d]*(\d+)(?:.(\d+)|)").captures(ua) {
        let family = "Obigo".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(MAXTHON|Maxthon) (\d+)\.(\d+)").captures(ua) {
        let family = "Maxthon".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Maxthon|MyIE2|Uzbl|Shiira)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "0".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(BrowseX) \((\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(NCSA_Mosaic)\/(\d+)\.(\d+)").captures(ua) {
        let family = "NCSA Mosaic".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(POLARIS)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Polaris".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Embider)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Polaris".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(BonEcho)\/(\d+)\.(\d+)\.?([ab]?\d+|)").captures(ua) {
        let family = "Bon Echo".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+GSA\/(\d+)\.(\d+)\.(\d+) Mobile").captures(ua) {
        let family = "Google".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+Version\/(\d+)\.(\d+)(?:\.(\d+)|).*[ +]Safari").captures(ua) {
        let family = "Mobile Safari".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPod touch|iPhone|iPad);.*CPU.*OS[ +](\d+)_(\d+)(?:_(\d+)|).* AppleNews\/\d+\.\d+\.\d+?").captures(ua) {
        let family = "Mobile Safari UI/WKWebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).+Version\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = "Mobile Safari UI/WKWebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPod touch|iPhone|iPad);.*CPU.*OS[ +](\d+)_(\d+)(?:_(\d+)|).*Mobile.*[ +]Safari").captures(ua) {
        let family = "Mobile Safari".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(iPod|iPod touch|iPhone|iPad);.*CPU.*OS[ +](\d+)_(\d+)(?:_(\d+)|).*Mobile").captures(ua) {
        let family = "Mobile Safari UI/WKWebView".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad).* Safari").is_match(ua) {
        let family = "Mobile Safari".to_owned();
        return [ family, major, minor, patch ];
    } else if crate::regex_cache::cached_regex(r"(iPod|iPhone|iPad)").is_match(ua) {
        let family = "Mobile Safari UI/WKWebView".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Watch)(\d+),(\d+)").captures(ua) {
        let family = "Apple $1 App".replace("$1", result.get(1).unwrap().into());
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Outlook-iOS)\/\d+\.\d+\.prod\.iphone \((\d+)\.(\d+)\.(\d+)\)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(AvantGo) (\d+).(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(OneBrowser)\/(\d+).(\d+)").captures(ua) {
        let family = "ONE Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Avant)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "1".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(QtCarBrowser)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "1".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(iBrowser\/Mini)(\d+).(\d+)").captures(ua) {
        let family = "iBrowser Mini".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(iBrowser|iRAPP)\/(\d+).(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"^(Nokia)").is_match(ua) {
        let family = "Nokia Services (WAP) Browser".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(NokiaBrowser)\/(\d+)\.(\d+).(\d+)\.(\d+)").captures(ua) {
        let family = "Nokia Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(NokiaBrowser)\/(\d+)\.(\d+).(\d+)").captures(ua) {
        let family = "Nokia Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(NokiaBrowser)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Nokia Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(BrowserNG)\/(\d+)\.(\d+).(\d+)").captures(ua) {
        let family = "Nokia Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"(Series60)\/5\.0").is_match(ua) {
        let family = "Nokia Browser".to_owned();
        let major = "7".to_owned();
        let minor="0".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Series60)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Nokia OSS Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(S40OviBrowser)\/(\d+)\.(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Ovi Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Nokia)[EN]?(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(PlayBook).+RIM Tablet OS (\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "BlackBerry WebKit".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Black[bB]erry|BB10).+Version\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "BlackBerry WebKit".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Black[bB]erry)\s?(\d+)").captures(ua) {
        let family = "BlackBerry".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(OmniWeb)\/v(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Blazer)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Palm Blazer".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Pre)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Palm Pre".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(ELinks)\/(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(ELinks) \((\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Links) \((\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(QtWeb) Internet Browser\/(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(PhantomJS)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(AppleWebKit)\/(\d+)(?:\.(\d+)|)\+ .* Safari").captures(ua) {
        let family = "WebKit Nightly".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Version)\/(\d+)\.(\d+)(?:\.(\d+)|).*Safari\/").captures(ua) {
        let family = "Safari".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Safari)\/\d+").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(OLPC)\/Update(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(OLPC)\/Update()\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = "0".to_owned();
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(SEMC\-Browser)\/(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if  crate::regex_cache::cached_regex(r"(Teleca)").is_match(ua) {
        let family = "Teleca Browser".to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Phantom)\/V(\d+)\.(\d+)").captures(ua) {
        let family = "Phantom Browser".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Trident)\/(7|8)\.(0)").captures(ua) {
        let family = "IE".to_owned();
        let major = "11".to_owned();
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Trident)\/(6)\.(0)").captures(ua) {
        let family = "IE".to_owned();
        let major = "10".to_owned();
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Trident)\/(5)\.(0)").captures(ua) {
        let family = "IE".to_owned();
        let major = "9".to_owned();
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Trident)\/(4)\.(0)").captures(ua) {
        let family = "IE".to_owned();
        let major = "8".to_owned();
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Espial)\/(\d+)(?:\.(\d+)|)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(AppleWebKit)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Apple Mail".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Firefox)\/(\d+)\.(\d+)(pre|[ab]\d+[a-z]*|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"([MS]?IE) (\d+)\.(\d+)").captures(ua) {
        let family = "IE".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(python-requests)\/(\d+)\.(\d+)").captures(ua) {
        let family = "Python Requests".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\b(Windows-Update-Agent|Microsoft-CryptoAPI|SophosUpdateManager|SophosAgent|Debian APT-HTTP|Ubuntu APT-HTTP|libcurl-agent|libwww-perl|urlgrabber|curl|PycURL|Wget|aria2|Axel|OpenBSD ftp|lftp|jupdate|insomnia|fetch libfetch|akka-http|got)(?:[ /](\d+)(?:\.(\d+)|)(?:\.(\d+)|)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Python\/3\.\d{1,3} aiohttp)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Python\/3\.\d{1,3} aiohttp)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Java)[/ ]{0,1}\d+\.(\d+)\.(\d+)[_-]*([a-zA-Z0-9]+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Cyberduck)\/(\d+)\.(\d+)\.(\d+)(?:\.\d+|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(S3 Browser) (\d+)-(\d+)-(\d+)(?:\s*http:\/\/s3browser\.com|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(S3Gof3r)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"\b(ibm-cos-sdk-(?:core|java|js|python))\/(\d+)\.(\d+)(?:\.(\d+)|)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(rusoto)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(rclone)\/v(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Roku)\/DVP-(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"(Kurio)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "Kurio App".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(Box(?: Sync)?)\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = Into::<&str>::into(result.get(1).unwrap()).to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    } else if let Some(result) = crate::regex_cache::cached_regex(r"^(ViaFree|Viafree)-(?:tvOS-)?[A-Z]{2}\/(\d+)\.(\d+)\.(\d+)").captures(ua) {
        let family = "ViaFree".to_owned();
        let major = match result.get(2) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        let minor = match result.get(3) {
            Some(r) => {
                    Into::<&str>::into(r).to_string()
            },
            None => {
                    "0".to_string()
            }
        };
        return [ family, major, minor, patch ];
    }
    [family, major, minor, patch]
}
