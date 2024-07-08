// time_labels.rs
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::fmt;
use wasm_bindgen::prelude::*;

// Define Language Enum
#[derive(Default, Eq, PartialEq, Hash)] // Necessary for using as HashMap key
#[wasm_bindgen]
pub enum Language {
    CHN, SPN, #[default] ENG, BNG, HND, POR, RUS, JPN, GER, WUU,
    JAN, KKN, FRN, VIE, TCW, YUH, MRT, TCV, TRK, URD,
    CFR, CJY, GJR, PQL, ARZ, UKR, ITN, HSN, MJS, HAK,
    KJV, ORY, PNB, SUO, PNJ, RUM, BHJ, AZB, PES, MKP,
    HUA, ARQ, BMS, SRC, KNN, AWD, THJ, DUT, YOR, SND,
    ARY, AEC, UZB, MLI, AMH, INZ, IGR, TGL, NEP, APD,
    SKR, CEB, APC, TTS, ASM, HNG, CIT, ACM, MHJ, SNH,
    BGC, MKD, CZC, GRK, MQM, HNE, DCC, MNP, RUW, CCX,
    ARS, PBU, SOM, MEX, AEB, RUA, ZUU, BLG, SWD, LMO,
    GAZ, PBT, KAZ, ILO, TTR, FUV, AYN, UIG, HAT, AZE,
    NPL, KMR, PRS, TWS, HIL, KUR, SHD,
}

// Implement Display for Language
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::CHN =>  "Chinese",
                Language::SPN =>  "Spanish",
                Language::ENG =>  "English",
                Language::BNG =>  "Bengali",
                Language::HND =>  "Hindi",
                Language::POR =>  "Portuguese",
                Language::RUS =>  "Russian",
                Language::JPN =>  "Japanese",
                Language::GER =>  "German",
                Language::WUU =>  "Wu Chinese", 
                Language::JAN =>  "Javanese",
                Language::KKN =>  "Kokni",
                Language::FRN =>  "French",
                Language::VIE =>  "Vietnamese",
                Language::TCW =>  "Tagalog",
                Language::YUH =>  "Chinese, Yue",
                Language::MRT =>  "Marathi",
                Language::TCV =>  "Caviteño",
                Language::TRK =>  "Turkish",
                Language::URD =>  "Urdu",
                Language::CFR =>  "Crimean Tatar",
                Language::CJY =>  "Jinyu Chinese",
                Language::GJR =>  "Gujarati",
                Language::PQL =>  "Pomeranian",
                Language::ARZ =>  "Egyptian Arabic",
                Language::UKR =>  "Ukrainian",
                Language::ITN =>  "Italian",
                Language::HSN =>  "Xiang Chinese",
                Language::MJS =>  "Malaccan Creole Malay",
                Language::HAK =>  "Hakka Chinese",
                Language::KJV =>  "Western Panjabi",
                Language::ORY =>  "Odia",
                Language::PNB =>  "Western Panjabi",
                Language::SUO =>  "Finnish",
                Language::PNJ =>  "Western Panjabi",
                Language::RUM =>  "Romanian",
                Language::BHJ =>  "Bhojpuri",
                Language::AZB =>  "South Azerbaijani",
                Language::PES =>  "Western Farsi",
                Language::MKP =>  "Mohawk",
                Language::HUA =>  "Mandarin Chinese",
                Language::ARQ =>  "Algerian Arabic",
                Language::BMS =>  "Javanese",
                Language::SRC =>  "Serbian",
                Language::KNN =>  "Gan Chinese", 
                Language::AWD =>  "Awadhi",
                Language::THJ =>  "Tai",
                Language::DUT =>  "Dutch; Flemish",
                Language::YOR =>  "Yoruba",
                Language::SND =>  "Sindhi",
                Language::ARY =>  "Moroccan Arabic",
                Language::AEC =>  "Saidi Arabic",
                Language::UZB =>  "Uzbek",
                Language::MLI =>  "Malayalam",
                Language::AMH =>  "Amharic",
                Language::INZ =>  "Indonesian",
                Language::IGR =>  "Igbo",
                Language::TGL =>  "Tagalog",
                Language::NEP =>  "Nepali",
                Language::APD =>  "Samar-Leyte Visayan",
                Language::SKR =>  "Saraiki",
                Language::CEB =>  "Cebuano",
                Language::APC =>  "North Levantine Arabic",
                Language::TTS =>  "Tausug",
                Language::ASM =>  "Assamese",
                Language::HNG =>  "Hungarian",
                Language::CIT =>  "Citak",
                Language::ACM =>  "Iraqi Arabic",
                Language::MHJ =>  "Mahajani",
                Language::SNH =>  "Sinhala; Sinhalese",
                Language::BGC =>  "Haryanvi",
                Language::MKD =>  "Macedonian",
                Language::CZC =>  "Czech",
                Language::GRK =>  "Greek",
                Language::MQM =>  "Macaense",
                Language::HNE =>  "Chhattisgarhi",
                Language::DCC =>  "Dakhini",
                Language::MNP =>  "Manipuri",
                Language::RUW =>  "Kinyarwanda",
                Language::CCX =>  "Northern Luri",
                Language::ARS =>  "Najdi Arabic",
                Language::PBU =>  "Northern Pashto",
                Language::SOM =>  "Somali",
                Language::MEX =>  "Mexican Spanish",
                Language::AEB =>  "Tunisian Arabic",
                Language::RUA =>  "Kinyarwanda",
                Language::ZUU =>  "Zulu",
                Language::BLG =>  "Bulgarian",
                Language::SWD =>  "Swedish",
                Language::LMO =>  "Lombard",
                Language::GAZ =>  "West Central Oromo",
                Language::PBT =>  "Western Farsi",
                Language::KAZ =>  "Kazakh",
                Language::ILO =>  "Iloko",
                Language::TTR =>  "Tatar",
                Language::FUV =>  "Nigerian Fulfulde",
                Language::AYN =>  "Northern Altai",
                Language::UIG =>  "Uighur; Uyghur",
                Language::HAT =>  "Haitian; Haitian Creole",
                Language::AZE =>  "Azerbaijani",
                Language::NPL =>  "Nepali",
                Language::KMR =>  "Kurmanji Kurdish",
                Language::PRS =>  "Dari",
                Language::TWS =>  "Twi",
                Language::HIL =>  "Hiligaynon",
                Language::KUR =>  "Central Kurdish",
                Language::SHD =>  "Kundal Shahi",
            }
        )
    }
}

pub struct TimeLabelData {
    intervals:  Vec<(&'static str, &'static str)>,
    ago_label: &'static str,
    just_now_label: &'static str,
}

impl TimeLabelData {
    pub fn intervals(&self) -> &Vec<(&'static str, &'static str)> {
        &self.intervals
    }

    pub fn ago_label(&self) -> &str {
        &self.ago_label
    }

    pub fn just_now_label(&self) -> &str {
        &self.just_now_label
    }
}


lazy_static! {
    pub static ref TIME_LABELS: HashMap<Language, TimeLabelData> = { 
        let mut map = HashMap::new();
        map.insert(Language::CHN, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("天", "天"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::SPN, TimeLabelData { intervals: vec![("año", "años"), ("mes", "meses"), ("día", "días"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")], ago_label: "hace", just_now_label: "ahora mismo" });
        map.insert(Language::ENG, TimeLabelData { intervals: vec![("year", "years"), ("month", "months"), ("day", "days"), ("hour", "hours"), ("minute", "minutes"), ("second", "seconds")], ago_label: "ago", just_now_label: "just now" });
        map.insert(Language::BNG, TimeLabelData { intervals: vec![("বছর", "বছর"), ("মাস", "মাস"), ("দিন", "দিন"), ("ঘন্টা", "ঘন্টা"), ("মিনিট", "মিনিট"), ("সেকেন্ড", "সেকেন্ড")], ago_label: "আগে", just_now_label: "এখনই" });
        map.insert(Language::HND, TimeLabelData { intervals: vec![("वर्ष", "वर्ष"), ("महीना", "महीने"), ("दिन", "दिन"), ("घंटा", "घंटे"), ("मिनट", "मिनट"), ("सेकंड", "सेकंड")], ago_label: "पहले", just_now_label: "अभी अभी" });
        map.insert(Language::POR, TimeLabelData { intervals: vec![("ano", "anos"), ("mês", "meses"), ("dia", "dias"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")], ago_label: "atrás", just_now_label: "agora mesmo" });
        map.insert(Language::RUS, TimeLabelData { intervals: vec![("год", "года"), ("месяц", "месяцы"), ("день", "дни"), ("час", "часа"), ("минута", "минуты"), ("секунда", "секунды")], ago_label: "назад", just_now_label: "только что" });
        map.insert(Language::JPN, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("時間", "時間"), ("分", "分"), ("秒", "秒")], ago_label: "前", just_now_label: "たった今" });
        map.insert(Language::GER, TimeLabelData { intervals: vec![("Jahr", "Jahre"), ("Monat", "Monate"), ("Tag", "Tage"), ("Stunde", "Stunden"), ("Minute", "Minuten"), ("Sekunde", "Sekunden")], ago_label: "vor", just_now_label: "gerade eben" });
        map.insert(Language::WUU, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::JAN, TimeLabelData { intervals: vec![("taun", "taun"), ("wulan", "wulan"), ("dina", "dina"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")], ago_label: "kepungkur", just_now_label: "saiki" });
        map.insert(Language::KKN, TimeLabelData { intervals: vec![("jiiq", "jiiq"), ("wer", "wer"), ("uruq", "uruq"), ("sa’a", "sa’aat"), ("daqiiqa", "daqaa’iq"), ("thaaniya", "thawaani")], ago_label: "kadim", just_now_label: "attaan" });
        map.insert(Language::FRN, TimeLabelData { intervals: vec![("an", "ans"), ("mois", "mois"), ("jour", "jours"), ("heure", "heures"), ("minute", "minutes"), ("seconde", "secondes")], ago_label: "il y a", just_now_label: "à l'instant" });
        map.insert(Language::VIE, TimeLabelData { intervals: vec![("năm", "năm"), ("tháng", "tháng"), ("ngày", "ngày"), ("giờ", "giờ"), ("phút", "phút"), ("giây", "giây")], ago_label: "trước", just_now_label: "vừa mới" });
        map.insert(Language::TCW, TimeLabelData { intervals: vec![("hidaw", "hidaw"), ("buwan", "buwan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::YUH, TimeLabelData { intervals: vec![("nián", "nián"), ("yuè", "yuè"), ("tiān", "tiān"), ("xiǎoshí", "xiǎoshí"), ("fēnzhōng", "fēnzhōng"), ("miǎo", "miǎo")], ago_label: "之前", just_now_label: "刚刚" });
        map.insert(Language::MRT, TimeLabelData { intervals: vec![("वर्ष", "वर्ष"), ("महिना", "महिने"), ("दिवस", "दिवस"), ("तास", "तास"), ("मिनिट", "मिनिटे"), ("सेकंद", "सेकंद")], ago_label: "पूर्वी", just_now_label: "आत्ताच" });
        map.insert(Language::TCV, TimeLabelData { intervals: vec![("kawak", "kawak"), ("bulan", "bulan"), ("aldow", "aldow"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::TRK, TimeLabelData { intervals: vec![("yıl", "yıl"), ("ay", "ay"), ("gün", "gün"), ("saat", "saat"), ("dakika", "dakika"), ("saniye", "saniye")], ago_label: "önce", just_now_label: "az önce" });
        map.insert(Language::URD, TimeLabelData { intervals: vec![("سال", "سال"), ("مہینہ", "مہینے"), ("دن", "دن"), ("گھنٹہ", "گھنٹے"), ("منٹ", "منٹ"), ("سیکنڈ", "سیکنڈ")], ago_label: "پہلے", just_now_label: "ابھی ابھی" });
        map.insert(Language::CFR, TimeLabelData { intervals: vec![("an", "ani"), ("lună", "luni"), ("zi", "zile"), ("oră", "ore"), ("minut", "minute"), ("secundă", "secunde")], ago_label: "în urmă", just_now_label: "chiar acum" });
        map.insert(Language::CJY, TimeLabelData { intervals: vec![("rok", "lata"), ("miesiąc", "miesiące"), ("dzień", "dni"), ("godzina", "godziny"), ("minuta", "minuty"), ("sekunda", "sekundy")], ago_label: "temu", just_now_label: "przed chwilą" });
        map.insert(Language::GJR, TimeLabelData { intervals: vec![("વર્ષ", "વર્ષ"), ("મહિનો", "મહિના"), ("દિવસ", "દિવસો"), ("કલાક", "કલાકો"), ("મિનિટ", "મિનિટ"), ("સેકન્ડ", "સેકન્ડ")], ago_label: "પહેલાં", just_now_label: "હમણાં" });
        map.insert(Language::PQL, TimeLabelData { intervals: vec![("rok", "roky"), ("miesąc", "miesiące"), ("dzień", "dni"), ("godzina", "godziny"), ("minuta", "minuty"), ("sekunda", "sekundy")], ago_label: "temu", just_now_label: "przed chwilą" });
        map.insert(Language::ARZ, TimeLabelData { intervals: vec![("سنة", "سنين"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقايق"), ("ثانية", "ثواني")], ago_label: "منذ", just_now_label: "الآن" });
        map.insert(Language::UKR, TimeLabelData { intervals: vec![("рік", "роки"), ("місяць", "місяці"), ("день", "дні"), ("година", "години"), ("хвилина", "хвилини"), ("секунда", "секунди")], ago_label: "тому", just_now_label: "щойно" });
        map.insert(Language::ITN, TimeLabelData { intervals: vec![("anno", "anni"), ("mese", "mesi"), ("giorno", "giorni"), ("ora", "ore"), ("minuto", "minuti"), ("secondo", "secondi")], ago_label: "fa", just_now_label: "adesso" });
        map.insert(Language::HSN, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::MJS, TimeLabelData { intervals: vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("minit", "minit"), ("saat", "saat")], ago_label: "yang lalu", just_now_label: "sekarang" });
        map.insert(Language::HAK, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::KJV, TimeLabelData { intervals: vec![("taun", "taun"), ("wulan", "wulan"), ("dino", "dino"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")], ago_label: "kepungkur", just_now_label: "saiki" });
        map.insert(Language::ORY, TimeLabelData { intervals: vec![("ବର୍ଷ", "ବର୍ଷ"), ("ମାସ", "ମାସ"), ("ଦିନ", "ଦିନ"), ("ଘଣ୍ଟା", "ଘଣ୍ଟା"), ("ମିନିଟ୍", "ମିନିଟ୍"), ("ସେକେଣ୍ଡ", "ସେକେଣ୍ଡ")], ago_label: "ପୂର୍ବେ", just_now_label: "ବର୍ତ୍ତମାନ" });
        map.insert(Language::PNB, TimeLabelData { intervals: vec![("سال", "سال"), ("مہینا", "مہینے"), ("دن", "دن"), ("گھنٹہ", "گھنٹے"), ("منٹ", "منٹ"), ("سیکنڈ", "سیکنڈ")], ago_label: "پہلے", just_now_label: "ابھی ابھی" });
        map.insert(Language::SUO, TimeLabelData { intervals: vec![("vuosi", "vuotta"), ("kuukausi", "kuukautta"), ("päivä", "päivää"), ("tunti", "tuntia"), ("minuutti", "minuuttia"), ("sekunti", "sekuntia")], ago_label: "sitten", just_now_label: "juuri nyt" });
        map.insert(Language::PNJ, TimeLabelData { intervals: vec![("sal", "saal"), ("mahina", "mahine"), ("din", "din"), ("ghanta", "ghante"), ("minat", "minat"), ("sekand", "sekand")], ago_label: "pehlan", just_now_label: "hun" });
        map.insert(Language::RUM, TimeLabelData { intervals: vec![("an", "ani"), ("lună", "luni"), ("zi", "zile"), ("oră", "ore"), ("minut", "minute"), ("secundă", "secunde")], ago_label: "în urmă", just_now_label: "chiar acum" });
        map.insert(Language::BHJ, TimeLabelData { intervals: vec![("sal", "saal"), ("mahino", "mahine"), ("divas", "divas"), ("ghanta", "ghante"), ("minit", "minite"), ("sekand", "sekand")], ago_label: "pahile", just_now_label: "abhiye" });
        map.insert(Language::AZB, TimeLabelData { intervals: vec![("il", "illər"), ("ay", "aylar"), ("gün", "günlər"), ("saat", "saatlar"), ("dəqiqə", "dəqiqələr"), ("saniyə", "saniyələr")], ago_label: "əvvəl", just_now_label: "indi" });
        map.insert(Language::PES, TimeLabelData { intervals: vec![("sâl", "sâl"), ("mâh", "mâh"), ("ruz", "ruz"), ("sâ`at", "sâ`at"), ("daghigheh", "daghigheh"), ("sâniye", "sâniye")], ago_label: "پیش", just_now_label: "همین الان" });
        map.insert(Language::HUA, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::MKP, TimeLabelData { intervals: vec![("taun", "taun"), ("bulan", "bulan"), ("ari", "ari"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")], ago_label: "yang lalu", just_now_label: "sekarang" }); 
        map.insert(Language::HUA, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::ARQ, TimeLabelData { intervals: vec![("عام", "أعوام"), ("شهر", "أشهر"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثوان")], ago_label: "قبل", just_now_label: "الآن" });
        map.insert(Language::BMS, TimeLabelData { intervals: vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("menit", "menit"), ("saat", "saat")], ago_label: "yang lalu", just_now_label: "sekarang" });
        map.insert(Language::SRC, TimeLabelData { intervals: vec![("godina", "godine"), ("mesec", "meseci"), ("dan", "dani"), ("sat", "sati"), ("minut", "minuta"), ("sekund", "sekundi")], ago_label: "pre", just_now_label: "upravo sada" });
        map.insert(Language::KNN, TimeLabelData { intervals: vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")], ago_label: "前", just_now_label: "刚刚" });
        map.insert(Language::AWD, TimeLabelData { intervals: vec![("yaro", "yaro"), ("wata", "watanni"), ("rana", "kwanaki"), ("awa", "awowi"), ("minti", "mintoci"), ("sakan", "sakwanni")], ago_label: "da", just_now_label: "yanzu yanzu" });
        map.insert(Language::THJ, TimeLabelData { intervals: vec![("pì", "pì"), ("dəən", "dəən"), ("wǎn", "wǎn"), ("ɕíamōːŋ", "ɕíamōːŋ"), ("naːtʰiː", "naːtʰiː"), ("wíʔnaːtʰiː", "wíʔnaːtʰiː")], ago_label: "ก่อน", just_now_label: "เมื่อกี้นี้" });
        map.insert(Language::DUT, TimeLabelData { intervals: vec![("jaar", "jaar"), ("maand", "maanden"), ("dag", "dagen"), ("uur", "uren"), ("minuut", "minuten"), ("seconde", "seconden")], ago_label: "geleden", just_now_label: "zojuist" });
        map.insert(Language::YOR, TimeLabelData { intervals: vec![("ọdún", "ọdún"), ("osù", "osù"), ("ọjọ́", "ọjọ́"), ("wákàtí", "wákàtí"), ("ìṣẹ́jú", "ìṣẹ́jú"), ("àáyá", "àáyá")], ago_label: "sẹ́yìn", just_now_label: "nisisiyi" });
        map.insert(Language::SND, TimeLabelData { intervals: vec![("سال", "سال"), ("مهينو", "مهينا"), ("ڏينهن", "ڏينهن"), ("ڪلاڪ", "ڪلاڪ"), ("منٽ", "منٽ"), ("سيڪنڊ", "سيڪنڊ")], ago_label: "اڳ", just_now_label: "هاڻي ئي" });
        map.insert(Language::ARY, TimeLabelData { intervals: vec![("عام", "أعوام"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثواني")], ago_label: "قبل", just_now_label: "الآن" });
        map.insert(Language::AEC, TimeLabelData { intervals: vec![("ano", "anos"), ("mês", "meses"), ("dia", "dias"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")], ago_label: "atrás", just_now_label: "agora mesmo" });
        map.insert(Language::UZB, TimeLabelData { intervals: vec![("yil", "yillar"), ("oy", "oylar"), ("kun", "kunlar"), ("soat", "soatlar"), ("daqiqa", "daqiqalar"), ("soniya", "soniya")], ago_label: "oldin", just_now_label: "hozirgina" });
        map.insert(Language::MLI, TimeLabelData { intervals: vec![("വര്‍ഷം", "വര്‍ഷം"), ("മാസം", "മാസം"), ("ദിവസം", "ദിവസം"), ("മണിക്കൂര്‍", "മണിക്കൂര്‍"), ("മിനിറ്റ്", "മിനിറ്റ്"), ("സെക്കന്റ്", "സെക്കന്റ്")], ago_label: "മുമ്പ്", just_now_label: "ഇപ്പോള്‍" });
        map.insert(Language::AMH, TimeLabelData { intervals: vec![("ዓመት", "ዓመታት"), ("ወር", "ወራት"), ("ቀን", "ቀናት"), ("ሰዓት", "ሰዓታት"), ("ደቂቃ", "ደቂቃዎች"), ("ሰከንድ", "ሰከንዶች")], ago_label: "በፊት", just_now_label: "አሁን" });
        map.insert(Language::INZ, TimeLabelData { intervals: vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")], ago_label: "yang lalu", just_now_label: "sekarang" });
        map.insert(Language::IGR, TimeLabelData { intervals: vec![("afọ", "afọ"), ("ọnwa", "ọnwa"), ("ụbọchị", "ụbọchị"), ("awa", "awa"), ("nkeji", "nkeji"), ("sekọnd", "sekọnd")], ago_label: "gara aga", just_now_label: "ugbu a" });
        map.insert(Language::TGL, TimeLabelData { intervals: vec![("taon", "taon"), ("buwan", "buwan"), ("araw", "araw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "nakalipas", just_now_label: "ngayon lang" });
        map.insert(Language::NEP, TimeLabelData { intervals: vec![("वर्ष", "वर्ष"), ("महिना", "महिना"), ("दिन", "दिन"), ("घण्टा", "घण्टा"), ("मिनेट", "मिनेट"), ("सेकेन्ड", "सेकेन्ड")], ago_label: "अगाडि", just_now_label: "अहिले" });
        map.insert(Language::APD, TimeLabelData { intervals: vec![("taon", "taon"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::SKR, TimeLabelData { intervals: vec![("rok", "roky"), ("mesiac", "mesiace"), ("deň", "dni"), ("hodina", "hodiny"), ("minúta", "minúty"), ("sekunda", "sekundy")], ago_label: "pred", just_now_label: "práve teraz" });
        map.insert(Language::CEB, TimeLabelData { intervals: vec![("tuig", "tuig"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::APC, TimeLabelData { intervals: vec![("taon", "taon"), ("bulan", "bulan"), ("aldaw", "aldaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::TTS, TimeLabelData { intervals: vec![("taon", "taon"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::ASM, TimeLabelData { intervals: vec![("বছৰ", "বছৰ"), ("মাহ", "মাহ"), ("দিন", "দিন"), ("ঘণ্টা", "ঘণ্টা"), ("মিনিট", "মিনিট"), ("ছেকেণ্ড", "ছেকেণ্ড")], ago_label: "আগতে", just_now_label: "এতিয়াই" });
        map.insert(Language::HNG, TimeLabelData { intervals: vec![("év", "évek"), ("hónap", "hónapok"), ("nap", "napok"), ("óra", "órák"), ("perc", "percek"), ("másodperc", "másodpercek")], ago_label: "ezelőtt", just_now_label: "most" });
        map.insert(Language::CIT, TimeLabelData { intervals: vec![("taun", "taun"), ("bulan", "bulan"), ("ari", "ari"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")], ago_label: "kapungkur", just_now_label: "ayeuna" });
        map.insert(Language::ACM, TimeLabelData { intervals: vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("minit", "minit"), ("saat", "saat")], ago_label: "yang lalu", just_now_label: "sekarang" });
        map.insert(Language::MHJ, TimeLabelData { intervals: vec![("taon", "taon"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "kaniadtong", just_now_label: "karong bag-o lang" });
        map.insert(Language::SNH, TimeLabelData { intervals: vec![("අවුරුද්ද", "අවුරුදු"), ("මාසය", "මාස"), ("දිනය", "දින"), ("පැය", "පැය"), ("මිනිත්තුව", "මිනිත්තු"), ("තත්පරය", "තත්පර")], ago_label: "කලින්", just_now_label: "දැන්ම" });
        map.insert(Language::BGC, TimeLabelData { intervals: vec![("godina", "godine"), ("mjesec", "mjeseci"), ("dan", "dana"), ("sat", "sati"), ("minuta", "minuta"), ("sekunda", "sekunde")], ago_label: "prije", just_now_label: "upravo sada" });
        map.insert(Language::MKD, TimeLabelData { intervals: vec![("година", "години"), ("месец", "месеци"), ("ден", "дена"), ("час", "часа"), ("минута", "минути"), ("секунда", "секунди")], ago_label: "пред", just_now_label: "токму сега" });
        map.insert(Language::CZC, TimeLabelData { intervals: vec![("rok", "roky"), ("měsíc", "měsíce"), ("den", "dny"), ("hodina", "hodiny"), ("minuta", "minuty"), ("sekunda", "sekundy")], ago_label: "před", just_now_label: "právě teď" });
        map.insert(Language::GRK, TimeLabelData { intervals: vec![("χρόνος", "χρόνια"), ("μήνας", "μήνες"), ("μέρα", "μέρες"), ("ώρα", "ώρες"), ("λεπτό", "λεπτά"), ("δευτερόλεπτο", "δευτερόλεπτα")], ago_label: "πριν", just_now_label: "μόλις τώρα" });
        map.insert(Language::MQM, TimeLabelData { intervals: vec![("jdu", "jdu"), ("wer", "wer"), ("du", "du"), ("saat", "saac"), ("daqiiqa", "daqiiqa"), ("ilbiriqsi", "ilbiriqsi")], ago_label: "antigu", just_now_label: "agora mesmu" });
        map.insert(Language::HNE, TimeLabelData { intervals: vec![("mitta", "mitta"), ("hila", "hila"), ("ha'ati", "ha'ati"), ("hola", "hola"), ("miniti", "miniti"), ("sekoni", "sekoni")], ago_label: "pahile", just_now_label: "abhi" });
        map.insert(Language::DCC, TimeLabelData { intervals: vec![("yaro", "yaro"), ("wata", "watanni"), ("rana", "kwanaki"), ("awa", "awowi"), ("minti", "mintoci"), ("sakan", "sakwanni")], ago_label: "da", just_now_label: "yanzu yanzu" });
        map.insert(Language::MNP, TimeLabelData { intervals: vec![("jil", "jiliin"), ("sara", "sar"), ("ö Dör", "ödör"), ("tsag", "tsag"), ("minut", "minut"), ("sekund", "sekund")], ago_label: "öön", just_now_label: "sain" });
        map.insert(Language::RUW, TimeLabelData { intervals: vec![("mwaka", "miaka"), ("mwezi", "miezi"), ("siku", "siku"), ("saa", "masaa"), ("dakika", "dakika"), ("sekunde", "sekunde")], ago_label: "ilizopita", just_now_label: "none" });
        map.insert(Language::CCX, TimeLabelData { intervals: vec![("jaar", "jaar"), ("maand", "maanden"), ("dag", "dagen"), ("uur", "uren"), ("minuut", "minuten"), ("sekonde", "sekonde")], ago_label: "gelede", just_now_label: "nou net" });
        map.insert(Language::ARS, TimeLabelData { intervals: vec![("سنة", "سنين"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثواني")], ago_label: "مضى", just_now_label: "الآن" });
        map.insert(Language::PBU, TimeLabelData { intervals: vec![("saal", "saal"), ("maahina", "maahine"), ("din", "din"), ("ghantaa", "ghante"), ("minat", "minat"), ("sekand", "sekand")], ago_label: "وړاندې", just_now_label: "همدا اوس" });
        map.insert(Language::SOM, TimeLabelData { intervals: vec![("sanad", "sanado"), ("bil", "bilood"), ("maalin", "maalmo"), ("saac", "saacadood"), ("daqiiqad", "daqiiqado"), ("ilbiriqsi", "ilbiriqsiyo")], ago_label: "kahor", just_now_label: "Hadda" });
        map.insert(Language::MEX, TimeLabelData { intervals: vec![("año", "años"), ("mes", "meses"), ("día", "días"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")], ago_label: "hace", just_now_label: "ahora mismo" });
        map.insert(Language::AEB, TimeLabelData { intervals: vec![("عام", "أعوام"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثواني")], ago_label: "قبل", just_now_label: "الآن" });
        map.insert(Language::RUA, TimeLabelData { intervals: vec![("mwaka", "miaka"), ("mwezi", "miezi"), ("siku", "siku"), ("saa", "masaa"), ("dakika", "dakika"), ("sekunde", "sekunde")], ago_label: "ilizopita", just_now_label: "sasa hivi" });
        map.insert(Language::ZUU, TimeLabelData { intervals: vec![("umnyaka", "iminyaka"), ("inyanga", "izinyanga"), ("usuku", "izinsuku"), ("ihora", "amahora"), ("umzuzu", "imizuzu"), ("isekhondi", "amasekhondi")], ago_label: "esidlule", just_now_label: "manje" });
        map.insert(Language::BLG, TimeLabelData { intervals: vec![("година", "години"), ("месец", "месеци"), ("ден", "дни"), ("час", "часа"), ("минута", "минути"), ("секунда", "секунди")], ago_label: "преди", just_now_label: "току-що" });
        map.insert(Language::SWD, TimeLabelData { intervals: vec![("år", "år"), ("månad", "månader"), ("dag", "dagar"), ("timme", "timmar"), ("minut", "minuter"), ("sekund", "sekunder")], ago_label: "sedan", just_now_label: "just nu" });
        map.insert(Language::LMO, TimeLabelData { intervals: vec![("ann", "ann"), ("mess", "mess"), ("dìa", "dìa"), ("ora", "or"), ("minütt", "minütt"), ("segond", "segond")], ago_label: "fa", just_now_label: "adess" });
        map.insert(Language::GAZ, TimeLabelData { intervals: vec![("waggaa", "waggaawwan"), ("ji'a", "ji'oota"), ("guyyaa", "guyyaawwan"), ("sa'aatii", "sa'aatiiwwan"), ("daqiiqaa", "daqiiqaa"), ("sekendii", "sekendiiwwan")], ago_label: "dura", just_now_label: "yeroo ammaa" });
        map.insert(Language::PBT, TimeLabelData { intervals: vec![("سال", "سال"), ("ماه", "ماه"), ("روز", "روز"), ("ساعت", "ساعت"), ("دقیقه", "دقیقه"), ("ثانیه", "ثانیه")], ago_label: "پیش", just_now_label: "همین الان" });
        map.insert(Language::KAZ, TimeLabelData { intervals: vec![("жыл", "жылдар"), ("ай", "айлар"), ("күн", "күндер"), ("сағат", "сағат"), ("минут", "минут"), ("секунд", "секунд")], ago_label: "бұрын", just_now_label: "жаңа ғана" });
        map.insert(Language::ILO, TimeLabelData { intervals: vec![("tawən", "tawən"), ("bulan", "bulan"), ("aldáw", "aldáw"), ("óras", "óras"), ("minúto", "minúto"), ("segúndo", "segúndo")], ago_label: "napalabas", just_now_label: "ta agdama" });
        map.insert(Language::TTR, TimeLabelData { intervals: vec![("ел", "ел"), ("ай", "ай"), ("көн", "көн"), ("сәгать", "сәгать"), ("минут", "минут"), ("секунд", "секунд")], ago_label: "элек", just_now_label: "әле генә" });
        map.insert(Language::FUV, TimeLabelData { intervals: vec![("jaaru", "jaaru"), ("handu", "handuuji"), ("nyalaande", "nyalaande"), ("waktu", "waktu"), ("miniti", "miniti"), ("sikin", "sikin")], ago_label: "ko", just_now_label: "yanzu yanzu" });
        map.insert(Language::AYN, TimeLabelData { intervals: vec![("jil", "jila"), ("wat", "watan"), ("ur", "uraq"), ("saat", "saat"), ("minuta", "minuta"), ("sekunda", "sekunda")], ago_label: "burun", just_now_label: "qazir" });
        map.insert(Language::UIG, TimeLabelData { intervals: vec![("يىل", "يىل"), ("ئاي", "ئاي"), ("كۈن", "كۈن"), ("سائەت", "سائەت"), ("مىنۇت", "مىنۇت"), ("سىكۇنت", "سىكۇنت")], ago_label: "بۇرۇن", just_now_label: "ھازىر" });
        map.insert(Language::HAT, TimeLabelData { intervals: vec![("ane", "ane"), ("mwa", "mwa"), ("jou", "jou"), ("lè", "lè(z)"), ("minit", "minit"), ("segond", "segond")], ago_label: "de sa", just_now_label: "kounye a" });
        map.insert(Language::AZE, TimeLabelData { intervals: vec![("il", "illər"), ("ay", "aylar"), ("gün", "günlər"), ("saat", "saatlar"), ("dəqiqə", "dəqiqələr"), ("saniyə", "saniyələr")], ago_label: "əvvəl", just_now_label: "indi" });
        map.insert(Language::NPL, TimeLabelData { intervals: vec![("वर्ष", "वर्ष"), ("महिना", "महिना"), ("दिन", "दिन"), ("घण्टा", "घण्टा"), ("मिनेट", "मिनेट"), ("सेकेन्ड", "सेकेन्ड")], ago_label: "अगाडि", just_now_label: "अहिले" });
        map.insert(Language::KMR, TimeLabelData { intervals: vec![("ឆ្នាំ", "ឆ្នាំ"), ("ខែ", "ខែ"), ("ថ្ងៃ", "ថ្ងៃ"), ("ម៉ោង", "ម៉ោង"), ("នាទី", "នាទី"), ("វិនាទី", "វិនាទី")], ago_label: "មុន", just_now_label: "ឥឡូវនេះ" });
        map.insert(Language::PRS, TimeLabelData { intervals: vec![("سال", "سال"), ("ماه", "ماه"), ("روز", "روز"), ("ساعت", "ساعت"), ("دقیقه", "دقیقه"), ("ثانیه", "ثانیه")], ago_label: "پیش", just_now_label: "همین الان" });
        map.insert(Language::TWS, TimeLabelData { intervals: vec![("afe", "mfe"), ("bosome", "abosome"), ("eda", "nneda"), ("dɔnhwer", "nnɔnhwer"), ("sekan", "nsekan"), ("simma", "nsimma")], ago_label: "ni", just_now_label: "seesei ara" });
        map.insert(Language::HIL, TimeLabelData { intervals: vec![("tuig", "tuig"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")], ago_label: "sang una", just_now_label: "subong lang" });
        map.insert(Language::KUR, TimeLabelData { intervals: vec![("sal", "sal"), ("meh", "meh"), ("roj", "roj"), ("saet", "saetan"), ("deqe", "deqe"), ("çirk", "çirk")], ago_label: "berê", just_now_label: "niha" });
        map.insert(Language::SHD, TimeLabelData { intervals: vec![("jaar", "jaar"), ("maent", "maente"), ("daach", "daache"), ("stonde", "stonde"), ("minuut", "minute"), ("sekunde", "sekunde")], ago_label: "gelede", just_now_label: "zojuist" });

        map // Return the HashMap
    };
}