struct Series<'lifetime> {

    //values definitions
    _name: str,

    _url: str,

    _episodes: &'lifetime mut Vec<str>
}

impl Series{
//class constructor
    fn new(self, url: str, episodes: Vec<str>) -> Series {

        self._name = episodes[0]
            .trim_left_matches("http://fs.to/")
            .trim_left_matches("http://brb.to/")[66..episodes[0].size]
            .replace("."," ");

        self._url = url;

        self._episodes = episodes;
    }

//methods definitions
    fn Name() -> str { self._name }

    fn Url() -> str { self._url }

    fn EpCount() -> i32 { self._episodes.count }

    fn EpSet(episodes: Vec<str>) { self._episodes = episodes }

    fn EpAdd(episode: str) { self._episodes.push_str(episode) }
}
