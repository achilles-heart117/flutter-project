lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Күй"),
        ("Your Desktop", "Сіздің Жұмыс үстеліңіз"),
        ("desk_tip", "Сіздің Жұмыс үстеліңіз осы ID мен құпия сөз арқылы қолжетімді"),
        ("Password", "Құпия сөз"),
        ("Ready", "Дайын"),
        ("Established", "Қосылды"),
        ("connecting_status", "RustDesk желісіне қосылуда..."),
        ("Enable Service", "Сербесті қосу"),
        ("Start Service", "Сербесті іске қосу"),
        ("Service is running", "Сербес істеуде"),
        ("Service is not running", "Сербес істемеуде"),
        ("not_ready_status", "Дайын емес. Қосылымды тексеруді өтінеміз"),
        ("Control Remote Desktop", "Қашықтағы Жұмыс үстелін Басқару"),
        ("Transfer File", "Файыл Тасымалдау"),
        ("Connect", "Қосылу"),
        ("Recent Sessions", "Соңғы Сештер"),
        ("Address Book", "Мекенжай Кітабы"),
        ("Confirmation", "Мақұлдау"),
        ("TCP Tunneling", "TCP тунелдеу"),
        ("Remove", "Жою"),
        ("Refresh random password", "Кездейсоқ құпия сөзді жаңарту"),
        ("Set your own password", "Өз құпия сөзіңізді орнатыңыз"),
        ("Enable Keyboard/Mouse", "Пернетақта/Тінтуірді қосу"),
        ("Enable Clipboard", "Көшіру-тақтасын қосу"),
        ("Enable File Transfer", "Файыл Тасымалдауды қосу"),
        ("Enable TCP Tunneling", "TCP тунелдеуді қосу"),
        ("IP Whitelisting", "IP Ақ-тізімі"),
        ("ID/Relay Server", "ID/Relay сербері"),
        ("Import Server Config", "Серверді импорттау"),
        ("Export Server Config", ""),
        ("Import server configuration successfully", "Сервердің конфигурациясы сәтті импортталды"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "Жарамсыз сервердің конфигурациясы"),
        ("Clipboard is empty", "Көшіру-тақта бос"),
        ("Stop service", "Сербесті тоқтату"),
        ("Change ID", "ID ауыстыру"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Тек a-z, A-Z, 0-9 және _ (астынғы-сызық) таңбалары рұқсат етілген. Бірінші таңба a-z, A-Z болуы қажет. Ұзындығы 6 мен 16 арасы."),
        ("Website", "Web-сайт"),
        ("About", "Туралы"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Дыбыссыздандыру"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Аудио Еңгізу"),
        ("Enhancements", "Жақсартулар"),
        ("Hardware Codec", "Hardware Codec"),
        ("Adaptive Bitrate", "Adaptive Bitrate"),
        ("ID Server", "ID Сербері"),
        ("Relay Server", "Relay Сербері"),
        ("API Server", "API Сербері"),
        ("invalid_http", "http:// немесе https://'пен басталуы қажет"),
        ("Invalid IP", "Бұрыс IP-Мекенжай"),
        ("Invalid format", "Бұрыс формат"),
        ("server_not_support", "Сербер әзірше қолдамайды"),
        ("Not available", "Қолжетімсіз"),
        ("Too frequent", "Тым жиі"),
        ("Cancel", "Болдырмау"),
        ("Skip", "Өткізіп жіберу"),
        ("Close", "Жабу"),
        ("Retry", "Қайтадан көру"),
        ("OK", "OK"),
        ("Password Required", "Құпия сөз Қажет"),
        ("Please enter your password", "Құпия сөзіңізді еңгізуді өтінеміз"),
        ("Remember password", "Құпия сөзді есте сақтау"),
        ("Wrong Password", "Бұрыс Құпия сөз"),
        ("Do you want to enter again?", "Қайтадан кіргіңіз келеді ме?"),
        ("Connection Error", "Қосылым Қатесі"),
        ("Error", "Қате"),
        ("Reset by the peer", "Пир қалпына келтірді"),
        ("Connecting...", "Қосылуда..."),
        ("Connection in progress. Please wait.", "Қосылым барысында. Күтуді өтінеміз"),
        ("Please try 1 minute later", "1 минуттан соң қайта көріңіз"),
        ("Login Error", "Кіру Қатесі"),
        ("Successful", "Сәтті"),
        ("Connected, waiting for image...", "Қосылды, сурет күтілуде..."),
        ("Name", "Ат"),
        ("Type", "Түр"),
        ("Modified", "Өзгертілді"),
        ("Size", "Өлшем"),
        ("Show Hidden Files", "Жасырын Файылдарды Көрсету"),
        ("Receive", "Қабылдау"),
        ("Send", "Жіберу"),
        ("Refresh File", "Файылды жаңарту"),
        ("Local", "Лақал"),
        ("Remote", "Қашықтағы"),
        ("Remote Computer", "Қашықтағы Қампұтыр"),
        ("Local Computer", "Лақал Қампұтыр"),
        ("Confirm Delete", "Жоюды Растау"),
        ("Delete", "Жою"),
        ("Properties", "Қасиеттер"),
        ("Multi Select", "Көптік таңдау"),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", "Бос Бума"),
        ("Not an empty directory", "Бос бума емес"),
        ("Are you sure you want to delete this file?", "Бұл файылды жоюға сенімдісіз бе?"),
        ("Are you sure you want to delete this empty directory?", "Бұл бос буманы жоюға сенімдісіз бе?"),
        ("Are you sure you want to delete the file of this directory?", "Бұл буманың файылын жоюға сенімдісіз бе?"),
        ("Do this for all conflicts", "Мұны барлық қанпілектер үшін жасау"),
        ("This is irreversible!", "Бұл қайтымсыз!"),
        ("Deleting", "Жойылу"),
        ("files", "файылдар"),
        ("Waiting", "Күту"),
        ("Finished", "Аяқталды"),
        ("Speed", "Жылдамдық"),
        ("Custom Image Quality", "Теңшеулі Сурет Сапасы"),
        ("Privacy mode", "Құпиялылық Модасы"),
        ("Block user input", "Қолданушы еңгізуін бұғаттау"),
        ("Unblock user input", "Қолданушы еңгізуін бұғаттан шығару"),
        ("Adjust Window", "Терезені Реттеу"),
        ("Original", "Түпнұсқа"),
        ("Shrink", "Қысу"),
        ("Stretch", "Созу"),
        ("Scrollbar", "Scrollbar"),
        ("ScrollAuto", "ScrollAuto"),
        ("Good image quality", "Жақсы сурет сапасы"),
        ("Balanced", "Теңдестірілген"),
        ("Optimize reaction time", "Реакция уақытын оңтайландыру"),
        ("Custom", ""),
        ("Show remote cursor", "Қашықтағы курсорды көрсету"),
        ("Show quality monitor", "Сапа мониторын көрсету"),
        ("Disable clipboard", "Көшіру-тақтасын өшіру"),
        ("Lock after session end", "Сеш аяқталған соң құлыптау"),
        ("Insert", "Кірістіру"),
        ("Insert Lock", "Кірістіруді Құлыптау"),
        ("Refresh", "Жаңарту"),
        ("ID does not exist", "ID табылмады"),
        ("Failed to connect to rendezvous server", "Rendezvous серберіне қосылу сәтсіз"),
        ("Please try later", "Кейінірек қайта көруді өтінеміз"),
        ("Remote desktop is offline", "Қашықтағы жұмыс үстелі офлайн күйінде"),
        ("Key mismatch", "Кілт сәйкессіздігі"),
        ("Timeout", "Үзіліс"),
        ("Failed to connect to relay server", "Relay серберіне қосылу сәтсіз"),
        ("Failed to connect via rendezvous server", "Rendezvous сербері арқылы қосылу сәтсіз"),
        ("Failed to connect via relay server", "Relay сербері арқылы қосылу сәтсіз"),
        ("Failed to make direct connection to remote desktop", "Қашықтағы жұмыс үстеліне тікелей қосылым жасау сәтсіз"),
        ("Set Password", "Құпия сөзді Орнату"),
        ("OS Password", "OS Құпия сөзі"),
        ("install_tip", "UAC кесірінен, RustDesk кейбірде қашықтағы жақ ретінде дұрыс жұмыс істей алмайды. UAC'пен қиындықты болдырмау үшін, төмендегі батырманы басып RustDesk'ті жүйеге орнатыңыз."),
        ("Click to upgrade", "Жаңғырту үшін басыңыз"),
        ("Click to download", "Жүктеу үшін басыңыз"),
        ("Click to update", "Жаңарту үшін басыңыз"),
        ("Configure", "Қалыптау"),
        ("config_acc", "Сіздің Жұмыс үстеліңізді қашықтан басқару үшін, RustDesk'ке \"Қолжетімділік\" рұқсаттарын беруіңіз керек."),
        ("config_screen", "Сіздің Жұмыс үстеліңізге қашықтан қол жеткізу үшін, RustDesk'ке \"Екіренді Жазу\" рұқсаттарын беруіңіз керек."),
        ("Installing ...", "Орнатылу..."),
        ("Install", "Орнату"),
        ("Installation", "Орнатылу"),
        ("Installation Path", "Орнатылу Жолы"),
        ("Create start menu shortcuts", "Бастау мәзірі белгішесің жасау"),
        ("Create desktop icon", "Жұмыс үстелі белгішесің жасау"),
        ("agreement_tip", "Орнатуды бастасаңыз, сіз лисензе келісімін қабылдайсыз."),
        ("Accept and Install", "Қабылдау және Орнату"),
        ("End-user license agreement", "Түпкі қолданушының лисензе келісімі"),
        ("Generating ...", "Генератталуда..."),
        ("Your installation is lower version.", "Сіздің орнатуыныз төменгі нұсқа."),
        ("not_close_tcp_tip", "Тунел қолдану кезінде бұл терезені жаппаңыз"),
        ("Listening ...", "Тыңдау ..."),
        ("Remote Host", "Қашықтағы Хост"),
        ("Remote Port", "Қашықтағы Порт"),
        ("Action", "Әрекет"),
        ("Add", "Қосу"),
        ("Local Port", "Лақал Порт"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "Тез қосылым үшін өз серберіңізді орнатуды өтінеміз"),
        ("Too short, at least 6 characters.", "Тым қысқа, кемінде 6 таңба."),
        ("The confirmation is not identical.", "Растау сәйкес келмейді."),
        ("Permissions", "Рұқсаттар"),
        ("Accept", "Қабылдау"),
        ("Dismiss", "Босату"),
        ("Disconnect", "Ажырату"),
        ("Allow using keyboard and mouse", "Пернетақта мен тінтуірді қолдануды рұқсат ету"),
        ("Allow using clipboard", "Көшіру-тақтасын рұқсат ету"),
        ("Allow hearing sound", "Дыбыс естуді рұқсат ету"),
        ("Allow file copy and paste", "Файылды көшіру мен қоюды рұқсат ету"),
        ("Connected", "Қосылды"),
        ("Direct and encrypted connection", "Тікелей және кіриптелген қосылым"),
        ("Relayed and encrypted connection", "Релайданған және кіриптелген қосылым"),
        ("Direct and unencrypted connection", "Тікелей және кіриптелмеген қосылым"),
        ("Relayed and unencrypted connection", "Релайданған және кіриптелмеген қосылым"),
        ("Enter Remote ID", "Қашықтағы ID еңгізіңіз"),
        ("Enter your password", "Құпия сөзіңізді енгізіңіз"),
        ("Logging in...", "Кіруде..."),
        ("Enable RDP session sharing", "RDP сешті бөлісуді іске қосу"),
        ("Auto Login", "Ауты Кіру (\"Сеш аяқталған соң құлыптау\"'ды орнатқанда ғана жарамды)"),
        ("Enable Direct IP Access", "Тікелей IP Қолжетімді іске қосу"),
        ("Rename", "Атын өзгерту"),
        ("Space", "Орын"),
        ("Create Desktop Shortcut", "Жұмыс үстелі Таңбашасын Жасау"),
        ("Change Path", "Жолды өзгерту"),
        ("Create Folder", "Бума жасау"),
        ("Please enter the folder name", "Буманың атауын еңгізуді өтінеміз"),
        ("Fix it", "Түзету"),
        ("Warning", "Ескерту"),
        ("Login screen using Wayland is not supported", "Wayland қолданған Кіру екіреніне қолдау көрсетілмейді"),
        ("Reboot required", "Қайта-қосу қажет"),
        ("Unsupported display server", "Қолдаусыз дисплей сербері"),
        ("x11 expected", "x11 күтілген"),
        ("Port", "Порт"),
        ("Settings", "Орнатпалар"),
        ("Username", "Қолданушы аты"),
        ("Invalid port", "Бұрыс порт"),
        ("Closed manually by the peer", "Пир қолымен жабылған"),
        ("Enable remote configuration modification", "Қашықтан қалыптарды өзгертуді іске қосу"),
        ("Run without install", "Орнатпай-ақ Іске қосу"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Әрқашан да релай сербері арқылы қосылу"),
        ("whitelist_tip", "Маған тек ақ-тізімделген IP қол жеткізе алады"),
        ("Login", "Кіру"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Шығу"),
        ("Tags", "Тақтар"),
        ("Search ID", "ID Іздеу"),
        ("whitelist_sep", "Үтір, нүктелі үтір, бос орын және жаңа жолал арқылы бөлінеді"),
        ("Add ID", "ID Қосу"),
        ("Add Tag", "Тақ Қосу"),
        ("Unselect all tags", "Барлық тақтардың таңдауын алып тастау"),
        ("Network error", "Желі қатесі"),
        ("Username missed", "Қолданушы аты бос"),
        ("Password missed", "Құпия сөз бос"),
        ("Wrong credentials", "Бұрыс тіркелгі деректер"),
        ("Edit Tag", "Тақты Өндеу"),
        ("Unremember Password", "Құпия сөзді Ұмыту"),
        ("Favorites", "Таңдаулылар"),
        ("Add to Favorites", "Таңдаулыларға Қосу"),
        ("Remove from Favorites", "Таңдаулылардан алып тастау"),
        ("Empty", "Бос"),
        ("Invalid folder name", "Бұрыс бума атауы"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Хост атауы"),
        ("Discovered", "Табылды"),
        ("install_daemon_tip", "Бут кезінде қосылу үшін жүйелік сербесті орнатуыныз керек."),
        ("Remote ID", "Қашықтағы ID"),
        ("Paste", "Қою"),
        ("Paste here?", "Осында қою керек пе?"),
        ("Are you sure to close the connection?", "Қосылымды жабуға сенімдісіз бе?"),
        ("Download new version", "Жаңа нұсқаны жүктеу"),
        ("Touch mode", "Жанасатын мода"),
        ("Mouse mode", "Тінтуірлі мода"),
        ("One-Finger Tap", "Бір-Саусақпен Түрту"),
        ("Left Mouse", "Солақ Тінтуір"),
        ("One-Long Tap", "Бір-Ұзақ Түрту"),
        ("Two-Finger Tap", "Екі-Саусақпен Түрту"),
        ("Right Mouse", "Оңақ Тінтуір"),
        ("One-Finger Move", "Бір-Саусақпен Жылжыту"),
        ("Double Tap & Move", "Екі-рет Түртіп Жылжыту"),
        ("Mouse Drag", "Тінтуір Тартуы"),
        ("Three-Finger vertically", "Үш-Саусақпен тік-бағытты"),
        ("Mouse Wheel", "Тінтуір Дөңгелегі"),
        ("Two-Finger Move", "Екі-Саусақпен Жылжыту"),
        ("Canvas Move", "Кенеп Жылжуы"),
        ("Pinch to Zoom", "Зумдау үшін Шымшыңыз"),
        ("Canvas Zoom", "Кенеп Зумы"),
        ("Reset canvas", "Кенепті қалпына келтіру"),
        ("No permission of file transfer", "Файыл алмасуға рұқсат берілмеген"),
        ("Note", "Нота"),
        ("Connection", "Қосылым"),
        ("Share Screen", "Екіренді Бөлісу"),
        ("Chat", "Чат"),
        ("Total", "Барлығы"),
        ("items", "зат"),
        ("Selected", "Таңдалған"),
        ("Screen Capture", "Екіренді Түсіру"),
        ("Input Control", "Еңгізуді Басқару/Қадағалау"),
        ("Audio Capture", "Аудио Түсіру"),
        ("File Connection", "Файыл Қосылымы"),
        ("Screen Connection", "Екірен Қосылымы"),
        ("Do you accept?", "Қабылдайсыз ба?"),
        ("Open System Setting", "Жүйе Орнатпаларын Ашу"),
        ("How to get Android input permission?", "Android еңгізу рұқсатын қалай алуға болады?"),
        ("android_input_permission_tip1", "Қашықтағы құрылғы сіздің Android құрылғыңызды тінтуір немесе түрту арқылы басқару үшін, RustDesk'ке \"Қолжетімділік\" сербесін қолдануға рұқсат беруініз керек."),
        ("android_input_permission_tip2", "Келесі Жүйе Орнатпалары бетіне барып, [Орнатылған Сербестер]'ді тауып кіріңіз, сосын [RustDesk Еңгізу] сербесін іске қосыңыз."),
        ("android_new_connection_tip", "Сіздің ағымдағы құрылғыңызды басқаруды қалайтын жаңа басқару сұранысы түсті."),
        ("android_service_will_start_tip", "\"Екіренді Тұсіру\" қосылған кезде сербес аутыматты іске қосылып, басқа құрылғыларға сіздің құрылғыға қосылым сұраныстауға мүмкіндің береді."),
        ("android_stop_service_tip", "Сербесті жабу аутыматты түрде барлық орнатылған қосылымдарды жабады."),
        ("android_version_audio_tip", "Ағымдағы Android нұсқасы аудионы түсіруді қолдамайды, Android 10 не жоғарғысына жаңғыртуды өтінеміз."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Есепкі"),
        ("Overwrite", "Үстінен қайта жазу"),
        ("This file exists, skip or overwrite this file?", "Бұл файыл бар, өткізіп жіберу әлде үстінен қайта жазу керек пе?"),
        ("Quit", "Шығу"),
        ("doc_mac_permission", ""),
        ("Help", "Көмек"),
        ("Failed", "Сәтсіз"),
        ("Succeeded", "Сәтті"),
        ("Someone turns on privacy mode, exit", "Біреу құпиялылық модасын қосты, шығу"),
        ("Unsupported", "Қолдаусыз"),
        ("Peer denied", "Пир қабылдамады"),
        ("Please install plugins", "Плагиндерді орнатуды өтінеміз"),
        ("Peer exit", "Пирдің шығуы"),
        ("Failed to turn off", "Сөндіру сәтсіз болды"),
        ("Turned off", "Өшірілген"),
        ("In privacy mode", "Құпиялылық модасында"),
        ("Out privacy mode", "Құпиялылық модасынан Шығу"),
        ("Language", "Тіл"),
        ("Keep RustDesk background service", "Артжақтағы RustDesk сербесін сақтап тұру"),
        ("Ignore Battery Optimizations", "Бәтері Оңтайландыруларын Елемеу"),
        ("android_open_battery_optimizations_tip", "Егер де бұл ерекшелікті өшіруді қаласаңыз, келесі RustDesk апылқат орнатпалары бетіне барып, [Бәтері]'ні тауып кіріңіз де [Шектеусіз]'ден құсбелгіні алып тастауды өтінеміз"),
        ("Start on Boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Қосылу рұқсат етілмеген"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", "Тұрақты құпия сөзді қолдану"),
        ("Use both passwords", "Қос құпия сөзді қолдану"),
        ("Set permanent password", "Тұрақты құпия сөзді орнату"),
        ("Enable Remote Restart", "Қашықтан қайта-қосуды іске қосу"),
        ("Allow remote restart", "Қашықтан қайта-қосуды рұқсат ету"),
        ("Restart Remote Device", "Қашықтағы құрылғыны қайта-қосу"),
        ("Are you sure you want to restart", "Қайта-қосуға сенімдісіз бе?"),
        ("Restarting Remote Device", "Қашықтағы Құрылғыны қайта-қосуда"),
        ("remote_restarting_tip", "Қашықтағы құрылғы қайта-қосылуда, бұл хабар терезесін жабып, біраздан соң тұрақты құпия сөзбен қайта қосылуды өтінеміз"),
        ("Copied", "Көшірілді"),
        ("Exit Fullscreen", "Толық екіреннен Шығу"),
        ("Fullscreen", "Толық екірен"),
        ("Mobile Actions", "Мабыл Әрекеттері"),
        ("Select Monitor", "Мониторды Таңдау"),
        ("Control Actions", "Басқару Әрекеттері"),
        ("Display Settings", "Дисплей Орнатпалары"),
        ("Ratio", "Арақатынас"),
        ("Image Quality", "Сурет Сапасы"),
        ("Scroll Style", "Scroll Теңшетұрі"),
        ("Show Menubar", "Мәзір жолағын көрсету"),
        ("Hide Menubar", "Мәзір жолағын жасыру"),
        ("Direct Connection", "Тікелей Қосылым"),
        ("Relay Connection", "Релай Қосылым"),
        ("Secure Connection", "Қауіпсіз Қосылым"),
        ("Insecure Connection", "Қатерлі Қосылым"),
        ("Scale original", "Scale original"),
        ("Scale adaptive", "Scale adaptive"),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Light Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable Audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Deny remote access", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Enable RDP", ""),
        ("Pin menubar", "Мәзір жолағын бекіту"),
        ("Unpin menubar", "Мәзір жолағын босату"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", ""),
        ("Allow recording session", ""),
        ("Enable LAN Discovery", ""),
        ("Deny LAN Discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland Ubuntu 21.04 немесе одан жоғары нұсқасын қажет етеді."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland linux дистрибутивінің жоғарырақ нұсқасын қажет етеді. X11 жұмыс үстелін қолданып көріңіз немесе операциялық жүйеңізді өзгертіңіз."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Бөлісетін экранды таңдаңыз (бірдей жағынан жұмыс жасаңыз)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("idd_driver_tip", ""),
        ("confirm_idd_driver_tip", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
    ].iter().cloned().collect();
}
