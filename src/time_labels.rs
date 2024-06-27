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



lazy_static! {
    pub static ref TIME_LABELS: HashMap<Language, Vec<(&'static str, &'static str)>> = {
        let mut map = HashMap::new();
        map.insert(Language::CHN, vec![("年", "年"), ("月", "月"), ("天", "天"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")]);
        map.insert(Language::SPN, vec![("año", "años"), ("mes", "meses"), ("día", "días"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")]);
        map.insert(Language::ENG, vec![("year", "years"), ("month", "months"), ("day", "days"), ("hour", "hours"), ("minute", "minutes"), ("second", "seconds")]);
        map.insert(Language::BNG, vec![("বছর", "বছর"), ("মাস", "মাস"), ("দিন", "দিন"), ("ঘন্টা", "ঘন্টা"), ("মিনিট", "মিনিট"), ("সেকেন্ড", "সেকেন্ড")]);
        map.insert(Language::HND, vec![("वर्ष", "वर्ष"), ("महीना", "महीने"), ("दिन", "दिन"), ("घंटा", "घंटे"), ("मिनट", "मिनट"), ("सेकंड", "सेकंड")]);
        map.insert(Language::POR, vec![("ano", "anos"), ("mês", "meses"), ("dia", "dias"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")]);
        map.insert(Language::RUS, vec![("год", "года"), ("месяц", "месяцы"), ("день", "дни"), ("час", "часа"), ("минута", "минуты"), ("секунда", "секунды")]);
        map.insert(Language::JPN, vec![("年", "年"), ("月", "月"), ("日", "日"), ("時間", "時間"), ("分", "分"), ("秒", "秒")]);
        map.insert(Language::GER, vec![("Jahr", "Jahre"), ("Monat", "Monate"), ("Tag", "Tage"), ("Stunde", "Stunden"), ("Minute", "Minuten"), ("Sekunde", "Sekunden")]);
        map.insert(Language::WUU, vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")]);
        map.insert(Language::JAN, vec![("年", "年"), ("月", "月"), ("日", "日"), ("時間", "時間"), ("分", "分"), ("秒", "秒")]);
        map.insert(Language::KKN, vec![("jiiq", "jiiq"), ("wer", "wer"), ("uruq", "uruq"), ("sa’a", "sa’aat"), ("daqiiqa", "daqaa’iq"), ("thaaniya", "thawaani")]);
        map.insert(Language::FRN, vec![("an", "ans"), ("mois", "mois"), ("jour", "jours"), ("heure", "heures"), ("minute", "minutes"), ("seconde", "secondes")]);
        map.insert(Language::VIE, vec![("năm", "năm"), ("tháng", "tháng"), ("ngày", "ngày"), ("giờ", "giờ"), ("phút", "phút"), ("giây", "giây")]);
        map.insert(Language::TCW, vec![("hidaw", "hidaw"), ("buwan", "buwan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::YUH, vec![("nián", "nián"), ("yuè", "yuè"), ("tiān", "tiān"), ("xiǎoshí", "xiǎoshí"), ("fēnzhōng", "fēnzhōng"), ("miǎo", "miǎo")]);
        map.insert(Language::MRT, vec![("वर्ष", "वर्ष"), ("महिना", "महिने"), ("दिवस", "दिवस"), ("तास", "तास"), ("मिनिट", "मिनिटे"), ("सेकंद", "सेकंद")]);
        map.insert(Language::TCV, vec![("kawak", "kawak"), ("bulan", "bulan"), ("aldow", "aldow"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::TRK, vec![("yıl", "yıl"), ("ay", "ay"), ("gün", "gün"), ("saat", "saat"), ("dakika", "dakika"), ("saniye", "saniye")]);
        map.insert(Language::URD, vec![("سال", "سال"), ("مہینہ", "مہینے"), ("دن", "دن"), ("گھنٹہ", "گھنٹے"), ("منٹ", "منٹ"), ("سیکنڈ", "سیکنڈ")]);
        map.insert(Language::CFR, vec![("an", "ani"), ("lună", "luni"), ("zi", "zile"), ("oră", "ore"), ("minut", "minute"), ("secundă", "secunde")]);
        map.insert(Language::CJY, vec![("rok", "lata"), ("miesiąc", "miesiące"), ("dzień", "dni"), ("godzina", "godziny"), ("minuta", "minuty"), ("sekunda", "sekundy")]);
        map.insert(Language::GJR, vec![("વર્ષ", "વર્ષ"), ("મહિનો", "મહિના"), ("દિવસ", "દિવસો"), ("કલાક", "કલાકો"), ("મિનિટ", "મિનિટ"), ("સેકન્ડ", "સેકન્ડ")]);
        map.insert(Language::PQL, vec![("rok", "roky"), ("miesąc", "miesiące"), ("dzień", "dni"), ("godzina", "godziny"), ("minuta", "minuty"), ("sekunda", "sekundy")]);
        map.insert(Language::ARZ, vec![("سنة", "سنين"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقايق"), ("ثانية", "ثواني")]);
        map.insert(Language::UKR, vec![("рік", "роки"), ("місяць", "місяці"), ("день", "дні"), ("година", "години"), ("хвилина", "хвилини"), ("секунда", "секунди")]);
        map.insert(Language::ITN, vec![("anno", "anni"), ("mese", "mesi"), ("giorno", "giorni"), ("ora", "ore"), ("minuto", "minuti"), ("secondo", "secondi")]);
        map.insert(Language::HSN, vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")]);
        map.insert(Language::MJS, vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("minit", "minit"), ("saat", "saat")]);
        map.insert(Language::HAK, vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")]);
        map.insert(Language::KJV, vec![("taun", "taun"), ("wulan", "wulan"), ("dino", "dino"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")]);
        map.insert(Language::ORY, vec![("ବର୍ଷ", "ବର୍ଷ"), ("ମାସ", "ମାସ"), ("ଦିନ", "ଦିନ"), ("ଘଣ୍ଟା", "ଘଣ୍ଟା"), ("ମିନିଟ୍", "ମିନିଟ୍"), ("ସେକେଣ୍ଡ", "ସେକେଣ୍ଡ")]);
        map.insert(Language::PNB, vec![("سال", "سال"), ("مہینا", "مہینے"), ("دن", "دن"), ("گھنٹہ", "گھنٹے"), ("منٹ", "منٹ"), ("سیکنڈ", "سیکنڈ")]);
        map.insert(Language::SUO, vec![("vuosi", "vuotta"), ("kuukausi", "kuukautta"), ("päivä", "päivää"), ("tunti", "tuntia"), ("minuutti", "minuuttia"), ("sekunti", "sekuntia")]);
        map.insert(Language::PNJ, vec![("sal", "saal"), ("mahina", "mahine"), ("din", "din"), ("ghanta", "ghante"), ("minat", "minat"), ("sekand", "sekand")]);
        map.insert(Language::RUM, vec![("an", "ani"), ("lună", "luni"), ("zi", "zile"), ("oră", "ore"), ("minut", "minute"), ("secundă", "secunde")]);
        map.insert(Language::BHJ, vec![("sal", "saal"), ("mahino", "mahine"), ("divas", "divas"), ("ghanta", "ghante"), ("minit", "minite"), ("sekand", "sekand")]);
        map.insert(Language::AZB, vec![("il", "illər"), ("ay", "aylar"), ("gün", "günlər"), ("saat", "saatlar"), ("dəqiqə", "dəqiqələr"), ("saniyə", "saniyələr")]);
        map.insert(Language::PES, vec![("sâl", "sâl"), ("mâh", "mâh"), ("ruz", "ruz"), ("sâ`at", "sâ`at"), ("daghigheh", "daghigheh"), ("sâniye", "sâniye")]);
        map.insert(Language::MKP, vec![("taun", "taun"), ("bulan", "bulan"), ("ari", "ari"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")]);
        map.insert(Language::HUA, vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")]);
        map.insert(Language::ARQ, vec![("عام", "أعوام"), ("شهر", "أشهر"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثوان")]);
        map.insert(Language::BMS, vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("minit", "minit"), ("saat", "saat")]);
        map.insert(Language::SRC, vec![("godina", "godine"), ("mesec", "meseci"), ("dan", "dani"), ("sat", "sati"), ("minut", "minuta"), ("sekund", "sekundi")]);
        map.insert(Language::KNN, vec![("年", "年"), ("月", "月"), ("日", "日"), ("小时", "小时"), ("分钟", "分钟"), ("秒", "秒")]);
        map.insert(Language::AWD, vec![("yaro", "yaro"), ("wata", "watanni"), ("rana", "kwanaki"), ("awa", "awowi"), ("minti", "mintoci"), ("sakan", "sakwanni")]);
        map.insert(Language::THJ, vec![("pì", "pì"), ("dəən", "dəən"), ("wǎn", "wǎn"), ("ɕíamōːŋ", "ɕíamōːŋ"), ("naːtʰiː", "naːtʰiː"), ("wíʔnaːtʰiː", "wíʔnaːtʰiː")]);
        map.insert(Language::DUT, vec![("jaar", "jaar"), ("maand", "maanden"), ("dag", "dagen"), ("uur", "uren"), ("minuut", "minuten"), ("seconde", "seconden")]);
        map.insert(Language::YOR, vec![("ọdún", "ọdún"), ("osù", "osù"), ("ọjọ́", "ọjọ́"), ("wákàtí", "wákàtí"), ("ìṣẹ́jú", "ìṣẹ́jú"), ("àáyá", "àáyá")]);
        map.insert(Language::SND, vec![("سال", "سال"), ("مهينو", "مهينا"), ("ڏينهن", "ڏينهن"), ("ڪلاڪ", "ڪلاڪ"), ("منٽ", "منٽ"), ("سيڪنڊ", "سيڪنڊ")]);
        map.insert(Language::ARY, vec![("عام", "أعوام"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثواني")]);
        map.insert(Language::AEC, vec![("ano", "anos"), ("mês", "meses"), ("dia", "dias"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")]);
        map.insert(Language::UZB, vec![("yil", "yillar"), ("oy", "oylar"), ("kun", "kunlar"), ("soat", "soatlar"), ("daqiqa", "daqiqalar"), ("soniya", "soniya")]);
        map.insert(Language::MLI, vec![("വര്‍ഷം", "വര്‍ഷം"), ("മാസം", "മാസം"), ("ദിവസം", "ദിവസം"), ("മണിക്കൂര്‍", "മണിക്കൂര്‍"), ("മിനിറ്റ്", "മിനിറ്റ്"), ("സെക്കന്റ്", "സെക്കന്റ്")]);
        map.insert(Language::AMH, vec![("ዓመት", "ዓመታት"), ("ወር", "ወራት"), ("ቀን", "ቀናት"), ("ሰዓት", "ሰዓታት"), ("ደቂቃ", "ደቂቃዎች"), ("ሰከንድ", "ሰከንዶች")]);
        map.insert(Language::INZ, vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")]);
        map.insert(Language::IGR, vec![("afọ", "afọ"), ("ọnwa", "ọnwa"), ("ụbọchị", "ụbọchị"), ("awa", "awa"), ("nkeji", "nkeji"), ("sekọnd", "sekọnd")]);
        map.insert(Language::TGL, vec![("taon", "taon"), ("buwan", "buwan"), ("araw", "araw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::NEP, vec![("वर्ष", "वर्ष"), ("महिना", "महिना"), ("दिन", "दिन"), ("घण्टा", "घण्टा"), ("मिनेट", "मिनेट"), ("सेकेन्ड", "सेकेन्ड")]);
        map.insert(Language::APD, vec![("taun", "taun"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::SKR, vec![("rok", "roky"), ("mesiac", "mesiace"), ("deň", "dni"), ("hodina", "hodiny"), ("minúta", "minúty"), ("sekunda", "sekundy")]);
        map.insert(Language::CEB, vec![("tuig", "tuig"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::APC, vec![("taon", "taon"), ("bulan", "bulan"), ("aldaw", "aldaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::TTS, vec![("taon", "taon"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::ASM, vec![("বছৰ", "বছৰ"), ("মাহ", "মাহ"), ("দিন", "দিন"), ("ঘণ্টা", "ঘণ্টা"), ("মিনিট", "মিনিট"), ("ছেকেণ্ড", "ছেকেণ্ড")]);
        map.insert(Language::HNG, vec![("év", "évek"), ("hónap", "hónapok"), ("nap", "napok"), ("óra", "órák"), ("perc", "percek"), ("másodperc", "másodpercek")]);
        map.insert(Language::CIT, vec![("taun", "taun"), ("bulan", "bulan"), ("ari", "ari"), ("jam", "jam"), ("menit", "menit"), ("detik", "detik")]);
        map.insert(Language::ACM, vec![("tahun", "tahun"), ("bulan", "bulan"), ("hari", "hari"), ("jam", "jam"), ("minit", "minit"), ("saat", "saat")]);
        map.insert(Language::MHJ, vec![("taon", "taon"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::SNH, vec![("අවුරුද්ද", "අවුරුදු"), ("මාසය", "මාස"), ("දිනය", "දින"), ("පැය", "පැය"), ("මිනිත්තුව", "මිනිත්තු"), ("තත්පරය", "තත්පර")]);
        map.insert(Language::BGC, vec![("godina", "godine"), ("mjesec", "mjeseci"), ("dan", "dana"), ("sat", "sati"), ("minuta", "minuta"), ("sekunda", "sekunde")]);
        map.insert(Language::MKD, vec![("година", "години"), ("месец", "месеци"), ("ден", "дена"), ("час", "часа"), ("минута", "минути"), ("секунда", "секунди")]);
        map.insert(Language::CZC, vec![("rok", "roky"), ("měsíc", "měsíce"), ("den", "dny"), ("hodina", "hodiny"), ("minuta", "minuty"), ("sekunda", "sekundy")]);
        map.insert(Language::GRK, vec![("χρόνος", "χρόνια"), ("μήνας", "μήνες"), ("μέρα", "μέρες"), ("ώρα", "ώρες"), ("λεπτό", "λεπτά"), ("δευτερόλεπτο", "δευτερόλεπτα")]);
        map.insert(Language::MQM, vec![("jdu", "jdu"), ("wer", "wer"), ("du", "du"), ("saat", "saac"), ("daqiiqa", "daqiiqa"), ("ilbiriqsi", "ilbiriqsi")]);
        map.insert(Language::HNE, vec![("mitta", "mitta"), ("hila", "hila"), ("ha'ati", "ha'ati"), ("hola", "hola"), ("miniti", "miniti"), ("sekoni", "sekoni")]);
        map.insert(Language::DCC, vec![("yaro", "yaro"), ("wata", "watanni"), ("rana", "kwanaki"), ("awa", "awowi"), ("minti", "mintoci"), ("sakan", "sakwanni")]);
        map.insert(Language::MNP, vec![("jil", "jiliin"), ("sara", "sar"), ("ö Dör", "ödör"), ("tsag", "tsag"), ("minut", "minut"), ("sekund", "sekund")]);
        map.insert(Language::RUW, vec![("mwaka", "miaka"), ("mwezi", "miezi"), ("siku", "siku"), ("saa", "masaa"), ("dakika", "dakika"), ("sekunde", "sekunde")]);
        map.insert(Language::CCX, vec![("jaar", "jaar"), ("maand", "maanden"), ("dag", "dagen"), ("uur", "uren"), ("minuut", "minuten"), ("sekonde", "sekonde")]);
        map.insert(Language::ARS, vec![("سنة", "سنين"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثواني")]);
        map.insert(Language::PBU, vec![("saal", "saal"), ("maahina", "maahine"), ("din", "din"), ("ghantaa", "ghante"), ("minat", "minat"), ("sekand", "sekand")]);
        map.insert(Language::SOM, vec![("sanad", "sanado"), ("bil", "bilood"), ("maalin", "maalmo"), ("saac", "saacadood"), ("daqiiqad", "daqiiqado"), ("ilbiriqsi", "ilbiriqsiyo")]);
        map.insert(Language::MEX, vec![("año", "años"), ("mes", "meses"), ("día", "días"), ("hora", "horas"), ("minuto", "minutos"), ("segundo", "segundos")]);
        map.insert(Language::AEB, vec![("عام", "أعوام"), ("شهر", "شهور"), ("يوم", "أيام"), ("ساعة", "ساعات"), ("دقيقة", "دقائق"), ("ثانية", "ثواني")]);
        map.insert(Language::RUA, vec![("mwaka", "miaka"), ("mwezi", "miezi"), ("siku", "siku"), ("saa", "masaa"), ("dakika", "dakika"), ("sekunde", "sekunde")]);
        map.insert(Language::ZUU, vec![("umnyaka", "iminyaka"), ("inyanga", "izinyanga"), ("usuku", "izinsuku"), ("ihora", "amahora"), ("umzuzu", "imizuzu"), ("isekhondi", "amasekhondi")]);
        map.insert(Language::BLG, vec![("година", "години"), ("месец", "месеци"), ("ден", "дни"), ("час", "часа"), ("минута", "минути"), ("секунда", "секунди")]);
        map.insert(Language::SWD, vec![("år", "år"), ("månad", "månader"), ("dag", "dagar"), ("timme", "timmar"), ("minut", "minuter"), ("sekund", "sekunder")]);
        map.insert(Language::LMO, vec![("ann", "ann"), ("mess", "mess"), ("dìa", "dìa"), ("ora", "or"), ("minütt", "minütt"), ("segond", "segond")]);
        map.insert(Language::GAZ, vec![("waggaa", "waggaawwan"), ("ji'a", "ji'oota"), ("guyyaa", "guyyaawwan"), ("sa'aatii", "sa'aatiiwwan"), ("daqiiqaa", "daqiiqaa"), ("sekendii", "sekendiiwwan")]);
        map.insert(Language::PBT, vec![("سال", "سال"), ("ماه", "ماه"), ("روز", "روز"), ("ساعت", "ساعت"), ("دقیقه", "دقیقه"), ("ثانیه", "ثانیه")]);
        map.insert(Language::KAZ, vec![("жыл", "жылдар"), ("ай", "айлар"), ("күн", "күндер"), ("сағат", "сағат"), ("минут", "минут"), ("секунд", "секунд")]);
        map.insert(Language::ILO, vec![("tawən", "tawən"), ("bulan", "bulan"), ("aldáw", "aldáw"), ("óras", "óras"), ("minúto", "minúto"), ("segúndo", "segúndo")]);
        map.insert(Language::TTR, vec![("таон", "таон"), ("булан", "булан"), ("алдав", "алдав"), ("олас", "олас"), ("минуто", "минуто"), ("сегундо", "сегундо")]);
        map.insert(Language::FUV, vec![("jaaru", "jaaru"), ("handu", "handuuji"), ("nyalaande", "nyalaande"), ("waktu", "waktu"), ("miniti", "miniti"), ("sikin", "sikin")]);
        map.insert(Language::AYN, vec![("jil", "jila"), ("wat", "watan"), ("ur", "uraq"), ("saat", "saat"), ("minuta", "minuta"), ("sekunda", "sekunda")]);
        map.insert(Language::UIG, vec![("يىل", "يىل"), ("ئاي", "ئاي"), ("كۈن", "كۈن"), ("سائەت", "سائەت"), ("مىنۇت", "مىنۇت"), ("سىكۇنت", "سىكۇنت")]);
        map.insert(Language::HAT, vec![("ane", "ane"), ("mwa", "mwa"), ("jou", "jou"), ("lè", "lè(z)"), ("minit", "minit"), ("segond", "segond")]);
        map.insert(Language::AZE, vec![("il", "illər"), ("ay", "aylar"), ("gün", "günlər"), ("saat", "saatlar"), ("dəqiqə", "dəqiqələr"), ("saniyə", "saniyələr")]);
        map.insert(Language::NPL, vec![("वर्ष", "वर्ष"), ("महिना", "महिना"), ("दिन", "दिन"), ("घण्टा", "घण्टा"), ("मिनेट", "मिनेट"), ("सेकेन्ड", "सेकेन्ड")]);
        map.insert(Language::KMR, vec![("ឆ្នាំ", "ឆ្នាំ"), ("ខែ", "ខែ"), ("ថ្ងៃ", "ថ្ងៃ"), ("ម៉ោង", "ម៉ោង"), ("នាទី", "នាទី"), ("វិនាទី", "វិនាទី")]);        map.insert(Language::PRS, vec![("سال", "سال"), ("ماه", "ماه"), ("روز", "روز"), ("ساعت", "ساعت"), ("دقیقه", "دقیقه"), ("ثانیه", "ثانیه")]);
        map.insert(Language::TWS, vec![("ni", "ni"), ("nyanga", "tinyanga"), ("siku", "masiku"), ("ola", "maola"), ("munota", "minoto"), ("sekonde", "masekonde")]);
        map.insert(Language::HIL, vec![("tuig", "tuig"), ("bulan", "bulan"), ("adlaw", "adlaw"), ("oras", "oras"), ("minuto", "minuto"), ("segundo", "segundo")]);
        map.insert(Language::KUR, vec![("sal", "sal"), ("meh", "meh"), ("roj", "roj"), ("seat", "seatan"), ("deqe", "deqe"), ("çirk", "çirk")]);
        map.insert(Language::SHD, vec![("jaar", "jaar"), ("maent", "maente"), ("daach", "daache"), ("stonde", "stonde"), ("minuut", "minute"), ("sekunde", "sekunde")]);               
        // ... (Add the rest of the 107 languages in the same format)
        map // Return the HashMap
    };
}