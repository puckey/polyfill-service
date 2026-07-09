
pub fn parse(ua: &str) -> [String; 4] {
    if let Some(result) = crate::regex_cache::cached_regex("(Rival IQ, rivaliq.com)").captures(ua) {
        let family = "Rival IQ";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(ESPN)[%20| ]+Radio/(\\d+)\\.(\\d+)\\.(\\d+) CFNetwork")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Antenna)/(\\d+) CFNetwork")
        .captures(ua)
    {
        let family = "AntennaPod";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(TopPodcasts)Pro/(\\d+) CFNetwork")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MusicDownloader)Lite/(\\d+)\\.(\\d+)\\.(\\d+) CFNetwork")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(.*)-iPad/(\\d+)\\.?(\\d+)?.?(\\d+)?.?(\\d+)? CFNetwork")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(.*)-iPhone/(\\d+)\\.?(\\d+)?.?(\\d+)?.?(\\d+)? CFNetwork")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(.*)/(\\d+)\\.?(\\d+)?.?(\\d+)?.?(\\d+)? CFNetwork")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(espn\\.go)").captures(ua) {
        let family = "ESPN";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(espnradio\\.com)").captures(ua) {
        let family = "ESPN";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("ESPN APP$").captures(ua) {
        let family = "ESPN";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(audioboom\\.com)").captures(ua) {
        let family = "AudioBoom";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Rivo) RHYTHM").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)(?:/(\\d+)\\.(\\d+)\\.?(\\d+)?)?")
        .captures(ua)
    {
        let family = "CFNetwork";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Pingdom.com_bot_version_)(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "PingdomBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PingdomTMS)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "PingdomBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(NewRelicPinger)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "NewRelicPingerBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(\\(StatusCake\\))").captures(ua) {
        let family = "StatusCakeBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(facebookexternalhit)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "FacebookBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Google.*/\\+/web/snippet")
        .captures(ua)
    {
        let family = "GooglePlusBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("via ggpht.com GoogleImageProxy")
        .captures(ua)
    {
        let family = "GmailImageProxy";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Twitterbot)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "TwitterBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("/((?:Ant-)?Nutch|[A-z]+[Bb]ot|[A-z]+[Ss]pider|Axtaris|fetchurl|Isara|ShopSalad|Tailsweep)[ \\-](\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("\\b(008|Altresium|Argus|BaiduMobaider|BoardReader|DNSGroup|DataparkSearch|EDI|Goodzer|Grub|INGRID|Infohelfer|LinkedInBot|LOOQ|Nutch|PathDefender|Peew|PostPost|Steeler|Twitterbot|VSE|WebCrunch|WebZIP|Y!J-BR[A-Z]|YahooSeeker|envolk|sproose|wminer)/(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(MSIE) (\\d+)\\.(\\d+)([a-z]\\d?)?;.* MSIECrawle")
        .captures(ua)
    {
        let family = "MSIECrawle";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(DAVdroid)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Google-HTTP-Java-Client|Apache-HttpClient|http%20client|Python-urllib|HttpMonitor|TLSProber|WinHTTP|JNLP|okhttp)(?:[ /](\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Pinterest(?:bot)?)/(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?[;\\s\\(]+\\+https://www.pinterest.com/bot.html").captures(ua) {
    let family = "Pinterestbot";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(1470\\.net crawler|50\\.nu|8bo Crawler Bot|Aboundex|Accoona-[A-z]+-Agent|AdsBot-Google(?:-[a-z]+)?|altavista|AppEngine-Google|archive.*?\\.org_bot|archiver|Ask Jeeves|[Bb]ai[Dd]u[Ss]pider(?:-[A-Za-z]+)*|bingbot|BingPreview|blitzbot|BlogBridge|Bloglovin|BoardReader(?: [A-Za-z]+)*|boitho.com-dc|BotSeer|\\b\\w*favicon\\w*\\b|\\bYeti(?:-[a-z]+)?|Catchpoint(?: bot)?|[Cc]harlotte|Checklinks|clumboot|Comodo HTTP\\(S\\) Crawler|Comodo-Webinspector-Crawler|ConveraCrawler|CRAWL-E|CrawlConvera|Daumoa(?:-feedfetcher)?|Feed Seeker Bot|Feedbin|findlinks|Flamingo_SearchEngine|FollowSite Bot|furlbot|Genieo|gigabot|GomezAgent|gonzo1|(?:[a-zA-Z]+-)?Googlebot(?:-[a-zA-Z]+)?|Google SketchUp|grub-client|gsa-crawler|heritrix|HiddenMarket|holmes|HooWWWer|htdig|ia_archiver|ICC-Crawler|Icarus6j|ichiro(?:/mobile)?|IconSurf|IlTrovatore(?:-Setaccio)?|InfuzApp|Innovazion Crawler|InternetArchive|IP2[a-z]+Bot|jbot\\b|KaloogaBot|Kraken|Kurzor|larbin|LEIA|LesnikBot|Linguee Bot|LinkAider|LinkedInBot|Lite Bot|Llaut|lycos|Mail\\.RU_Bot|masscan|masidani_bot|Mediapartners-Google|Microsoft .*? Bot|mogimogi|mozDex|MJ12bot|msnbot(?:-media *)?|msrbot|Mtps Feed Aggregation System|netresearch|Netvibes|NewsGator[^/]*|^NING|Nutch[^/]*|Nymesis|ObjectsSearch|Orbiter|OOZBOT|PagePeeker|PagesInventory|PaxleFramework|Peeplo Screenshot Bot|PlantyNet_WebRobot|Pompos|Qwantify|Read%20Later|Reaper|RedCarpet|Retreiver|Riddler|Rival IQ|scooter|Scrapy|Scrubby|searchsight|seekbot|semanticdiscovery|Simpy|SimplePie|SEOstats|SimpleRSS|SiteCon|Slackbot-LinkExpanding|Slack-ImgProxy|Slurp|snappy|Speedy Spider|Squrl Java|Stringer|TheUsefulbot|ThumbShotsBot|Thumbshots\\.ru|Tiny Tiny RSS|TwitterBot|WhatsApp|URL2PNG|Vagabondo|VoilaBot|^vortex|Votay bot|^voyager|WASALive.Bot|Web-sniffer|WebThumb|WeSEE:[A-z]+|WhatWeb|WIRE|WordPress|Wotbox|www\\.almaden\\.ibm\\.com|Xenu(?:.s)? Link Sleuth|Xerka [A-z]+Bot|yacy(?:bot)?|Yahoo[a-z]*Seeker|Yahoo! Slurp|Yandex\\w+|YodaoBot(?:-[A-z]+)?|YottaaMonitor|Yowedo|^Zao|^Zao-Crawler|ZeBot_www\\.ze\\.bz|ZooShot|ZyBorg)(?:[ /]v?(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(?:\\/[A-Za-z0-9\\.]+)? *([A-Za-z0-9 \\-_\\!\\[\\]:]*(?:[Aa]rchiver|[Ii]ndexer|[Ss]craper|[Bb]ot|[Ss]pider|[Cc]rawl[a-z]*))/(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(?:\\/[A-Za-z0-9\\.]+)? *([A-Za-z0-9 _\\!\\[\\]:]*(?:[Aa]rchiver|[Ii]ndexer|[Ss]craper|[Bb]ot|[Ss]pider|[Cc]rawl[a-z]*)) (\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("((?:[A-z0-9]+|[A-z\\-]+ ?)?(?: the )?(?:[Ss][Pp][Ii][Dd][Ee][Rr]|[Ss]crape|[A-Za-z0-9-]*(?:[^C][^Uu])[Bb]ot|[Cc][Rr][Aa][Ww][Ll])[A-z0-9]*)(?:(?:[ /]| v)(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)/(\\d+)\\.(\\d+)\\.(\\d+) \\(")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Chimera|SeaMonkey|Camino)/(\\d+)\\.(\\d+)\\.?([ab]?\\d+[a-z]*)?")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\[FB.*;(FBAV)/(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?")
        .captures(ua)
    {
        let family = "Facebook";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\[(Pinterest)/[^\\]]+\\]")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Pinterest)(?: for Android(?: Tablet)?)?/(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PaleMoon)/(\\d+)\\.(\\d+)\\.?(\\d+)?")
        .captures(ua)
    {
        let family = "Pale Moon";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Fennec)/(\\d+)\\.(\\d+)\\.?([ab]?\\d+[a-z]*)")
        .captures(ua)
    {
        let family = "Firefox Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Fennec)/(\\d+)\\.(\\d+)(pre)")
        .captures(ua)
    {
        let family = "Firefox Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Fennec)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Firefox Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:Mobile|Tablet);.*(Firefox)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Firefox Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Namoroka|Shiretoko|Minefield)/(\\d+)\\.(\\d+)\\.(\\d+(?:pre)?)")
            .captures(ua)
    {
        let family = "Firefox ($1)";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)/(\\d+)\\.(\\d+)(a\\d+[a-z]*)")
        .captures(ua)
    {
        let family = "Firefox Alpha";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)/(\\d+)\\.(\\d+)(b\\d+[a-z]*)")
        .captures(ua)
    {
        let family = "Firefox Beta";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)-(?:\\d+\\.\\d+)?/(\\d+)\\.(\\d+)(a\\d+[a-z]*)")
        .captures(ua)
    {
        let family = "Firefox Alpha";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)-(?:\\d+\\.\\d+)?/(\\d+)\\.(\\d+)(b\\d+[a-z]*)")
        .captures(ua)
    {
        let family = "Firefox Beta";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Namoroka|Shiretoko|Minefield)/(\\d+)\\.(\\d+)([ab]\\d+[a-z]*)?")
            .captures(ua)
    {
        let family = "Firefox ($1)";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox).*Tablet browser (\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "MicroB";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MozillaDeveloperPreview)/(\\d+)\\.(\\d+)([ab]\\d+[a-z]*)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(FxiOS)/(\\d+)\\.(\\d+)(\\.(\\d+))?(\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Firefox iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Flock)/(\\d+)\\.(\\d+)(b\\d+?)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(RockMelt)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Navigator)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Netscape";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Navigator)/(\\d+)\\.(\\d+)([ab]\\d+)")
        .captures(ua)
    {
        let family = "Netscape";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Netscape6)/(\\d+)\\.(\\d+)\\.?([ab]?\\d+)?")
        .captures(ua)
    {
        let family = "Netscape";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MyIBrow)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "My Internet Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(UC? ?Browser|UCWEB|U3)[ /]?(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "UC Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Opera Tablet).*Version/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Opera Mini)(?:/att)?/?(\\d+)?(?:\\.(\\d+))?(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Opera)/.+Opera Mobi.+Version/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Opera Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Opera)/(\\d+)\\.(\\d+).+Opera Mobi")
        .captures(ua)
    {
        let family = "Opera Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Opera Mobi.+(Opera)(?:/|\\s+)(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Opera Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Opera Mobi").captures(ua) {
        let family = "Opera Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Opera)/9.80.*Version/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:Mobile Safari).*(OPR)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Opera Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:Chrome).*(OPR)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Opera";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Coast)/(\\d+).(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "Opera Coast";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(OPiOS)/(\\d+).(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "Opera Mini";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Chrome/.+( MMS)/(\\d+).(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "Opera Neon";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(hpw|web)OS/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "webOS Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(luakit)").captures(ua) {
        let family = "LuaKit";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Snowshoe)/(\\d+)\\.(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("Gecko/\\d+ (Lightning)/(\\d+)\\.(\\d+)\\.?((?:[ab]?\\d+[a-z]*)|(?:\\d*))")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)/(\\d+)\\.(\\d+)\\.(\\d+(?:pre)?) \\(Swiftfox\\)")
        .captures(ua)
    {
        let family = "Swiftfox";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)/(\\d+)\\.(\\d+)([ab]\\d+[a-z]*)? \\(Swiftfox\\)")
        .captures(ua)
    {
        let family = "Swiftfox";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(rekonq)/(\\d+)\\.(\\d+)\\.?(\\d+)? Safari")
        .captures(ua)
    {
        let family = "Rekonq";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("rekonq").captures(ua) {
        let family = "Rekonq";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(conkeror|Conkeror)/(\\d+)\\.(\\d+)\\.?(\\d+)?")
        .captures(ua)
    {
        let family = "Conkero";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(konqueror)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Konquero";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(WeTab)-Browse").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Comodo_Dragon)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Comodo Dragon";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Symphony) (\\d+).(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("PLAYSTATION 3.+WebKit").captures(ua) {
        let family = "NetFront NX";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("PLAYSTATION 3").captures(ua) {
        let family = "NetFront";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PlayStation Portable)").captures(ua) {
        let family = "NetFront";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PlayStation Vita)").captures(ua) {
        let family = "NetFront NX";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("AppleWebKit.+ (NX)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "NetFront NX";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Nintendo 3DS)").captures(ua) {
        let family = "NetFront NX";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Silk)/(\\d+)\\.(\\d+)(?:\\.([0-9\\-]+))?")
        .captures(ua)
    {
        let family = "Amazon Silk";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Puffin)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone .*(Edge)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Edge Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(SamsungBrowser)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Samsung Internet";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(SznProhlizec)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Seznam.cz";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(coc_coc_browser)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Coc Coc";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(baidubrowser)[/\\s](\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?")
        .captures(ua)
    {
        let family = "Baidu Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(FlyFlow)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Baidu Explore";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MxBrowser)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Maxthon";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Crosswalk)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; wv\\).+(Chrome)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Chrome Mobile WebView";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CrMo)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Chrome Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CriOS)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Chrome Mobile iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Chrome)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+) Mobile(?:[ /]|$)")
        .captures(ua)
    {
        let family = "Chrome Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" Mobile .*(Chrome)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Chrome Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(chromeframe)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Chrome Frame";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(SLP Browser)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Tizen Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(SE 2\\.X) MetaSr (\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Sogou Explore";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MQQBrowser/Mini)(?:(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?)?")
        .captures(ua)
    {
        let family = "QQ Browser Mini";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MQQBrowser)(?:/(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?)?")
        .captures(ua)
    {
        let family = "QQ Browser Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(QQBrowser)(?:/(\\d+)(?:\\.(\\d+)\\.(\\d+)(?:\\.(\\d+))?)?)?")
        .captures(ua)
    {
        let family = "QQ Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Rackspace Monitoring)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "RackspaceBot";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PyAMF)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(YaBrowser)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Yandex Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Chrome)/(\\d+)\\.(\\d+)\\.(\\d+).* MRCHROME")
        .captures(ua)
    {
        let family = "Mail.ru Chromium Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(AOL) (\\d+)\\.(\\d+); AOLBuild (\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(PodCruncher|Downcast)[ /]?(\\d+)\\.?(\\d+)?\\.?(\\d+)?\\.?(\\d+)?")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (BoxNotes)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Slack_SSB)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Slack Desktop Client";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HipChat)/?(\\d+)?").captures(ua) {
        let family = "HipChat Desktop Client";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(MobileIron|FireWeb|Jasmine|ANTGalio|Midori|Fresco|Lobo|PaleMoon|Maxthon|Lynx|OmniWeb|Dillo|Camino|Demeter|Fluid|Fennec|Epiphany|Shiira|Sunrise|Spotify|Flock|Netscape|Lunascape|WebPilot|NetFront|Netfront|Konqueror|SeaMonkey|Kazehakase|Vienna|Iceape|Iceweasel|IceWeasel|Iron|K-Meleon|Sleipnir|Galeon|GranParadiso|Opera Mini|iCab|NetNewsWire|ThunderBrowse|Iris|UP\\.Browser|Bunjalloo|Google Earth|Raven for Mac|Openwave|MacOutlook|Electron)/(\\d+)\\.(\\d+)\\.(\\d+)").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Microsoft Office Outlook 12\\.\\d+\\.\\d+|MSOffice 12")
        .captures(ua)
    {
        let family = "Outlook";
        let major = "2007";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Microsoft Outlook 14\\.\\d+\\.\\d+|MSOffice 14")
        .captures(ua)
    {
        let family = "Outlook";
        let major = "2010";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Microsoft Outlook 15\\.\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Outlook";
        let major = "2013";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Microsoft Outlook (?:Mail )?16\\.\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Outlook";
        let major = "2016";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Outlook-Express\\/7\\.0.*")
        .captures(ua)
    {
        let family = "Windows Live Mail";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Airmail) (\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Thunderbird)/(\\d+)\\.(\\d+)(?:\\.(\\d+(?:pre)?))?")
        .captures(ua)
    {
        let family = "Thunderbird";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Postbox)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Postbox";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Barca(?:Pro)?)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Barca";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Lotus-Notes)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Lotus Notes";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Vivaldi)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Edge)/(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(brave)/(\\d+)\\.(\\d+)\\.(\\d+) Chrome")
        .captures(ua)
    {
        let family = "Brave";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Chrome)/(\\d+)\\.(\\d+)\\.(\\d+)[\\d.]* Iron[^/]")
        .captures(ua)
    {
        let family = "Iron";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(Dolphin)(?: |HDCN/|/INT\\-)(\\d+)\\.(\\d+)\\.?(\\d+)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HeadlessChrome)(?:/(\\d+)\\.(\\d+)\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Evolution)/(\\d+)\\.(\\d+)\\.(\\d+\\.\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(RCM CardDAV plugin)/(\\d+)\\.(\\d+)\\.(\\d+(?:-dev)?)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(bingbot|Bolt|AdobeAIR|Jasmine|IceCat|Skyfire|Midori|Maxthon|Lynx|Arora|IBrowse|Dillo|Camino|Shiira|Fennec|Phoenix|Flock|Netscape|Lunascape|Epiphany|WebPilot|Opera Mini|Opera|NetFront|Netfront|Konqueror|Googlebot|SeaMonkey|Kazehakase|Vienna|Iceape|Iceweasel|IceWeasel|Iron|K-Meleon|Sleipnir|Galeon|GranParadiso|iCab|iTunes|MacAppStore|NetNewsWire|Space Bison|Stainless|Orca|Dolfin|BOLT|Minimo|Tizen Browser|Polaris|Abrowser|Planetweb|ICE Browser|mDolphin|qutebrowser|Otter|QupZilla|MailBar|kmail2|YahooMobileMail|ExchangeWebServices|ExchangeServicesClient|Dragon|Outlook-iOS-Android)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Chromium|Chrome)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(IEMobile)[ /](\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "IE Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BacaBerita App)\\/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(bPod|Pocket Casts|Player FM)$")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(AlexaMediaPlayer|VLC)/(\\d+)\\.(\\d+)\\.([^.\\s]+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(AntennaPod|WMPlayer|Zune|Podkicker|Radio|ExoPlayerDemo|Overcast|PocketTunes|NSPlayer|okhttp|DoggCatcher|QuickNews|QuickTime|Peapod|Podcasts|GoldenPod|VLC|Spotify|Miro|MediaGo|Juice|iPodder|gPodder|Banshee)/(\\d+)\\.(\\d+)\\.?(\\d+)?\\.?(\\d+)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("^(Peapod|Liferea)/([^.\\s]+)\\.([^.\\s]+)?\\.?([^.\\s]+)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(bPod|Player FM) BMID/(\\S+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Podcast ?Addict)/v(\\d+) ")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Podcast ?Addict) ").captures(ua) {
        let family = "PodcastAddict";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Replay) AV").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(VOX) Music Playe").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CITA) RSS Aggregator/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Pocket Casts)$").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Player FM)$").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(LG Player|Doppler|FancyMusic|MediaMonkey|Clementine) (\\d+)\\.(\\d+)\\.?([^.\\s]+)?\\.?([^.\\s]+)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(philpodder)/(\\d+)\\.(\\d+)\\.?([^.\\s]+)?\\.?([^.\\s]+)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Player FM|Pocket Casts|DoggCatcher|Spotify|MediaMonkey|MediaGo|BashPodder)")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(QuickTime)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Kinoma)(\\d+)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Fancy) Cloud Music (\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "FancyMusic";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("EspnDownloadManage").captures(ua) {
        let family = "ESPN";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(ESPN) Radio (\\d+)\\.(\\d+)\\.?(\\d+)? ?[rv:]?(\\d+)? ")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(podracer|jPodder) v ?(\\d+)\\.(\\d+)\\.?(\\d+)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(ZDM)/(\\d+)\\.(\\d+)[; ]?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Zune|BeyondPod) (\\d+)\\.?(\\d+)?[\\);]")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(WMPlayer)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Lavf)").captures(ua) {
        let family = "WMPlaye";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(RSSRadio)[ /]?(\\d+)?").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(RSS_Radio) (\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "RSSRadio";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Podkicker) \\S+/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Podkicke";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("^(HTC) Streaming Player \\S+ / \\S+ / \\S+ / (\\d+)\\.(\\d+)\\.?(\\d+)?")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Stitcher)/iOS").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Stitcher)/Android").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(VLC) .*version (\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (VLC) fo").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(vlc)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "VLC";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(foobar)\\S+/([^.\\s]+)\\.([^.\\s]+)?\\.?([^.\\s]+)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Clementine)\\S+ ([^.\\s]+)\\.([^.\\s]+)?\\.?([^.\\s]+)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(amarok)/([^.\\s]+)\\.([^.\\s]+)?\\.?([^.\\s]+)?")
        .captures(ua)
    {
        let family = "Amarok";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Custom)-Feed Reade").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iRider|Crazy Browser|SkipStone|iCab|Lunascape|Sleipnir|Maemo Browser) (\\d+)\\.(\\d+)\\.(\\d+)").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(iCab|Lunascape|Opera|Android|Jasmine|Polaris|Microsoft SkyDriveSync|The Bat!) (\\d+)\\.(\\d+)\\.?(\\d+)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Kindle)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Donut").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "1";
        let minor = "2";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Eclai").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2";
        let minor = "1";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Froyo").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2";
        let minor = "2";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Gingerbread").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2";
        let minor = "3";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Honeycomb").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "3";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MSIE) (\\d+)\\.(\\d+).*XBLWP7")
        .captures(ua)
    {
        let family = "IE Large Screen";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Nextcloud)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(mirall)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(ownCloud-android)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Owncloud";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Obigo)InternetBrowse").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Obigo)\\-Browse").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Obigo|OBIGO)[^\\d]*(\\d+)(?:.(\\d+))?")
        .captures(ua)
    {
        let family = "Obigo";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MAXTHON|Maxthon) (\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Maxthon";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Maxthon|MyIE2|Uzbl|Shiira)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "0";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BrowseX) \\((\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(NCSA_Mosaic)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "NCSA Mosaic";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(POLARIS)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Polaris";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Embider)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Polaris";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BonEcho)/(\\d+)\\.(\\d+)\\.?([ab]?\\d+)?")
        .captures(ua)
    {
        let family = "Bon Echo";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(iPod|iPhone|iPad).+Version/(\\d+)\\.(\\d+)(?:\\.(\\d+))?.*[ +]Safari")
            .captures(ua)
    {
        let family = "Mobile Safari";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPod|iPod touch|iPhone|iPad);.*CPU.*OS[ +](\\d+)_(\\d+)(?:_(\\d+))?.* AppleNews\\/\\d+\\.\\d+\\.\\d+?").captures(ua) {
    let family = "Mobile Safari UI/WKWebView";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(iPod|iPhone|iPad).+Version/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Mobile Safari UI/WKWebView";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPod|iPod touch|iPhone|iPad);.*CPU.*OS[ +](\\d+)_(\\d+)(?:_(\\d+))?.*Mobile.*[ +]Safari")
    .captures(ua)
    {
        let family = "Mobile Safari";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(iPod|iPod touch|iPhone|iPad);.*CPU.*OS[ +](\\d+)_(\\d+)(?:_(\\d+))?.*Mobile")
            .captures(ua)
    {
        let family = "Mobile Safari UI/WKWebView";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPod|iPhone|iPad).* Safari")
        .captures(ua)
    {
        let family = "Mobile Safari";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPod|iPhone|iPad)").captures(ua) {
        let family = "Mobile Safari UI/WKWebView";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(AvantGo) (\\d+).(\\d+)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(OneBrowser)/(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "ONE Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Avant)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(QtCarBrowser)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(iBrowser/Mini)(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "iBrowser Mini";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(iBrowser|iRAPP)/(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Nokia)").captures(ua) {
        let family = "Nokia Services (WAP) Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(NokiaBrowser)/(\\d+)\\.(\\d+).(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Nokia Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(NokiaBrowser)/(\\d+)\\.(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "Nokia Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(NokiaBrowser)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Nokia Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BrowserNG)/(\\d+)\\.(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "Nokia Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Series60)/5\\.0").captures(ua) {
        let family = "Nokia Browse";
        let major = "7";
        let minor = "0";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Series60)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Nokia OSS Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(S40OviBrowser)/(\\d+)\\.(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Ovi Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Nokia)[EN]?(\\d+)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PlayBook).+RIM Tablet OS (\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "BlackBerry WebKit";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Black[bB]erry|BB10).+Version/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "BlackBerry WebKit";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Black[bB]erry)\\s?(\\d+)")
        .captures(ua)
    {
        let family = "BlackBerry";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(OmniWeb)/v(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Blazer)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Palm Blaze";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Pre)/(\\d+)\\.(\\d+)").captures(ua) {
        let family = "Palm Pre";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(ELinks)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(ELinks) \\((\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Links) \\((\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(QtWeb) Internet Browser/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PhantomJS)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(AppleWebKit)/(\\d+)\\.?(\\d+)?\\+ .* Safari")
        .captures(ua)
    {
        let family = "WebKit Nightly";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Version)/(\\d+)\\.(\\d+)(?:\\.(\\d+))?.*Safari/")
        .captures(ua)
    {
        let family = "Safari";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Safari)/\\d+").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(OLPC)/Update(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(OLPC)/Update()\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "0";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(SEMC\\-Browser)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Teleca)").captures(ua) {
        let family = "Teleca Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Phantom)/V(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Phantom Browse";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Trident)/(7)\\.(0)").captures(ua) {
        let family = "IE";
        let major = "11";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Trident)/(6)\\.(0)").captures(ua) {
        let family = "IE";
        let major = "10";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Trident)/(5)\\.(0)").captures(ua) {
        let family = "IE";
        let major = "9";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Trident)/(4)\\.(0)").captures(ua) {
        let family = "IE";
        let major = "8";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Espial)/(\\d+)(?:\\.(\\d+))?(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(AppleWebKit)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Apple Mail";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Firefox)/(\\d+)\\.(\\d+)(pre|[ab]\\d+[a-z]*)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("([MS]?IE) (\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "IE";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(python-requests)/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Python Requests";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(Windows-Update-Agent|Microsoft-CryptoAPI|SophosUpdateManager|SophosAgent|Debian APT-HTTP|Ubuntu APT-HTTP|libcurl-agent|libwww-perl|urlgrabber|curl|Wget|OpenBSD ftp|jupdate)(?:[ /](\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Java)[/ ]{0,1}\\d+\\.(\\d+)\\.(\\d+)[_-]*([a-zA-Z0-9]+)*")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Roku)/DVP-(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Kurio)\\/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Kurio App";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Box(?: Sync)?)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Wget)/(\\d+)\\.(\\d+)\\.?([ab]?\\d+[a-z]*)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(curl)/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "cURL";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);

        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Rival IQ, rivaliq.com)").captures(ua) {
        let family = "Spide";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:(?:iPhone|Windows CE|Windows Phone|Android).*(?:(?:Bot|Yeti)-Mobile|YRSpider|BingPreview|bots?/\\d|(?:bot|spider)\\.html)|AdsBot-Google-Mobile.*iPhone)").captures(ua) {
    let family = "Spide";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(?:DoCoMo|\\bMOT\\b|\\bLG\\b|Nokia|Samsung|SonyEricsson).*(?:(?:Bot|Yeti)-Mobile|bots?/\\d|(?:bot|crawler)\\.html|(?:jump|google|Wukong)bot|ichiro/mobile|/spider|YahooSeeker)").captures(ua) {
    let family = "Spide";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("\\bSmartWatch *\\( *([^;]+) *; *([^;]+) *;")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("Android Application[^\\-]+ - (Sony) ?(Ericsson)? (.+) \\w+ - ")
            .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android Application[^\\-]+ - (?:HTC|HUAWEI|LGE|LENOVO|MEDION|TCT) (HTC|HUAWEI|LG|LENOVO|MEDION|ALCATEL)[ _\\-](.+) \\w+ - ").captures(ua) {
    let family = "$1 $2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Android Application[^\\-]+ - ([^ ]+) (.+) \\w+ - ")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([BLRQ]C\\d{4}[A-Z]+) +Build/")
        .captures(ua)
    {
        let family = "3Q $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:3Q_)([^;/]+) +Build")
        .captures(ua)
    {
        let family = "3Q $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android [34].*; *(A100|A101|A110|A200|A210|A211|A500|A501|A510|A511|A700(?: Lite| 3G)?|A701|B1-A71|A1-\\d{3}|B1-\\d{3}|V360|V370|W500|W500P|W501|W501P|W510|W511|W700|Slider SL101|DA22[^;/]+) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *Acer Iconia Tab ([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(Z1[1235]0|E320[^/]*|S500|S510|Liquid[^;/]*|Iconia A\\d+) Build")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Acer |ACER )([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Advent )?(Vega(?:Bean|Comb)?).* Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Ainol )?((?:NOVO|[Nn]ovo)[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *AIRIS[ _\\-]?([^/;\\)]+) *(?:;|\\)|Build)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(OnePAD[^;/]+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Airpad[ \\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Airpad $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(one ?touch) (EVO7|T10|T20) Build")
        .captures(ua)
    {
        let family = "Alcatel One Touch $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:alcatel[ _])?(?:(?:one[ _]?touch[ _])|ot[ \\-])([^;/]+);? Build")
            .captures(ua)
    {
        let family = "Alcatel One Touch $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TCL)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Vodafone Smart II|Optimus_Madrid) Build")
        .captures(ua)
    {
        let family = "Alcatel $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *BASE_Lutea_3 Build").captures(ua) {
        let family = "Alcatel One Touch 998";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *BASE_Varia Build").captures(ua) {
        let family = "Alcatel One Touch 918D";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:FINE|Fine)\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ALLVIEW[ _]?|Allview[ _]?)((?:Speed|SPEED).*) Build/")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ALLVIEW[ _]?|Allview[ _]?)?(AX1_Shine|AX2_Frenzy) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ALLVIEW[ _]?|Allview[ _]?)([^;/]*) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A13-MID) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Allwinner)[ _\\-]?([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A651|A701B?|A702|A703|A705|A706|A707|A711|A712|A713|A717|A722|A785|A801|A802|A803|A901|A902|A1002|A1003|A1006|A1007|A9701|A9703|Q710|Q80) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:AMOI|Amoi)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Amoi $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(?:AMOI|Amoi)[ _]([^;/]+) Linux")
        .captures(ua)
    {
        let family = "Amoi $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MW(?:0[789]|10)[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(G7|M1013|M1015G|M11[CG]?|M-?12[B]?|M15|M19[G]?|M30[ACQ]?|M31[GQ]|M32|M33[GQ]|M36|M37|M38|M701T|M710|M712B|M713|M715G|M716G|M71(?:G|GS|T)?|M72[T]?|M73[T]?|M75[GT]?|M77G|M79T|M7L|M7LN|M81|M810|M81T|M82|M92|M92KS|M92S|M717G|M721|M722G|M723|M725G|M739|M785|M791|M92SK|M93D) Build").captures(ua) {
    let family = "Aoson $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *Aoson ([^;/]+) Build").captures(ua) {
        let family = "Aoson $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *[Aa]panda[ _\\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Apanda $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:ARCHOS|Archos) ?(GAMEPAD.*?)(?: Build|[;/\\(\\)\\-])")
        .captures(ua)
    {
        let family = "Archos $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("ARCHOS; GOGI; ([^;]+);").captures(ua) {
        let family = "Archos $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:ARCHOS|Archos)[ _]?(.*?)(?: Build|[;/\\(\\)\\-]|$)")
        .captures(ua)
    {
        let family = "Archos $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AN(?:7|8|9|10|13)[A-Z0-9]{1,4}) Build")
        .captures(ua)
    {
        let family = "Archos $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A28|A32|A43|A70(?:BHT|CHT|HB|S|X)|A101(?:B|C|IT)|A7EB|A7EB-WK|101G9|80G9) Build")
    .captures(ua)
    {
        let family = "Archos $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PAD-FMD[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(BioniQ) ?([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AN\\d[^;/]+|ARCHM\\d+) Build")
        .captures(ua)
    {
        let family = "Arnova $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:ARNOVA|Arnova) ?([^;/]+) Build")
        .captures(ua)
    {
        let family = "Arnova $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:ASSISTANT )?(AP)-?([1789]\\d{2}[A-Z]{0,2}|80104) Build")
        .captures(ua)
    {
        let family = "Assistant $1-$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ME17\\d[^;/]*|ME3\\d{2}[^;/]+|K00[A-Z]|Nexus 10|Nexus 7(?: 2013)?|PadFone[^;/]*|Transformer[^;/]*|TF\\d{3}[^;/]*|eeepc) Build").captures(ua) {
    let family = "Asus $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *ASUS[ _]*([^;/]+) Build")
        .captures(ua)
    {
        let family = "Asus $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Garmin-Asus ([^;/]+) Build")
        .captures(ua)
    {
        let family = "Garmin-Asus $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Garminfone) Build").captures(ua) {
        let family = "Garmin $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; (@TAB-[^;/]+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-(?:07|[^0]\\d)[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Axioo[ _\\-]([^;/]+)|(picopad)[ _\\-]([^;/]+)) Build")
        .captures(ua)
    {
        let family = "Axioo $1$2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(V(?:100|700|800)[^;/]*) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IBAK\\-[^;/]*) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(HY5001|HY6501|X12|X21|I5) Build")
        .captures(ua)
    {
        let family = "Bedove $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(JC-[^;/]*) Build").captures(ua) {
        let family = "Benss $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(BB) ([^;/]+) Build").captures(ua) {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(BlackBird)[ _](I8.*) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(BlackBird)[ _](.*) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([0-9]+BP[EM][^;/]*|Endeavour[^;/]+) Build")
        .captures(ua)
    {
        let family = "Blaupunkt $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:BLU|Blu)[ _\\-])([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:BMOBILE )?(Blu|BLU|DASH [^;/]+|VIVO 4\\.3|TANK 4\\.5) Build")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TOUCH\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AX5\\d+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([Bb]q) ([^;/]+);? Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Maxwell [^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:B-Tab|B-TAB) ?\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Broncho) ([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *CAPTIVA ([^;/]+) Build")
        .captures(ua)
    {
        let family = "Captiva $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(C771|CAL21|IS11CA) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Cat|CAT) ([^;/]+) Build")
        .captures(ua)
    {
        let family = "Cat $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Cat)(Nova.*) Build")
        .captures(ua)
    {
        let family = "Cat $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(INM8002KP|ADM8000KP_[AB]) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:[Cc]elkon[ _\\*]|CELKON[ _\\*])([^;/\\)]+) ?(?:Build|;|\\))")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Build/(?:[Cc]elkon)+_?([^;/_\\)]+)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(CT)-?(\\d+) Build").captures(ua) {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A19|A19Q|A105|A107[^;/\\)]*) ?(?:Build|;|\\))")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TPC[0-9]{4,5}) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Cloudfone)[ _](Excite)([^ ][^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Excite|ICE)[ _](\\d+[^;/]+) Build")
        .captures(ua)
    {
        let family = "Cloudfone $1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Cloudfone|CloudPad)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:Aquila|Clanga|Rapax)[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:CFW-|Kyros )?(MID[0-9]{4}(?:[ABC]|SR|TV)?)(\\(3G\\)-4G| GB 8K| 3G| 8K| GB)? *(?:Build|[;\\)])").captures(ua) {
    let family = "CobyKyros $1$2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *([^;/]*)Coolpad[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(CUBE[ _])?([KU][0-9]+ ?GT.*|A5300) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *CUBOT ([^;/]+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(BOBBY) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Dslide [^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(XCD)[ _]?(28|35) Build")
        .captures(ua)
    {
        let family = "Dell $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(001DL) Build").captures(ua) {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Dell|DELL) (Streak) Build")
        .captures(ua)
    {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(101DL|GS01|Streak Pro[^;/]*) Build")
        .captures(ua)
    {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([Ss]treak ?7) Build").captures(ua) {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Mini-3iX) Build").captures(ua) {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:Dell|DELL)[ _](Aero|Venue|Thunder|Mini.*|Streak[ _]Pro) Build")
            .captures(ua)
    {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Dell[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Dell ([^;/]+) Build").captures(ua) {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TA[CD]-\\d+[^;/]*) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(iP[789]\\d{2}(?:-3G)?|IP10\\d{2}(?:-8GB)?) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AirTab)[ _\\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(F\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(HT-03A) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(HT\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(L\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(N\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(P\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SC\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SH\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SO\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T\\-0[12][^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(DOOV)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Enot|ENOT)[ -]?([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *[^;/]+ Build/(?:CROSS|Cross)+[ _\\-]([^\\)]+)")
        .captures(ua)
    {
        let family = "CROSS $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(CROSS|Cross)[ _\\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Explay[_ ](.+?)(?:[\\)]| Build)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IQ.*) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Fly|FLY)[ _](IQ[^;]+|F[34]\\d+[^;]*);? Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M532|Q572|FJL21) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(G1) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Geeksphone) ([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(G[^F]?FIVE) ([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Gionee)[ _\\-]([^;/]+)(?:/[^;/]+)? Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(GN\\d+[A-Z]?|INFINITY_PASSION|Ctrl_V1) Build")
        .captures(ua)
    {
        let family = "Gionee $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(E3) Build/JOP40D").captures(ua) {
        let family = "Gionee $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\sGIONEE[-\\s_](\\w*)").captures(ua) {
        let family = "Gionee $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:FONE|QUANTUM|INSIGNIA) \\d+[^;/]*|PLAYTAB) Build")
        .captures(ua)
    {
        let family = "GoClever $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *GOCLEVER ([^;/]+) Build")
        .captures(ua)
    {
        let family = "GoClever $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Glass \\d+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Pixel \\w+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(GSmart)[ -]([^/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(imx5[13]_[^/]+) Build")
        .captures(ua)
    {
        let family = "Freescale $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Haier[ _\\-]([^/]+) Build")
        .captures(ua)
    {
        let family = "Haier $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PAD1016) Build").captures(ua) {
        let family = "Haipad $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M701|M7|M8|M9) Build")
        .captures(ua)
    {
        let family = "Haipad $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SN\\d+T[^;\\)/]*)(?: Build|[;\\)])")
        .captures(ua)
    {
        let family = "Hannspree $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Build/HCL ME Tablet ([^;\\)]+)[\\);]")
        .captures(ua)
    {
        let family = "HCLme $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([^;\\/]+) Build/HCL").captures(ua) {
        let family = "HCLme $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MID-?\\d{4}C[EM]) Build")
        .captures(ua)
    {
        let family = "Hena $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(EG\\d{2,}|HS-[^;/]+|MIRA[^;/]+) Build")
        .captures(ua)
    {
        let family = "Hisense $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(andromax[^;/]+) Build")
        .captures(ua)
    {
        let family = "Hisense $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:AMAZE[ _](S\\d+)|(S\\d+)[ _]AMAZE) Build")
        .captures(ua)
    {
        let family = "AMAZE $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PlayBook) Build").captures(ua) {
        let family = "HP $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *HP ([^/]+) Build").captures(ua) {
        let family = "HP $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([^/]+_tenderloin) Build")
        .captures(ua)
    {
        let family = "HP TouchPad";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(HUAWEI |Huawei-)?([UY][^;/]+) Build/(?:Huawei|HUAWEI)([UY][^\\);]+)\\)")
            .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *([^;/]+) Build[/ ]Huawei(MT1-U06|[A-Z]+\\d+[^\\);]+)[^\\);]*\\)")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(S7|M860) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:HUAWEI|Huawei)[ \\-]?)(MediaPad) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:HUAWEI[ _]?|Huawei[ _])?Ascend[ _])([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:HUAWEI|Huawei)[ _\\-]?)((?:G700-|MT-)[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:HUAWEI|Huawei)[ _\\-]?)([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MediaPad[^;]+|SpringBoard) Build/Huawei")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([^;]+) Build/(?:Huawei|HUAWEI)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([Uu])([89]\\d{3}) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Ideos |IDEOS )(S7) Build")
        .captures(ua)
    {
        let family = "Huawei Ideos$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Ideos |IDEOS )([^;/]+\\s*|\\s*)Build")
        .captures(ua)
    {
        let family = "Huawei Ideos$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Orange Daytona|Pulse|Pulse Mini|Vodafone 858|C8500|C8600|C8650|C8660|Nexus 6P|ATH-.+?) Build[/ ]").captures(ua) {
    let family = "Huawei $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *HTC[ _]([^;]+); Windows Phone")
        .captures(ua)
    {
        let family = "HTC $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:HTC[ _/])+([^ _/]+)(?:[/\\\\]1\\.0 | V|/| +)\\d+\\.\\d[\\d\\.]*(?: *Build|\\))")
    .captures(ua)
    {
        let family = "HTC $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:HTC[ _/])+([^ _/]+)(?:[ _/]([^ _/]+))?(?:[/\\\\]1\\.0 | V|/| +)\\d+\\.\\d[\\d\\.]*(?: *Build|\\))").captures(ua) {
    let family = "HTC $1 $2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:HTC[ _/])+([^ _/]+)(?:[ _/]([^ _/]+)(?:[ _/]([^ _/]+))?)?(?:[/\\\\]1\\.0 | V|/| +)\\d+\\.\\d[\\d\\.]*(?: *Build|\\))").captures(ua) {
    let family = "HTC $1 $2 $3";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:HTC[ _/])+([^ _/]+)(?:[ _/]([^ _/]+)(?:[ _/]([^ _/]+)(?:[ _/]([^ _/]+))?)?)?(?:[/\\\\]1\\.0 | V|/| +)\\d+\\.\\d[\\d\\.]*(?: *Build|\\))").captures(ua) {
    let family = "HTC $1 $2 $3 $4";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:(?:HTC|htc)(?:_blocked)*[ _/])+([^ _/;]+)(?: *Build|[;\\)]| - )")
            .captures(ua)
    {
        let family = "HTC $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:(?:HTC|htc)(?:_blocked)*[ _/])+([^ _/]+)(?:[ _/]([^ _/;\\)]+))?(?: *Build|[;\\)]| - )").captures(ua) {
    let family = "HTC $1 $2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:(?:HTC|htc)(?:_blocked)*[ _/])+([^ _/]+)(?:[ _/]([^ _/]+)(?:[ _/]([^ _/;\\)]+))?)?(?: *Build|[;\\)]| - )").captures(ua) {
    let family = "HTC $1 $2 $3";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:(?:HTC|htc)(?:_blocked)*[ _/])+([^ _/]+)(?:[ _/]([^ _/]+)(?:[ _/]([^ _/]+)(?:[ _/]([^ /;]+))?)?)?(?: *Build|[;\\)]| - )").captures(ua) {
    let family = "HTC $1 $2 $3 $4";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("HTC Streaming Player [^\\/]*/[^\\/]*/ htc_([^/]+) /")
        .captures(ua)
    {
        let family = "HTC $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:[;,] *|^)(?:htccn_chs-)?HTC[ _-]?([^;]+?)(?: *Build|clay|Android|-?Mozilla| Opera| Profile| UNTRUSTED|[;/\\(\\)]|$)").captures(ua) {
    let family = "HTC $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(A6277|ADR6200|ADR6300|ADR6350|ADR6400[A-Z]*|ADR6425[A-Z]*|APX515CKT|ARIA|Desire[^_ ]*|Dream|EndeavorU|Eris|Evo|Flyer|HD2|Hero|HERO200|Hero CDMA|HTL21|Incredible|Inspire[A-Z0-9]*|Legend|Liberty|Nexus ?(?:One|HD2)|One|One S C2|One[ _]?(?:S|V|X\\+?)\\w*|PC36100|PG06100|PG86100|S31HT|Sensation|Wildfire)(?: Build|[/;\\(\\)])").captures(ua) {
    let family = "HTC $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(ADR6200|ADR6400L|ADR6425LVW|Amaze|DesireS?|EndeavorU|Eris|EVO|Evo\\d[A-Z]+|HD2|IncredibleS?|Inspire[A-Z0-9]*|Inspire[A-Z0-9]*|Sensation[A-Z0-9]*|Wildfire)[ _-](.+?)(?:[/;\\)]|Build|MIUI|1\\.0)").captures(ua) {
    let family = "HTC $1 $2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *HYUNDAI (T\\d[^/]*) Build")
        .captures(ua)
    {
        let family = "Hyundai $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *HYUNDAI ([^;/]+) Build")
        .captures(ua)
    {
        let family = "Hyundai $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(X700|Hold X|MB-6900) Build")
        .captures(ua)
    {
        let family = "Hyundai $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:iBall[ _\\-])?(Andi)[ _]?(\\d[^;/]*) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IBall)(?:[ _]([^;/]+)|) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(NT-\\d+[^ ;/]*|Net[Tt]AB [^;/]+|Mercury [A-Z]+|iconBIT)(?: S/N:[^;/]+)? Build")
    .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IMO)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *i-?mobile[ _]([^/]+) Build/")
        .captures(ua)
    {
        let family = "i-mobile $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(i-(?:style|note)[^/]*) Build/")
        .captures(ua)
    {
        let family = "i-mobile $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ImPAD) ?(\\d+(?:.)*) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Infinix)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Informer)[ \\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TAB) ?([78][12]4) Build")
        .captures(ua)
    {
        let family = "Intenso $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:Intex[ _])?(AQUA|Aqua)([ _\\.\\-])([^;/]+) *(?:Build|;)")
            .captures(ua)
    {
        let family = "$1$2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:INTEX|Intex)(?:[_ ]([^\\ _;/]+))(?:[_ ]([^\\ _;/]+))? *(?:Build|;)")
            .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *([iI]Buddy)[ _]?(Connect)(?:_|\\?_| )?([^;/]*) *(?:Build|;)")
            .captures(ua)
    {
        let family = "$1 $2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(I-Buddy)[ _]([^;/]+) *(?:Build|;)")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(iOCEAN) ([^/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TP\\d+(?:\\.\\d+)?\\-\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "ionik $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M702pro) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(DE88Plus|MD70) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *IVIO[_\\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TPC-\\d+|JAY-TECH) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(JY-[^;/]+|G[234]S?) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(JXD)[ _\\-]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Karbonn[ _]?([^;/]+) *(?:Build|;)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([^;]+) Build/Karbonn")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(A11|A39|A37|A34|ST8|ST10|ST7|Smart Tab3|Smart Tab2|Titanium S\\d) +Build")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS01|IS03|IS05|IS\\d{2}SH) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS04) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS06|IS\\d{2}PT) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS11S) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS11CA) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS11LG) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS11N) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS11PT) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS12F) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS12M) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IS12S) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW11F) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW11HT) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW11K) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW11M) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW11SC) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW12HT) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW13HT) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ISW?[0-9]{2}[A-Z]{0,2}) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(INFOBAR [^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(JOYPAD|Joypad)[ _]([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Vox|VOX|Arc|K080) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(Kobo Touch)\\b").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(K-Touch)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:EV|KM)-S\\d+[A-Z]?) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Zio|Hydro|Torque|Event|EVENT|Echo|Milano|Rise|URBANO PROGRESSO|WX04K|WX06K|WX10K|KYL21|101K|C5[12]\\d{2}) Build/").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:LAVA[ _])?IRIS[ _\\-]?([^/;\\)]+) *(?:;|\\)|Build)")
        .captures(ua)
    {
        let family = "Iris $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *LAVA[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:(Aspire A1)|(?:LEMON|Lemon)[ _]([^;/]+))_? Build")
        .captures(ua)
    {
        let family = "Lemon $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TAB-1012) Build/").captures(ua) {
        let family = "Lenco $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; Lenco ([^;/]+) Build/").captures(ua) {
        let family = "Lenco $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A1_07|A2107A-H|S2005A-H|S1-37AH0) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Idea[Tp]ab)[ _]([^;/]+);? Build")
        .captures(ua)
    {
        let family = "Lenovo $1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Idea(?:Tab|pad)) ?([^;/]+) Build")
        .captures(ua)
    {
        let family = "Lenovo $1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ThinkPad) ?(Tablet) Build/")
        .captures(ua)
    {
        let family = "Lenovo $1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:LNV-)?(?:=?[Ll]enovo[ _\\-]?|LENOVO[ _])+(.+?)(?:Build|[;/\\)])")
            .captures(ua)
    {
        let family = "Lenovo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("[;,] (?:Vodafone )?(SmartTab) ?(II) ?(\\d+) Build/")
        .captures(ua)
    {
        let family = "Lenovo $1 $2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Ideapad )?K1 Build/")
        .captures(ua)
    {
        let family = "Lenovo Ideapad K1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(3GC101|3GW10[01]|A390) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(?:Lenovo|LENOVO)+[ _\\-]?([^,;:/ ]+)")
        .captures(ua)
    {
        let family = "Lenovo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MFC\\d+)[A-Z]{2}([^;,/]*),? Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(E[34][0-9]{2}|LS[6-8][0-9]{2}|VS[6-9][0-9]+[^;/]+|Nexus 4|Nexus 5X?|GT540f?|Optimus (?:2X|G|4X HD)|OptimusX4HD) *(?:Build|;)").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) =
        crate::regex_cache::cached_regex("[;:] *(L-\\d+[A-Z]|LGL\\d+[A-Z]?)(?:/V\\d+)? *(?:Build|[;\\)])")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(LG-)([A-Z]{1,2}\\d{2,}[^,;/\\)\\(]*?)(?:Build| V\\d+|[,;/\\)\\(]|$)")
            .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(LG[ \\-]|LG)([^;/]+)[;/]? Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(LG)-([^;/]+)/ Mozilla/.*; Android")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Web0S); Linux/(SmartTV)")
        .captures(ua)
    {
        let family = "LG $1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:SMB|smb)[^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Malata|MALATA) ([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MS[45][0-9]{3}|MID0[568][NS]?|MID[1-9]|MID[78]0[1-9]|MID970[1-9]|MID100[1-9]) Build/")
    .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M1052|M806|M9000|M9100|M9701|MID100|MID120|MID125|MID130|MID135|MID140|MID701|MID710|MID713|MID727|MID728|MID731|MID732|MID733|MID735|MID736|MID737|MID760|MID800|MID810|MID820|MID830|MID833|MID835|MID860|MID900|MID930|MID933|MID960|MID980) Build/").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(GenxDroid7|MSD7.*|AX\\d.*|Tab 701|Tab 722) Build/")
        .captures(ua)
    {
        let family = "Maxx $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M-PP[^;/]+|PhonePad ?\\d{2,}[^;/]+) Build")
        .captures(ua)
    {
        let family = "Mediacom $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M-MP[^;/]+|SmartPad ?\\d{2,}[^;/]+) Build")
        .captures(ua)
    {
        let family = "Mediacom $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:MD_)?LIFETAB[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Medion Lifetab $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *MEDION ([^;/]+) Build")
        .captures(ua)
    {
        let family = "Medion $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(M030|M031|M035|M040|M065|m9) Build")
        .captures(ua)
    {
        let family = "Meizu $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:meizu_|MEIZU )(.+?) *(?:Build|[;\\)])")
        .captures(ua)
    {
        let family = "Meizu $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Micromax[ _](A111|A240)|(A111|A240)) Build")
        .captures(ua)
    {
        let family = "Micromax $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Micromax[ _](A\\d{2,3}[^;/]*) Build")
        .captures(ua)
    {
        let family = "Micromax $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A\\d{2}|A[12]\\d{2}|A90S|A110Q) Build")
        .captures(ua)
    {
        let family = "Micromax $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Micromax[ _](P\\d{3}[^;/]*) Build")
        .captures(ua)
    {
        let family = "Micromax $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(P\\d{3}|P\\d{3}\\(Funbook\\)) Build")
        .captures(ua)
    {
        let family = "Micromax $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MITO)[ _\\-]?([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Cynus)[ _](F5|T\\d|.+?) *(?:Build|[;/\\)])")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MODECOM )?(FreeTab) ?([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MODECOM )([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MZ\\d{3}\\+?|MZ\\d{3} 4G|Xoom|XOOM[^;/]*) Build")
        .captures(ua)
    {
        let family = "Motorola $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Milestone )(XT[^;/]*) Build")
        .captures(ua)
    {
        let family = "Motorola $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Motoroi ?x|Droid X|DROIDX) Build")
        .captures(ua)
    {
        let family = "Motorola $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Droid[^;/]*|DROID[^;/]*|Milestone[^;/]*|Photon|Triumph|Devour|Titanium) Build")
    .captures(ua)
    {
        let family = "Motorola $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A555|A85[34][^;/]*|A95[356]|ME[58]\\d{2}\\+?|ME600|ME632|ME722|MB\\d{3}\\+?|MT680|MT710|MT870|MT887|MT917|WX435|WX453|WX44[25]|XT\\d{3,4}[A-Z\\+]*|CL[iI]Q|CL[iI]Q XT) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(Motorola MOT-|Motorola[ _\\-]|MOT\\-?)([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Moto[_ ]?|MOT\\-)([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:MP[DQ]C|MPG\\d{1,4}|MP\\d{3,4}|MID(?:(?:10[234]|114|43|7[247]|8[24]|7)C|8[01]1))[^;/]*) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:MSI[ _])?(Primo\\d+|Enjoy[ _\\-][^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Multilaser[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(My)[_]?(Pad)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(My)\\|?(Phone)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A\\d+)[ _](Duo)? Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(myTab[^;/]*) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(NABI2?-)([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(N-\\d+[CDE]) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(NEC-)(.*) Build/").captures(ua) {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(LT-NA7) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(NXM\\d+[A-z0-9_]*|Next\\d[A-z0-9_ \\-]*|NEXT\\d[A-z0-9_ \\-]*|Nextbook [A-z0-9_ ]*|DATAM803HC|M805)(?: Build|[\\);])").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(Nokia)([ _\\-]*)([^;/]*) Build")
        .captures(ua)
    {
        let family = "$1$2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Nook ?|Barnes & Noble Nook |BN )([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(NOOK )?(BNRV200|BNRV200A|BNTV250|BNTV250A|BNTV400|BNTV600|LogicPD Zoom2) Build")
    .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; Build/(Nook)").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(OP110|OliPad[^;/]+) Build")
        .captures(ua)
    {
        let family = "Olivetti $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *OMEGA[ _\\-](MID[^;/]+) Build")
        .captures(ua)
    {
        let family = "Omega $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(MID7500|MID\\d+) Mozilla/5\\.0 \\(iPad;")
        .captures(ua)
    {
        let family = "Omega $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:CIUS|cius)[^;/]*) Build")
        .captures(ua)
    {
        let family = "Openpeak $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Find ?(?:5|7a)|R8[012]\\d{1,2}|T703\\d{0,1}|U70\\d{1,2}T?|X90\\d{1,2}) Build")
    .captures(ua)
    {
        let family = "Oppo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *OPPO ?([^;/]+) Build/")
        .captures(ua)
    {
        let family = "Oppo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Odys\\-|ODYS\\-|ODYS )([^;/]+) Build")
        .captures(ua)
    {
        let family = "Odys $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SELECT) ?(7) Build").captures(ua) {
        let family = "Odys $1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PEDI)_(PLUS)_(W) Build")
        .captures(ua)
    {
        let family = "Odys $1 $2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AEON|BRAVIO|FUSION|FUSION2IN1|Genio|EOS10|IEOS[^;/]*|IRON|Loox|LOOX|LOOX Plus|Motion|NOON|NOON_PRO|NEXT|OPOS|PEDI[^;/]*|PRIME[^;/]*|STUDYTAB|TABLO|Tablet-PC-4|UNO_X8|XELIO[^;/]*|Xelio ?\\d+ ?[Pp]ro|XENO10|XPRESS PRO) Build").captures(ua) {
    let family = "Odys $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; (ONE [a-zA-Z]\\d+) Build/")
        .captures(ua)
    {
        let family = "OnePlus $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; (ONEPLUS [a-zA-Z]\\d+) Build/")
        .captures(ua)
    {
        let family = "OnePlus $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TP-\\d+) Build/").captures(ua) {
        let family = "Orion $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(G100W?) Build/").captures(ua) {
        let family = "PackardBell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Panasonic)[_ ]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(FZ-A1B|JT-B1) Build").captures(ua) {
        let family = "Panasonic $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(dL1|DL1) Build").captures(ua) {
        let family = "Panasonic $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SKY[ _])?(IM\\-[AT]\\d{3}[^;/]+).* Build/")
        .captures(ua)
    {
        let family = "Pantech $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *((?:ADR8995|ADR910L|ADR930L|ADR930VW|PTL21|P8000)(?: 4G)?) Build/")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Pantech([^;/]+).* Build/")
        .captures(ua)
    {
        let family = "Pantech $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(papyre)[ _\\-]([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Touchlet )?(X10\\.[^;/]+) Build/")
        .captures(ua)
    {
        let family = "Pearl $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; PHICOMM (i800) Build/").captures(ua) {
        let family = "Phicomm $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; PHICOMM ([^;/]+) Build/")
        .captures(ua)
    {
        let family = "Phicomm $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(FWS\\d{3}[^;/]+) Build/")
        .captures(ua)
    {
        let family = "Phicomm $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(D633|D822|D833|T539|T939|V726|W335|W336|W337|W3568|W536|W5510|W626|W632|W6350|W6360|W6500|W732|W736|W737|W7376|W820|W832|W8355|W8500|W8510|W930) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Philips|PHILIPS)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Philips $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("Android 4\\..*; *(M[12356789]|U[12368]|S[123])\\ ?(pro)? Build")
            .captures(ua)
    {
        let family = "Pipo $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MOMO[^;/]+) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Polaroid[ _])?((?:MIDC\\d{3,}|PMID\\d{2,}|PTAB\\d{3,})[^;/]*)(\\/[^;/]*)? Build/")
    .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Polaroid )(Tablet) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(POMP)[ _\\-](.+?) *(?:Build|[;/\\)])")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TB07STA|TB10STA|TB07FTA|TB10FTA) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Positivo )?((?:YPY|Ypy)[^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(MOB-[^;/]+) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *POV[ _\\-]([^;/]+) Build/")
        .captures(ua)
    {
        let family = "POV $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:TAB-PLAYTAB|TAB-PROTAB|PROTAB|PlayTabPro|Mobii[ _\\-]|TAB-P)[^;/]*) Build/")
    .captures(ua)
    {
        let family = "POV $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Prestigio )?((?:PAP|PMP)\\d[^;/]+) Build/")
        .captures(ua)
    {
        let family = "Prestigio $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PLT[0-9]{4}.*) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A2|A5|A8|A900)_?(Classic)? Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Q[Mm]obile)_([^_]+)_([^_]+) Build")
        .captures(ua)
    {
        let family = "Qmobile $2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Q\\-?[Mm]obile)[_ ](A[^;/]+) Build")
        .captures(ua)
    {
        let family = "Qmobile $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Q\\-Smart)[ _]([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Q\\-?[Mm]obile)[ _\\-](S[^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TA1013) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; (RCT\\w+) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(RK\\d+),? Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" Build/(RK\\d+)").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SAMSUNG |Samsung )?((?:Galaxy (?:Note II|S\\d)|GT-I9082|GT-I9205|GT-N7\\d{3}|SM-N9005)[^;/]*)\\/?[^;/]* Build/").captures(ua) {
    let family = "Samsung $1$2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(Google )?(Nexus [Ss](?: 4G)?) Build/")
        .captures(ua)
    {
        let family = "Samsung $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SAMSUNG |Samsung )([^\\/]*)\\/[^ ]* Build/")
        .captures(ua)
    {
        let family = "Samsung $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(Galaxy(?: Ace| Nexus| S ?II+|Nexus S| with MCR 1.2| Mini Plus 4G)?) Build/")
            .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SAMSUNG[ _\\-] *)+([^;/]+) Build")
        .captures(ua)
    {
        let family = "Samsung $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SAMSUNG-)?(GT\\-[BINPS]\\d{4}[^\\/]*)(\\/[^ ]*) Build")
        .captures(ua)
    {
        let family = "Samsung $1$2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:; *|^)((?:GT\\-[BIiNPS]\\d{4}|I9\\d{2}0[A-Za-z\\+]?\\b)[^;/\\)]*?)(?:Build|Linux|MIUI|[;/\\)])").captures(ua) {
    let family = "Samsung $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; (SAMSUNG-)([A-Za-z0-9\\-]+).* Build/")
        .captures(ua)
    {
        let family = "Samsung $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *((?:SCH|SGH|SHV|SHW|SPH|SC|SM)\\-[A-Za-z0-9 ]+)(/?[^ ]*)? Build")
            .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" ((?:SCH)\\-[A-Za-z0-9 ]+)(/?[^ ]*)? Build")
        .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(Behold ?(?:2|II)|YP\\-G[^;/]+|EK-GC100|SCL21|I9300) Build")
            .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SH\\-?\\d\\d[^;/]+|SBM\\d[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SHARP[ -])([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SPX[_\\-]\\d[^;/]*) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SX7\\-PEARL\\.GmbH) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SP[T]?\\-\\d{2}[^;/]*) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SK\\-.*) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:SKYTEX|SX)-([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(IMAGINE [^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SmartQ) ?([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(WF7C|WF10C|SBT[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SBM(?:003SH|005SH|006SH|007SH|102SH)) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(003P|101P|101P11C|102P) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(00\\dZ) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; HTC(X06HT) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(001HT|X06HT) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(201M) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ST\\d{4}.*)Build/ST").captures(ua) {
        let family = "Trekstor $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ST\\d{4}.*) Build/").captures(ua) {
        let family = "Trekstor $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Sony ?Ericsson ?)([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *((?:SK|ST|E|X|LT|MK|MT|WT)\\d{2}[a-z0-9]*(?:-o)?|R800i|U20i) Build")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Xperia (?:A8|Arc|Acro|Active|Live with Walkman|Mini|Neo|Play|Pro|Ray|X\\d+)[^;/]*) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; Sony (Tablet[^;/]+) Build")
        .captures(ua)
    {
        let family = "Sony $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; Sony ([^;/]+) Build").captures(ua) {
        let family = "Sony $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Sony)([A-Za-z0-9\\-]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Xperia [^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(C(?:1[0-9]|2[0-9]|53|55|6[0-9])[0-9]{2}|D[25]\\d{3}|D6[56]\\d{2}) Build")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SGP\\d{3}|SGPT\\d{2}) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(NW-Z1000Series) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("PLAYSTATION 3").captures(ua) {
        let family = "PlayStation 3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(PlayStation (?:Portable|Vita|\\d+))")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:CSL_Spice|Spice|SPICE|CSL)[ _\\-]?)?([Mm][Ii])([ _\\-])?(\\d{3}[^;/]*) Build/")
    .captures(ua)
    {
        let family = "$1$2$3$4";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Sprint )(.+?) *(?:Build|[;/])")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(Sprint)[: ]([^;,/ ]+)")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TAGI[ ]?)(MID) ?([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1$2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Oyster500|Opal 800) Build")
        .captures(ua)
    {
        let family = "Tecmobile $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TECNO[ _])([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Android for (Telechips|Techvision) ([^ ]+) ")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Hub2) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PAD) ?(100[12]) Build/")
        .captures(ua)
    {
        let family = "Terra $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T[BM]-\\d{3}[^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(tolino [^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *Build/.* (TOLINO_BROWSER)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:CJ[ -])?(ThL|THL)[ -]([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T100|T200|T5|W100|W200|W8s) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Mobile[ _]G2[ _]Touch) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Mobile[ _]G2) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Mobile myTouch Q) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Mobile myTouch) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Mobile_Espresso) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(T-Mobile G1) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("\\b(T-Mobile ?)?(myTouch)[ _]?([34]G)[ _]?([^\\/]*) (?:Mozilla|Build)")
            .captures(ua)
    {
        let family = "$1$2 $3 $4";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(T-Mobile)_([^_]+)_(.*) Build")
        .captures(ua)
    {
        let family = "$1 $2 $3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(T-Mobile)[_ ]?(.*?)Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (ATP[0-9]{4}) Build").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" *(TOOKY)[ _\\-]([^;/]+) ?(?:Build|;)")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(TOSHIBA_AC_AND_AZ|TOSHIBA_FOLIO_AND_A|FOLIO_AND_A)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([Ff]olio ?100) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AT[0-9]{2,3}(?:\\-A|LE\\-A|PE\\-A|SE|a)?|AT7-A|AT1S0|Hikari-iFrame/WDPF-[^;/]+|THRiVE|Thrive) Build/").captures(ua) {
    let family = "Toshiba $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *(TM-MID\\d+[^;/]+|TOUCHMATE|MID-750) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TM-SM\\d+[^;/]+) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A10 [Bb]asic2?) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(TREQ[ _\\-])([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(X-?5|X-?3) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(A502\\+?|A936|A603|X1|X2) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(TOUCH(?:TAB|PAD).+?) Build/")
        .captures(ua)
    {
        let family = "Versus $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(VERTU) ([^;/]+) Build/").captures(ua) {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Videocon)[ _\\-]([^;/]+) *(?:Build|;)")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (VT\\d{2}[A-Za-z]*) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *((?:ViewPad|ViewPhone|VSD)[^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ViewSonic-)([^;/]+) Build/")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(GTablet.*) Build/").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *([Vv]ivo)[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Vodafone) (.*) Build/").captures(ua) {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Walton[ _\\-])?(Primo[ _\\-][^;/]+) Build")
        .captures(ua)
    {
        let family = "Walton $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:WIKO[ \\-])?(CINK\\+?|BARRY|BLOOM|DARKFULL|DARKMOON|DARKNIGHT|DARKSIDE|FIZZ|HIGHWAY|IGGY|OZZY|RAINBOW|STAIRWAY|SUBLIM|WAX|CINK [^;/]+) Build/").captures(ua) {
    let family = "Wiko $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *WellcoM-([^;/]+) Build")
        .captures(ua)
    {
        let family = "Wellcom $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:(WeTab)-Browser|; (wetab) Build)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(AT-AS[^;/]+) Build").captures(ua) {
        let family = "Wolfgang $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Woxter|Wxt) ([^;/]+) Build")
        .captures(ua)
    {
        let family = "Woxter $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Xenta |Luna )?(TAB[234][0-9]{2}|TAB0[78]-\\d{3}|TAB0?9-\\d{3}|TAB1[03]-\\d{3}|SMP\\d{2}-\\d{3}) Build/").captures(ua) {
    let family = "Yarvik $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) =
        crate::regex_cache::cached_regex("; *([A-Z]{2,4})(M\\d{3,}[A-Z]{2})([^;\\)\\/]*)(?: Build|[;\\)])")
            .captures(ua)
    {
        let family = "Yifang $1$2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *((MI|HM|MI-ONE|Redmi)[ -](NOTE |Note )?[^;/]*) (Build|MIUI)/")
            .captures(ua)
    {
        let family = "XiaoMi $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *XOLO[ _]([^;/]*tab.*) Build")
        .captures(ua)
    {
        let family = "Xolo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *XOLO[ _]([^;/]+) Build")
        .captures(ua)
    {
        let family = "Xolo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(q\\d0{2,3}[a-z]?) Build")
        .captures(ua)
    {
        let family = "Xolo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(PAD ?[79]\\d+[^;/]*|TelePAD\\d+[^;/]) Build")
        .captures(ua)
    {
        let family = "Xoro $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; *(?:(?:ZOPO|Zopo)[ _]([^;/]+)|(ZP ?(?:\\d{2}[^;/]+|C2))|(C[2379])) Build")
            .captures(ua)
    {
        let family = "$1$2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ZiiLABS) (Zii[^;/]*) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Zii)_([^;/]*) Build").captures(ua) {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(ARIZONA|(?:ATLAS|Atlas) W|D930|Grand (?:[SX][^;]*|Era|Memo[^;]*)|JOE|(?:Kis|KIS)\\b[^;]*|Libra|Light [^;]*|N8[056][01]|N850L|N8000|N9[15]\\d{2}|N9810|NX501|Optik|(?:Vip )Racer[^;]*|RacerII|RACERII|San Francisco[^;]*|V9[AC]|V55|V881|Z[679][0-9]{2}[A-z]?) Build").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("; *([A-Z]\\d+)_USA_[^;]* Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(SmartTab\\d+)[^;]* Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Blade|BLADE|ZTE-BLADE)([^;/]*) Build")
        .captures(ua)
    {
        let family = "ZTE Blade$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:Skate|SKATE|ZTE-SKATE)([^;/]*) Build")
        .captures(ua)
    {
        let family = "ZTE Skate$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(Orange |Optimus )(Monte Carlo|San Francisco) Build")
        .captures(ua)
    {
        let family = "$1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(?:ZXY-ZTE_|ZTE\\-U |ZTE[\\- _]|ZTE-C[_ ])([^;/]+) Build")
        .captures(ua)
    {
        let family = "ZTE $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; (BASE) (lutea|Lutea 2|Tab[^;]*) Build")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; (Avea inTouch 2|soft stone|tmn smart a7|Movistar[ _]Link) Build")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(vp9plus)\\)").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("; ?(Cloud[ _]Z5|z1000|Z99 2G|z99|z930|z999|z990|z909|Z919|z900) Build/")
            .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFOT|Kindle Fire) Build\\b")
        .captures(ua)
    {
        let family = "Kindle Fire";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFOTE|Amazon Kindle Fire2) Build\\b")
        .captures(ua)
    {
        let family = "Kindle Fire 2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFTT) Build\\b").captures(ua) {
        let family = "Kindle Fire HD";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFJWI) Build\\b").captures(ua) {
        let family = "Kindle Fire HD 8.9\" WiFi";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFJWA) Build\\b").captures(ua) {
        let family = "Kindle Fire HD 8.9\" 4G";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFSOWI) Build\\b").captures(ua) {
        let family = "Kindle Fire HD 7\" WiFi";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFTHWI) Build\\b").captures(ua) {
        let family = "Kindle Fire HDX 7\" WiFi";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFTHWA) Build\\b").captures(ua) {
        let family = "Kindle Fire HDX 7\" 4G";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFAPWI) Build\\b").captures(ua) {
        let family = "Kindle Fire HDX 8.9\" WiFi";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(KFAPWA) Build\\b").captures(ua) {
        let family = "Kindle Fire HDX 8.9\" 4G";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?Amazon ([^;/]+) Build\\b")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(Kindle) Build\\b").captures(ua) {
        let family = "Kindle";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ?(Silk)/(\\d+)\\.(\\d+)(?:\\.([0-9\\-]+))? Build\\b")
        .captures(ua)
    {
        let family = "Kindle Fire";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Kindle)/(\\d+\\.\\d+)").captures(ua) {
        let family = "Kindle";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Silk|Kindle)/(\\d+)\\.")
        .captures(ua)
    {
        let family = "Kindle";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(sprd)\\-([^/]+)/").captures(ua) {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(H\\d{2}00\\+?) Build")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(iphone|iPhone5) Build/")
        .captures(ua)
    {
        let family = "Xianghe $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; *(e\\d{4}[a-z]?_?v\\d+|v89_[^;/]+)[^;/]+ Build/")
        .captures(ua)
    {
        let family = "Xianghe $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\bUSCC[_\\-]?([^ ;/\\)]+)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:ALCATEL)[^;]*; *([^;,\\)]+)").captures(ua) {
    let family = "Alcatel $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?|WpsLondonTest; ?)?(?:ASUS|Asus)[^;]*; *([^;,\\)]+)").captures(ua) {
    let family = "Asus $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:DELL|Dell)[^;]*; *([^;,\\)]+)").captures(ua) {
    let family = "Dell $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?|WpsLondonTest; ?)?(?:HTC|Htc|HTC_blocked[^;]*)[^;]*; *(?:HTC)?([^;,\\)]+)").captures(ua) {
    let family = "HTC $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:HUAWEI)[^;]*; *(?:HUAWEI )?([^;,\\)]+)").captures(ua) {
    let family = "Huawei $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:LG|Lg)[^;]*; *(?:LG[ \\-])?([^;,\\)]+)").captures(ua) {
    let family = "LG $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:rv:11; )?(?:NOKIA|Nokia)[^;]*; *(?:NOKIA ?|Nokia ?|LUMIA ?|[Ll]umia ?)*(\\d{3,}[^;\\)]*)").captures(ua) {
    let family = "Lumia $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:NOKIA|Nokia)[^;]*; *(RM-\\d{3,})").captures(ua) {
    let family = "Nokia $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(?:Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)]|WPDesktop;) ?(?:ARM; ?Touch; ?|Touch; ?)?(?:NOKIA|Nokia)[^;]*; *(?:NOKIA ?|Nokia ?|LUMIA ?|[Ll]umia ?)*([^;\\)]+)").captures(ua) {
    let family = "Nokia $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?)?(?:Microsoft(?: Corporation)?)[^;]*; *([^;,\\)]+)").captures(ua) {
    let family = "Microsoft $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?|WpsLondonTest; ?)?(?:SAMSUNG)[^;]*; *(?:SAMSUNG )?([^;,\\.\\)]+)").captures(ua) {
    let family = "Samsung $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?|WpsLondonTest; ?)?(?:TOSHIBA|FujitsuToshibaMobileCommun)[^;]*; *([^;,\\)]+)").captures(ua) {
    let family = "Toshiba $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Windows Phone [^;]+; .*?IEMobile/[^;\\)]+[;\\)] ?(?:ARM; ?Touch; ?|Touch; ?|WpsLondonTest; ?)?([^;]+); *([^;,\\)]+)").captures(ua) {
    let family = "$1 $2";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(?:^|; )SAMSUNG\\-([A-Za-z0-9\\-]+).* Bada/")
        .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\(Mobile; ALCATEL ?(One|ONE) ?(Touch|TOUCH) ?([^;/]+)(?:/[^;]+)?; rv:[^\\)]+\\) Gecko/[^\\/]+ Firefox/").captures(ua) {
    let family = "Alcatel $1 $2 $3";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) =
        crate::regex_cache::cached_regex("\\(Mobile; (?:ZTE([^;]+)|(OpenC)); rv:[^\\)]+\\) Gecko/[^\\/]+ Firefox/")
            .captures(ua)
    {
        let family = "ZTE $1$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Nokia(N[0-9]+)([A-z_\\-][A-z0-9_\\-]*)")
        .captures(ua)
    {
        let family = "Nokia $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:NOKIA|Nokia)(?:\\-| *)(?:([A-Za-z0-9]+)\\-[0-9a-f]{32}|([A-Za-z0-9\\-]+)(?:UCBrowser)|([A-Za-z0-9\\-]+))").captures(ua) {
    let family = "Nokia $1$2$3";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("Lumia ([A-Za-z0-9\\-]+)").captures(ua) {
        let family = "Lumia $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\(Symbian; U; S60 V5; [A-z]{2}\\-[A-z]{2}; (SonyEricsson|Samsung|Nokia|LG)([^;/]+)\\)")
    .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\(Symbian(?:/3)?; U; ([^;]+);")
        .captures(ua)
    {
        let family = "Nokia $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("BB10; ([A-Za-z0-9\\- ]+)\\)")
        .captures(ua)
    {
        let family = "BlackBerry $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Play[Bb]ook.+RIM Tablet OS")
        .captures(ua)
    {
        let family = "BlackBerry Playbook";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Black[Bb]erry ([0-9]+);").captures(ua) {
        let family = "BlackBerry $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Black[Bb]erry([0-9]+)").captures(ua) {
        let family = "BlackBerry $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Black[Bb]erry;").captures(ua) {
        let family = "BlackBerry";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Pre|Pixi)/\\d+\\.\\d+").captures(ua) {
        let family = "Palm $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Palm([0-9]+)").captures(ua) {
        let family = "Palm $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Treo([A-Za-z0-9]+)").captures(ua) {
        let family = "Palm Treo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("webOS.*(P160U(?:NA)?)/(\\d+).(\\d+)")
        .captures(ua)
    {
        let family = "HP Vee";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Touch[Pp]ad)/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "HP TouchPad";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HPiPAQ([A-Za-z0-9]+)/\\d+.\\d+")
        .captures(ua)
    {
        let family = "HP iPAQ $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("PDA; (PalmOS)/sony/model ([a-z]+)/Revision")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Apple\\s?TV)").captures(ua) {
        let family = "AppleTV";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(QtCarBrowser)").captures(ua) {
        let family = "Tesla Model S";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPhone|iPad|iPod)(\\d+,\\d+)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPad)(?:;| Simulator;)").captures(ua) {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPod)(?:;| touch;| Simulator;)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPhone)(?:;| Simulator;)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("iPhone").captures(ua) {
        let family = "iPhone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/\\d.*\\(((?:Mac|iMac|PowerMac|PowerBook)[^\\d]*)(\\d+)(?:,|%2C)(\\d+)")
    .captures(ua)
    {
        let family = "$1$2,$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/\\d+\\.\\d+\\.\\d+ \\(x86_64\\)")
        .captures(ua)
    {
        let family = "Mac";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/\\d").captures(ua) {
        let family = "iOS-Device";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("acer_([A-Za-z0-9]+)_").captures(ua) {
        let family = "Acer $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:ALCATEL|Alcatel)-([A-Za-z0-9\\-]+)")
        .captures(ua)
    {
        let family = "Alcatel $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:Amoi|AMOI)\\-([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "Amoi $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:; |\\/|^)((?:Transformer (?:Pad|Prime) |Transformer |PadFone[ _]?)[A-Za-z0-9]*)")
    .captures(ua)
    {
        let family = "Asus $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:asus.*?ASUS|Asus|ASUS|asus)[\\- ;]*((?:Transformer (?:Pad|Prime) |Transformer |Padfone |Nexus[ _])?[A-Za-z0-9]+)").captures(ua) {
    let family = "Asus $1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("\\bBIRD[ \\-\\.]([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "Bird $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\bDell ([A-Za-z0-9]+)").captures(ua) {
        let family = "Dell $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("DoCoMo/2\\.0 ([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "DoCoMo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("([A-Za-z0-9]+)_W;FOMA").captures(ua) {
        let family = "DoCoMo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("([A-Za-z0-9]+);FOMA").captures(ua) {
        let family = "DoCoMo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(?:HTC/|HTC/[a-z0-9]+/)?HTC[ _\\-;]? *(.*?)(?:-?Mozilla|fingerPrint|[;/\\(\\)]|$)")
    .captures(ua)
    {
        let family = "HTC $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Huawei([A-Za-z0-9]+)").captures(ua) {
        let family = "Huawei $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HUAWEI-([A-Za-z0-9]+)").captures(ua) {
        let family = "Huawei $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("vodafone([A-Za-z0-9]+)").captures(ua) {
        let family = "Huawei Vodafone $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("i\\-mate ([A-Za-z0-9]+)").captures(ua) {
        let family = "i-mate $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Kyocera\\-([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "Kyocera $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("KWC\\-([A-Za-z0-9]+)").captures(ua) {
        let family = "Kyocera $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Lenovo[_\\-]([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "Lenovo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)/[0-9]+\\.[0-9]+\\.[0-9]+ \\([^;]*; *(LG)E *; *([^;]*) *;[^;]*;[^;]*;\\)")
    .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)/1\\.1\\.1.*CE-HTML/1\\.\\d;(Vendor/)*(THOM[^;]*?)[;\\s](?:.*SW-Version/.*)*(LF[^;]+);?").captures(ua) {
    let family = "$1";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)(?:/1\\.1\\.1)?(?: ?\\(;;;\\))?; *CE-HTML(?:/1\\.\\d)?; *([^ ]+) ([^;]+);")
    .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)/1\\.1\\.1 \\(;;;\\) Maple_2011")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)/[0-9]+\\.[0-9]+\\.[0-9]+ \\([^;]*; *(?:CUS:([^;]*)|([^;]+)) *; *([^;]*) *;.*;")
    .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(HbbTV)/[0-9]+\\.[0-9]+\\.[0-9]+")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("LGE; (?:Media\\/)?([^;]*);[^;]*;[^;]*;?\\); \"?LG NetCast(\\.TV|\\.Media|)-\\d+")
    .captures(ua)
    {
        let family = "NetCast$2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("InettvBrowser/[0-9]+\\.[0-9A-Z]+ \\([^;]*;(Sony)([^;]*);[^;]*;[^\\)]*\\)")
            .captures(ua)
    {
        let family = "Inettv";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("InettvBrowser/[0-9]+\\.[0-9A-Z]+ \\([^;]*;([^;]*);[^;]*;[^\\)]*\\)")
            .captures(ua)
    {
        let family = "Inettv";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:InettvBrowser|TSBNetTV|NETTV|HBBTV)")
        .captures(ua)
    {
        let family = "Inettv";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Series60/\\d\\.\\d (LG)[\\-]?([A-Za-z0-9 \\-]+)")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(?:LGE[ \\-]LG\\-(?:AX)?|LGE |LGE?-LG|LGE?[ \\-]|LG[ /\\-]|lg[\\-])([A-Za-z0-9]+)\\b")
    .captures(ua)
    {
        let family = "LG $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:^LG[\\-]?|^LGE[\\-/]?)([A-Za-z]+[0-9]+[A-Za-z]*)")
        .captures(ua)
    {
        let family = "LG $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^LG([0-9]+[A-Za-z]*)").captures(ua) {
        let family = "LG $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(KIN\\.[^ ]+) (\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Microsoft $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:MSIE|XBMC).*\\b(Xbox)\\b")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("; ARM; Trident/6\\.0; Touch[\\);]")
        .captures(ua)
    {
        let family = "Microsoft Surface RT";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Motorola\\-([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "Motorola $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("MOTO\\-([A-Za-z0-9]+)").captures(ua) {
        let family = "Motorola $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("MOT\\-([A-z0-9][A-z0-9\\-]*)")
        .captures(ua)
    {
        let family = "Motorola $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Nintendo WiiU").captures(ua) {
        let family = "Nintendo Wii U";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Nintendo (DS|3DS|DSi|Wii);")
        .captures(ua)
    {
        let family = "Nintendo $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:Pantech|PANTECH)[ _-]?([A-Za-z0-9\\-]+)")
        .captures(ua)
    {
        let family = "Pantech $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Philips([A-Za-z0-9]+)").captures(ua) {
        let family = "Philips $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Philips ([A-Za-z0-9]+)").captures(ua) {
        let family = "Philips $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(SMART-TV); .* Tizen ").captures(ua) {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("SymbianOS/9\\.\\d.* Samsung[/\\-]([A-Za-z0-9 \\-]+)")
        .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Samsung)(SGH)(i[0-9]+)").captures(ua) {
        let family = "$1 $2$3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("SAMSUNG-ANDROID-MMS/([^;/]+)")
        .captures(ua)
    {
        let family = "$1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("SAMSUNG(?:; |[ -/])([A-Za-z0-9\\-]+)")
        .captures(ua)
    {
        let family = "Samsung $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Dreamcast)").captures(ua) {
        let family = "Sega $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^SIE-([A-Za-z0-9]+)").captures(ua) {
        let family = "Siemens $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Softbank/[12]\\.0/([A-Za-z0-9]+)")
        .captures(ua)
    {
        let family = "Softbank $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("SonyEricsson ?([A-Za-z0-9\\-]+)")
        .captures(ua)
    {
        let family = "Ericsson $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android [^;]+; ([^ ]+) (Sony)/")
        .captures(ua)
    {
        let family = "$2 $1";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Sony)(?:BDP\\/|\\/)?([^ /;\\)]+)[ /;\\)]")
        .captures(ua)
    {
        let family = "$1 $2";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Puffin/[\\d\\.]+IT").captures(ua) {
        let family = "iPad";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Puffin/[\\d\\.]+IP").captures(ua) {
        let family = "iPhone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Puffin/[\\d\\.]+AT").captures(ua) {
        let family = "Generic Tablet";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Puffin/[\\d\\.]+AP").captures(ua) {
        let family = "Generic Smartphone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android[\\- ][\\d]+\\.[\\d]+; [A-Za-z]{2}\\-[A-Za-z]{0,2}; WOWMobile (.+) Build[/ ]")
    .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android[\\- ][\\d]+\\.[\\d]+\\-update1; [A-Za-z]{2}\\-[A-Za-z]{0,2} *; *(.+?) Build[/ ]")
    .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android[\\- ][\\d]+(?:\\.[\\d]+){1,2}; *[A-Za-z]{2}[_\\-][A-Za-z]{0,2}\\-? *; *(.+?) Build[/ ]").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) =
        crate::regex_cache::cached_regex("Android[\\- ][\\d]+(?:\\.[\\d]+){1,2}; *[A-Za-z]{0,2}\\- *; *(.+?) Build[/ ]")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Android[\\- ][\\d]+(?:\\.[\\d]+){1,2}; *[a-z]{0,2}[_\\-]?[A-Za-z]{0,2};? Build[/ ]")
    .captures(ua)
    {
        let family = "Generic Smartphone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("Android[\\- ][\\d]+(?:\\.[\\d]+){1,2}; *\\-?[A-Za-z]{2}; *(.+?) Build[/ ]")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("Android[\\- ][\\d]+(?:\\.[\\d]+){1,2}(?:;.*)?; *(.+?) Build[/ ]")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(GoogleTV)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(WebTV)/\\d+.\\d+").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Roku)/DVP-\\d+\\.\\d+").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Android 3\\.\\d|Opera Tablet|Tablet; .+Firefox/|Android.*(?:Tab|Pad))")
            .captures(ua)
    {
        let family = "Generic Tablet";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Symbian|\\bS60(Version|V\\d)|\\bS60\\b|\\((Series 60|Windows Mobile|Palm OS|Bada); Opera Mini|Windows CE|Opera Mobi|BREW|Brew|Mobile; .+Firefox/|iPhone OS|Android|MobileSafari|Windows *Phone|\\(webOS/|PalmOS)").captures(ua) {
    let family = "Generic Smartphone";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(hiptop|avantgo|plucker|xiino|blazer|elaine)")
        .captures(ua)
    {
        let family = "Generic Smartphone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(bot|zao|borg|DBot|oegp|silk|Xenu|zeal|^NING|CCBot|crawl|htdig|lycos|slurp|teoma|voila|yahoo|Sogou|CiBra|Nutch|^Java/|^JNLP/|Daumoa|Genieo|ichiro|larbin|pompos|Scrapy|snappy|speedy|spider|msnbot|msrbot|vortex|^vortex|crawler|favicon|indexer|Riddler|scooter|scraper|scrubby|WhatWeb|WinHTTP|bingbot|BingPreview|openbot|gigabot|furlbot|polybot|seekbot|^voyager|archiver|Icarus6j|mogimogi|Netvibes|blitzbot|altavista|charlotte|findlinks|Retreiver|TLSProber|WordPress|SeznamBot|ProoXiBot|wsr\\-agent|Squrl Java|EtaoSpider|PaperLiBot|SputnikBot|A6\\-Indexer|netresearch|searchsight|baiduspider|YisouSpider|ICC\\-Crawler|http%20client|Python-urllib|dataparksearch|converacrawler|Screaming Frog|AppEngine-Google|YahooCacheSystem|fast\\-webcrawler|Sogou Pic Spider|semanticdiscovery|Innovazion Crawler|facebookexternalhit|Google.*/\\+/web/snippet|Google-HTTP-Java-Client|BlogBridge|IlTrovatore-Setaccio|InternetArchive|GomezAgent|WebThumbnail|heritrix|NewsGator|PagePeeker|Reaper|ZooShot|holmes|NL-Crawler|Pingdom|StatusCake|WhatsApp|masscan|Google Web Preview|Qwantify)").captures(ua) {
    let family = "Spide";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("^(1207|3gso|4thp|501i|502i|503i|504i|505i|506i|6310|6590|770s|802s|a wa|acer|acs\\-|airn|alav|asus|attw|au\\-m|aur |aus |abac|acoo|aiko|alco|alca|amoi|anex|anny|anyw|aptu|arch|argo|bmobile|bell|bird|bw\\-n|bw\\-u|beck|benq|bilb|blac|c55/|cdm\\-|chtm|capi|comp|cond|dall|dbte|dc\\-s|dica|ds\\-d|ds12|dait|devi|dmob|doco|dopo|dorado|el(?:38|39|48|49|50|55|58|68)|el[3456]\\d{2}dual|erk0|esl8|ex300|ez40|ez60|ez70|ezos|ezze|elai|emul|eric|ezwa|fake|fly\\-|fly_|g\\-mo|g1 u|g560|gf\\-5|grun|gene|go.w|good|grad|hcit|hd\\-m|hd\\-p|hd\\-t|hei\\-|hp i|hpip|hs\\-c|htc |htc\\-|htca|htcg)").captures(ua) {
    let family = "Generic Feature Phone";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("^(htcp|htcs|htct|htc_|haie|hita|huaw|hutc|i\\-20|i\\-go|i\\-ma|i\\-mobile|i230|iac|iac\\-|iac/|ig01|im1k|inno|iris|jata|kddi|kgt|kgt/|kpt |kwc\\-|klon|lexi|lg g|lg\\-a|lg\\-b|lg\\-c|lg\\-d|lg\\-f|lg\\-g|lg\\-k|lg\\-l|lg\\-m|lg\\-o|lg\\-p|lg\\-s|lg\\-t|lg\\-u|lg\\-w|lg/k|lg/l|lg/u|lg50|lg54|lge\\-|lge/|leno|m1\\-w|m3ga|m50/|maui|mc01|mc21|mcca|medi|meri|mio8|mioa|mo01|mo02|mode|modo|mot |mot\\-|mt50|mtp1|mtv |mate|maxo|merc|mits|mobi|motv|mozz|n100|n101|n102|n202|n203|n300|n302|n500|n502|n505|n700|n701|n710|nec\\-|nem\\-|newg|neon)").captures(ua) {
    let family = "Generic Feature Phone";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("^(netf|noki|nzph|o2 x|o2\\-x|opwv|owg1|opti|oran|ot\\-s|p800|pand|pg\\-1|pg\\-2|pg\\-3|pg\\-6|pg\\-8|pg\\-c|pg13|phil|pn\\-2|pt\\-g|palm|pana|pire|pock|pose|psio|qa\\-a|qc\\-2|qc\\-3|qc\\-5|qc\\-7|qc07|qc12|qc21|qc32|qc60|qci\\-|qwap|qtek|r380|r600|raks|rim9|rove|s55/|sage|sams|sc01|sch\\-|scp\\-|sdk/|se47|sec\\-|sec0|sec1|semc|sgh\\-|shar|sie\\-|sk\\-0|sl45|slid|smb3|smt5|sp01|sph\\-|spv |spv\\-|sy01|samm|sany|sava|scoo|send|siem|smar|smit|soft|sony|t\\-mo|t218|t250|t600|t610|t618|tcl\\-|tdg\\-|telm|tim\\-|ts70|tsm\\-|tsm3|tsm5|tx\\-9|tagt)").captures(ua) {
    let family = "Generic Feature Phone";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("^(talk|teli|topl|tosh|up.b|upg1|utst|v400|v750|veri|vk\\-v|vk40|vk50|vk52|vk53|vm40|vx98|virg|vertu|vite|voda|vulc|w3c |w3c\\-|wapj|wapp|wapu|wapm|wig |wapi|wapr|wapv|wapy|wapa|waps|wapt|winc|winw|wonu|x700|xda2|xdag|yas\\-|your|zte\\-|zeto|aste|audi|avan|blaz|brew|brvw|bumb|ccwa|cell|cldc|cmd\\-|dang|eml2|fetc|hipt|http|ibro|idea|ikom|ipaq|jbro|jemu|jigs|keji|kyoc|kyok|libw|m\\-cr|midp|mmef|moto|mwbp|mywa|newt|nok6|o2im|pant|pdxg|play|pluc|port|prox|rozo|sama|seri|smal|symb|treo|upsi|vx52|vx53|vx60|vx61|vx70|vx80|vx81|vx83|vx85|wap\\-|webc|whit|wmlb|xda\\-|xda_)").captures(ua) {
    let family = "Generic Feature Phone";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("^(Ice)$").captures(ua) {
        let family = "Generic Feature Phone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(wap[\\-\\ ]browser|maui|netfront|obigo|teleca|up\\.browser|midp|Opera Mini)")
            .captures(ua)
    {
        let family = "Generic Feature Phone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HbbTV/\\d+\\.\\d+\\.\\d+ \\( ;(LG)E ;NetCast 4.0")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2013";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HbbTV/\\d+\\.\\d+\\.\\d+ \\( ;(LG)E ;NetCast 3.0")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2012";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HbbTV/1.1.1 \\(;;;\\) Maple_2011")
        .captures(ua)
    {
        let family = "Samsung";
        let major = "2011";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("HbbTV/\\d+\\.\\d+\\.\\d+ \\(;(Samsung);SmartTV([0-9]{4});.*FXPDEUC")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = "UE40F7000";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("HbbTV/\\d+\\.\\d+\\.\\d+ \\(;(Samsung);SmartTV([0-9]{4});.*MST12DEUC")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = "UE32F4500";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HbbTV/1.1.1 \\(; (Philips);.*NETTV/4")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2013";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HbbTV/1.1.1 \\(; (Philips);.*NETTV/3")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2012";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("HbbTV/1.1.1 \\(; (Philips);.*NETTV/2")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2011";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("HbbTV/\\d+\\.\\d+\\.\\d+.*(firetv)-firefox-plugin (\\d+).(\\d+).(\\d+)")
            .captures(ua)
    {
        let family = "FireHbbTV";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("HbbTV/\\d+\\.\\d+\\.\\d+ \\(.*; ?([a-zA-Z]+) ?;.*(201[1-9]).*\\)")
            .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows Phone) (?:OS[ /])?(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CPU[ +]OS|iPhone[ +]OS|CPU[ +]iPhone)[ +]+(\\d+)[_\\.](\\d+)(?:[_\\.](\\d+))?.*Outlook-iOS-Android").captures(ua) {
    let family = "iOS";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Android)[ \\-/](\\d+)\\.(\\d+)(?:[.\\-]([a-z0-9]+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Donut").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "1";
        let minor = "2";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Eclai").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2";
        let minor = "1";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Froyo").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2";
        let minor = "2";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Gingerbread").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "2";
        let minor = "3";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Android) Honeycomb").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = "3";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^UCWEB.*; (Adr) (\\d+)\\.(\\d+)(?:[.\\-]([a-z0-9]+))?;")
        .captures(ua)
    {
        let family = "Android";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^UCWEB.*; (iPad|iPh|iPd) OS (\\d+)_(\\d+)(?:_(\\d+))?;")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^UCWEB.*; (wds) (\\d+)\\.(\\d+)(?:\\.(\\d+))?;")
        .captures(ua)
    {
        let family = "Windows Phone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("^(JUC).*; ?U; ?(?:Android)?(\\d+)\\.(\\d+)(?:[\\.\\-]([a-z0-9]+))?")
            .captures(ua)
    {
        let family = "Android";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Silk-Accelerated=[a-z]{4,5})")
        .captures(ua)
    {
        let family = "Android";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(XBLWP7)").captures(ua) {
        let family = "Windows Phone";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows ?Mobile)").captures(ua) {
        let family = "Windows Mobile";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows (?:NT 5\\.2|NT 5\\.1))")
        .captures(ua)
    {
        let family = "Windows";
        let major = "XP";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.1)").captures(ua) {
        let family = "Windows";
        let major = "7";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.0)").captures(ua) {
        let family = "Windows";
        let major = "Vista";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Win 9x 4\\.90)").captures(ua) {
        let family = "Windows";
        let major = "ME";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows 98|Windows XP|Windows ME|Windows 95|Windows CE|Windows 7|Windows NT 4\\.0|Windows Vista|Windows 2000|Windows 3.1)").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.2; ARM;)")
        .captures(ua)
    {
        let family = "Windows";
        let major = "RT";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.2)").captures(ua) {
        let family = "Windows";
        let major = "8";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.3; ARM;)")
        .captures(ua)
    {
        let family = "Windows";
        let major = "RT 8.1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.3)").captures(ua) {
        let family = "Windows";
        let major = "8.1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 6\\.4)").captures(ua) {
        let family = "Windows";
        let major = "10";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 10\\.0)").captures(ua) {
        let family = "Windows";
        let major = "10";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows NT 5\\.0)").captures(ua) {
        let family = "Windows";
        let major = "2000";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(WinNT4.0)").captures(ua) {
        let family = "Windows";
        let major = "NT 4.0";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows ?CE)").captures(ua) {
        let family = "Windows";
        let major = "CE";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Win ?(95|98|3.1|NT|ME|2000)")
        .captures(ua)
    {
        let family = "Windows";
        let major = "$1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Win16").captures(ua) {
        let family = "Windows";
        let major = "3.1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Win32").captures(ua) {
        let family = "Windows";
        let major = "95";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^Box.*Windows/([\\d.]+);")
        .captures(ua)
    {
        let family = "Windows";
        let major = "$1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Tizen)[/ ](\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("((?:Mac[ +]?|; )OS[ +]X)[\\s+/](?:(\\d+)[_.](\\d+)(?:[_.](\\d+))?|Mach-O)")
            .captures(ua)
    {
        let family = "Mac OS X";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex(" (Dar)(win)/(9).(\\d+).*\\((?:i386|x86_64|Power Macintosh)\\)")
            .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "5";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Dar)(win)/(10).(\\d+).*\\((?:i386|x86_64)\\)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "6";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Dar)(win)/(11).(\\d+).*\\((?:i386|x86_64)\\)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "7";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Dar)(win)/(12).(\\d+).*\\((?:i386|x86_64)\\)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "8";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex(" (Dar)(win)/(13).(\\d+).*\\((?:i386|x86_64)\\)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "9";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Mac_PowerPC").captures(ua) {
        let family = "Mac OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(?:PPC|Intel) (Mac OS X)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^Box.*;(Darwin)/(10)\\.(1\\d)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Apple\\s?TV)(?:/(\\d+)\\.(\\d+))?")
        .captures(ua)
    {
        let family = "ATV OS X";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CPU[ +]OS|iPhone[ +]OS|CPU[ +]iPhone|CPU IPhone OS)[ +]+(\\d+)[_\\.](\\d+)(?:[_\\.](\\d+))?").captures(ua) {
    let family = "iOS";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(iPhone|iPad|iPod); Opera")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iPhone|iPad|iPod).*Mac OS X.*Version/(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/(5)48\\.0\\.3.* Darwin/11\\.0\\.0")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/(5)48\\.(0)\\.4.* Darwin/(1)1\\.0\\.0")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/(5)48\\.(1)\\.4")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/(4)85\\.1(3)\\.9")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/(6)09\\.(1)\\.4")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/(6)(0)9").captures(ua) {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/6(7)2\\.(1)\\.13")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/6(7)2\\.(1)\\.(1)4")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CF)(Network)/6(7)(2)\\.1\\.15")
        .captures(ua)
    {
        let family = "iOS";
        let major = "7";
        let minor = "1";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/6(7)2\\.(0)\\.(?:2|8)")
        .captures(ua)
    {
        let family = "iOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CFNetwork)/709\\.1").captures(ua) {
        let family = "iOS";
        let major = "8";
        let minor = "0.b5";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CF)(Network)/711\\.(\\d)")
        .captures(ua)
    {
        let family = "iOS";
        let major = "8";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CF)(Network)/(720)\\.(\\d)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "10";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CF)(Network)/(760)\\.(\\d)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "11";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CF)(Network)/758\\.(\\d)")
        .captures(ua)
    {
        let family = "iOS";
        let major = "9";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CF)(Network)/808\\.(\\d)")
        .captures(ua)
    {
        let family = "iOS";
        let major = "10";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/16\\.\\d+.*\\(x86_64\\)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "12";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/15\\.\\d+.*\\(x86_64\\)")
        .captures(ua)
    {
        let family = "Mac OS X";
        let major = "10";
        let minor = "11";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/(9)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "1";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/(10)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "4";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/(11)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "5";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/.* Darwin/(13)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "6";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/6.* Darwin/(14)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "7";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/7.* Darwin/(14)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "8";
        let minor = "0";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/7.* Darwin/(15)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "9";
        let minor = "0";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/16\\.5\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "10";
        let minor = "3";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/16\\.6\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "10";
        let minor = "3";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/16\\.7\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "10";
        let minor = "3";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/(16)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "10";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/17\\.0\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "11";
        let minor = "0";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/17\\.2\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "11";
        let minor = "1";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/17\\.3\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "11";
        let minor = "2";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("CFNetwork/8.* Darwin/(17)\\.\\d+")
        .captures(ua)
    {
        let family = "iOS";
        let major = "11";
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(iOS[ /]|iOS; |iPhone(?:/| v|[ _]OS[/,]|; | OS : |\\d,\\d/|\\d,\\d; )|iPad/)(\\d{1,2})[_\\.](\\d{1,2})(?:[_\\.](\\d+))?").captures(ua) {
    let family = "iOS";
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("\\((iOS);").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(tvOS)/(\\d+).(\\d+)").captures(ua) {
        let family = "tvOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CrOS) [a-z0-9_]+ (\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "Chrome OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("([Dd]ebian)").captures(ua) {
        let family = "Debian";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Linux Mint)(?:/(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Mandriva)(?: Linux)?/(?:[\\d.-]+m[a-z]{2}(\\d+).(\\d))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Symbian[Oo][Ss])[/ ](\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Symbian OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Symbian/3).+NokiaBrowser/7\\.3")
        .captures(ua)
    {
        let family = "Symbian^3 Anna";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Symbian/3).+NokiaBrowser/7\\.4")
        .captures(ua)
    {
        let family = "Symbian^3 Belle";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Symbian/3)").captures(ua) {
        let family = "Symbian^3";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\b(Series 60|SymbOS|S60Version|S60V\\d|S60\\b)")
        .captures(ua)
    {
        let family = "Symbian OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(MeeGo)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Symbian [Oo][Ss]").captures(ua) {
        let family = "Symbian OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Series40;").captures(ua) {
        let family = "Nokia Series 40";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("Series30Plus;").captures(ua) {
        let family = "Nokia Series 30 Plus";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BB10);.+Version/(\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "BlackBerry OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Black[Bb]erry)[0-9a-z]+/(\\d+)\\.(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
            .captures(ua)
    {
        let family = "BlackBerry OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) =
        crate::regex_cache::cached_regex("(Black[Bb]erry).+Version/(\\d+)\\.(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
            .captures(ua)
    {
        let family = "BlackBerry OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(RIM Tablet OS) (\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "BlackBerry Tablet OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Play[Bb]ook)").captures(ua) {
        let family = "BlackBerry Tablet OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Black[Bb]erry)").captures(ua) {
        let family = "BlackBerry OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/18.0 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "1";
        let minor = "0";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/18.1 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "1";
        let minor = "1";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/26.0 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "1";
        let minor = "2";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/28.0 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "1";
        let minor = "3";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/30.0 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "1";
        let minor = "4";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/32.0 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "2";
        let minor = "0";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Gecko/34.0 Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = "2";
        let minor = "1";
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((?:Mobile|Tablet);.+Firefox/\\d+\\.\\d+")
        .captures(ua)
    {
        let family = "Firefox OS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BREW)[ /](\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(BREW);").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Brew MP|BMP)[ /](\\d+)\\.(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = "Brew MP";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("BMP;").captures(ua) {
        let family = "Brew MP";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(GoogleTV)(?: (\\d+)\\.(\\d+)(?:\\.(\\d+))?|/[\\da-z]+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(WebTV)/(\\d+).(\\d+)").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(CrKey)(?:[/](\\d+)\\.(\\d+)(?:\\.(\\d+))?)?")
        .captures(ua)
    {
        let family = "Chromecast";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(hpw|web)OS/(\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = "webOS";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(VRE);").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Fedora|Red Hat|PCLinuxOS|Puppy|Ubuntu|Kindle|Bada|Lubuntu|BackTrack|Slackware|(?:Free|Open|Net|\\b)BSD)[/ ](\\d+)\\.(\\d+)(?:\\.(\\d+)(?:\\.(\\d+))?)?").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Linux)[ /](\\d+)\\.(\\d+)(?:\\.(\\d+))?.*gentoo")
        .captures(ua)
    {
        let family = "Gentoo";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("\\((Bada);").captures(ua) {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Windows|Android|WeTab|Maemo|Web0S)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(Ubuntu|Kubuntu|Arch Linux|CentOS|Slackware|Gentoo|openSUSE|SUSE|Red Hat|Fedora|PCLinuxOS|Mageia|(?:Free|Open|Net|\\b)BSD)").captures(ua) {
    let family = result.get(0).map_or_else(|| "", Into::<&str>::into);
    let major = result.get(1).map_or_else(|| "0", Into::<&str>::into);
    let minor = result.get(2).map_or_else(|| "0", Into::<&str>::into);
    let patch = result.get(3).map_or_else(|| "0", Into::<&str>::into);
    return [family.to_owned(),major.to_owned(),minor.to_owned(), patch.to_owned()];
}
    if let Some(result) = crate::regex_cache::cached_regex("(Linux)(?:[ /](\\d+)\\.(\\d+)(?:\\.(\\d+))?)?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("SunOS").captures(ua) {
        let family = "Solaris";
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("^(Roku)/DVP-(\\d+)\\.(\\d+)")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);
        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    if let Some(result) = crate::regex_cache::cached_regex("(iOS) (\\d+)\\.(\\d+)(?:\\.(\\d+))?")
        .captures(ua)
    {
        let family = result
            .get(0).map_or_else(|| "", Into::<&str>::into);
        let major = result
            .get(1).map_or_else(|| "0", Into::<&str>::into);
        let minor = result
            .get(2).map_or_else(|| "0", Into::<&str>::into);
        let patch = result
            .get(3).map_or_else(|| "0", Into::<&str>::into);

        return [
            family.to_owned(),
            major.to_owned(),
            minor.to_owned(),
            patch.to_owned(),
        ];
    }
    [String::new(), String::new(), String::new(), String::new()]
}
