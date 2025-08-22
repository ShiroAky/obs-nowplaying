fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("FileDescription", "OBS NowPlaying Server");
    res.set("ProductName", "OBS NowPlaying");
    res.set("CompanyName", "ShiroAky");
    res.set("LegalCopyright", "© 2025 ShiroAky");
    res.set("OriginalFilename", "obs-nowplaying.exe");
    res.compile().unwrap();
}