// Generated by the build script. Do not edit directly!

pub static CATEGORIES: ::phf::Map<&'static str, Category> = ::phf::Map {
    key: 9603444721912725599,
    disps: ::phf::Slice::Static(&[
        (0, 55),
        (0, 0),
        (0, 15),
        (0, 77),
        (0, 23),
        (0, 2),
        (0, 0),
        (25, 112),
        (0, 36),
        (0, 32),
        (2, 11),
        (4, 89),
        (0, 90),
        (2, 0),
        (0, 61),
        (0, 118),
        (0, 0),
        (2, 86),
        (4, 86),
        (9, 127),
        (2, 0),
        (0, 2),
        (0, 70),
        (0, 1),
        (4, 128),
        (5, 132),
        (0, 0),
        (20, 93),
        (29, 42),
    ]),
    entries: ::phf::Slice::Static(&[
        ("KDE", Category::Additional { suggests: &[] }),
        ("FileManager", Category::Additional { suggests: &[] }),
        ("HardwareSettings", Category::Additional { suggests: &[] }),
        ("Geography", Category::Additional { suggests: &["Education", "Science"] }),
        ("Chart", Category::Additional { suggests: &[] }),
        ("Game", Category::Main { requires: &[] }),
        ("Accessibility", Category::Additional { suggests: &["Settings", "Utility"] }),
        ("ImageProcessing", Category::Additional { suggests: &["Education", "Science"] }),
        ("News", Category::Additional { suggests: &[] }),
        ("KidsGame", Category::Additional { suggests: &[] }),
        ("PackageManager", Category::Additional { suggests: &[] }),
        ("Music", Category::Additional { suggests: &["AudioVideo", "Education"] }),
        ("Qt", Category::Additional { suggests: &[] }),
        ("Recorder", Category::Additional { suggests: &["Audio", "Video", "AudioVideo"] }),
        ("DataVisualization", Category::Additional { suggests: &["Education", "Science"] }),
        ("Settings", Category::Main { requires: &[] }),
        ("Development", Category::Main { requires: &[] }),
        ("Math", Category::Additional { suggests: &["Education", "Science"] }),
        ("DiscBurning", Category::Additional { suggests: &[] }),
        ("AdventureGame", Category::Additional { suggests: &[] }),
        ("ArcadeGame", Category::Additional { suggests: &[] }),
        ("InstantMessaging", Category::Additional { suggests: &[] }),
        ("StrategyGame", Category::Additional { suggests: &[] }),
        ("ArtificialIntelligence", Category::Additional { suggests: &["Education", "Science"] }),
        ("Midi", Category::Additional { suggests: &[] }),
        ("Electricity", Category::Additional { suggests: &["Education", "Science"] }),
        ("NumericalAnalysis", Category::Additional { suggests: &["Education;Math", "Science;Math"] }),
        ("2DGraphics", Category::Additional { suggests: &[] }),
        ("Science", Category::Main { requires: &[] }),
        ("Database", Category::Additional { suggests: &["Office", "Development", "AudioVideo"] }),
        ("Economy", Category::Additional { suggests: &["Education", "Science"] }),
        ("Photography", Category::Additional { suggests: &["Graphics", "Office"] }),
        ("Office", Category::Main { requires: &[] }),
        ("Education", Category::Main { requires: &[] }),
        ("Chat", Category::Additional { suggests: &[] }),
        ("Shooter", Category::Additional { suggests: &[] }),
        ("Dialup", Category::Additional { suggests: &[] }),
        ("Amusement", Category::Additional { suggests: &[] }),
        ("AudioVideoEditing", Category::Additional { suggests: &["Audio", "Video", "AudioVideo"] }),
        ("Literature", Category::Additional { suggests: &["Education", "Science"] }),
        ("Network", Category::Main { requires: &[] }),
        ("Robotics", Category::Additional { suggests: &["Education", "Science"] }),
        ("Security", Category::Additional { suggests: &["Settings", "System"] }),
        ("Biology", Category::Additional { suggests: &["Education", "Science"] }),
        ("Archiving", Category::Additional { suggests: &[] }),
        ("Mixer", Category::Additional { suggests: &[] }),
        ("Shell", Category::Reserved),
        ("ProjectManagement", Category::Additional { suggests: &["Office", "Development"] }),
        ("WordProcessor", Category::Additional { suggests: &[] }),
        ("TextTools", Category::Additional { suggests: &[] }),
        ("CardGame", Category::Additional { suggests: &[] }),
        ("GTK", Category::Additional { suggests: &[] }),
        ("Finance", Category::Additional { suggests: &[] }),
        ("XFCE", Category::Additional { suggests: &[] }),
        ("Emulator", Category::Additional { suggests: &["System", "Game"] }),
        ("WebDevelopment", Category::Additional { suggests: &["Network", "Development"] }),
        ("AudioVideo", Category::Main { requires: &[] }),
        ("TextEditor", Category::Additional { suggests: &[] }),
        ("TV", Category::Additional { suggests: &[] }),
        ("3DGraphics", Category::Additional { suggests: &[] }),
        ("TerminalEmulator", Category::Additional { suggests: &[] }),
        ("Languages", Category::Additional { suggests: &["Education", "Science"] }),
        ("RasterGraphics", Category::Additional { suggests: &[] }),
        ("Monitor", Category::Additional { suggests: &["System", "Network"] }),
        ("TrayIcon", Category::Reserved),
        ("System", Category::Main { requires: &[] }),
        ("Presentation", Category::Additional { suggests: &[] }),
        ("Applet", Category::Reserved),
        ("RevisionControl", Category::Additional { suggests: &[] }),
        ("History", Category::Additional { suggests: &["Education", "Science"] }),
        ("RolePlaying", Category::Additional { suggests: &[] }),
        ("Email", Category::Additional { suggests: &["Office", "Network"] }),
        ("SportsGame", Category::Additional { suggests: &[] }),
        ("Motif", Category::Additional { suggests: &[] }),
        ("WebBrowser", Category::Additional { suggests: &[] }),
        ("VectorGraphics", Category::Additional { suggests: &[] }),
        ("Publishing", Category::Additional { suggests: &["Graphics", "Office"] }),
        ("Scanning", Category::Additional { suggests: &[] }),
        ("Maps", Category::Additional { suggests: &["Education", "Science", "Utility"] }),
        ("Documentation", Category::Additional { suggests: &[] }),
        ("TelephonyTools", Category::Additional { suggests: &[] }),
        ("Graphics", Category::Main { requires: &[] }),
        ("Video", Category::Main { requires: &["AudioVideo"] }),
        ("VideoConference", Category::Additional { suggests: &[] }),
        ("Tuner", Category::Additional { suggests: &[] }),
        ("Translation", Category::Additional { suggests: &[] }),
        ("Electronics", Category::Additional { suggests: &[] }),
        ("ComputerScience", Category::Additional { suggests: &["Education", "Science"] }),
        ("Utility", Category::Main { requires: &[] }),
        ("Printing", Category::Additional { suggests: &[] }),
        ("Audio", Category::Main { requires: &["AudioVideo"] }),
        ("FlowChart", Category::Additional { suggests: &[] }),
        ("Art", Category::Additional { suggests: &["Education", "Science"] }),
        ("Dictionary", Category::Additional { suggests: &["Office", "TextTools"] }),
        ("Debugger", Category::Additional { suggests: &[] }),
        ("GUIDesigner", Category::Additional { suggests: &[] }),
        ("Spirituality", Category::Additional { suggests: &["Education", "Science", "Utility"] }),
        ("LogicGame", Category::Additional { suggests: &[] }),
        ("ParallelComputing", Category::Additional { suggests: &["Education;ComputerScience", "Science;ComputerScience"] }),
        ("Calendar", Category::Additional { suggests: &[] }),
        ("HamRadio", Category::Additional { suggests: &["Network", "Audio"] }),
        ("Java", Category::Additional { suggests: &[] }),
        ("GNOME", Category::Additional { suggests: &[] }),
        ("BlocksGame", Category::Additional { suggests: &[] }),
        ("Physics", Category::Additional { suggests: &["Education", "Science"] }),
        ("Simulation", Category::Additional { suggests: &[] }),
        ("Construction", Category::Additional { suggests: &[] }),
        ("Sequencer", Category::Additional { suggests: &[] }),
        ("Clock", Category::Additional { suggests: &[] }),
        ("Player", Category::Additional { suggests: &["Audio", "Video", "AudioVideo"] }),
        ("PDA", Category::Additional { suggests: &[] }),
        ("ConsoleOnly", Category::Additional { suggests: &[] }),
        ("Engineering", Category::Additional { suggests: &[] }),
        ("DesktopSettings", Category::Additional { suggests: &[] }),
        ("ActionGame", Category::Additional { suggests: &[] }),
        ("ContactManagement", Category::Additional { suggests: &[] }),
        ("MedicalSoftware", Category::Additional { suggests: &["Education", "Science"] }),
        ("IRCClient", Category::Additional { suggests: &[] }),
        ("RemoteAccess", Category::Additional { suggests: &[] }),
        ("Sports", Category::Additional { suggests: &["Education", "Science"] }),
        ("Astronomy", Category::Additional { suggests: &["Education", "Science"] }),
        ("Profiling", Category::Additional { suggests: &[] }),
        ("Filesystem", Category::Additional { suggests: &[] }),
        ("P2P", Category::Additional { suggests: &[] }),
        ("Viewer", Category::Additional { suggests: &["Graphics", "Office"] }),
        ("Geoscience", Category::Additional { suggests: &["Education", "Science"] }),
        ("Chemistry", Category::Additional { suggests: &["Education", "Science"] }),
        ("Building", Category::Additional { suggests: &[] }),
        ("Core", Category::Additional { suggests: &[] }),
        ("IDE", Category::Additional { suggests: &[] }),
        ("Telephony", Category::Additional { suggests: &[] }),
        ("FileTransfer", Category::Additional { suggests: &[] }),
        ("BoardGame", Category::Additional { suggests: &[] }),
        ("FileTools", Category::Additional { suggests: &["Utility", "System"] }),
        ("Geology", Category::Additional { suggests: &["Education", "Science"] }),
        ("Humanities", Category::Additional { suggests: &["Education", "Science"] }),
        ("Calculator", Category::Additional { suggests: &[] }),
        ("Compression", Category::Additional { suggests: &[] }),
        ("Spreadsheet", Category::Additional { suggests: &[] }),
        ("Adult", Category::Additional { suggests: &[] }),
        ("Screensaver", Category::Reserved),
        ("Feed", Category::Additional { suggests: &[] }),
        ("OCR", Category::Additional { suggests: &[] }),
    ]),
};
