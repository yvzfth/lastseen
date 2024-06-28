# timeagoplus

`timeagoplus` is a WebAssembly (Wasm) package that provides a utility to calculate and display the time elapsed since a given timestamp in a human-readable format. It supports multiple languages.

## Features

- Calculates the time elapsed since a given timestamp.
- Supports detailed labels (e.g., "1 year, 2 months ago").
- Multi-language support.

## Installation

Install the package via npm:

```sh
npm install timeagoplus
```

## Usage

Here is an example of how to use the `timeagoplus` package in a React application:

```typescript
import React, { useState, useEffect } from 'react';
import init, { time_ago, Language } from 'timeagoplus';

const HomePage: React.FC = () => {
  const [time, setTime] = useState('');

  useEffect(() => {
    const fetchData = async () => {
      await init();
      const timeString = time_ago(
        new Date('2024-06-27T20:00:00+03:00').getTime(),
        true,
        Language.ENG
      );
      setTime(timeString);
    };

    fetchData();

    const intervalId = setInterval(fetchData, 60000);

    return () => clearInterval(intervalId);
  }, []);

  return <div>{time}</div>;
};

export default HomePage;
```

### `time_ago` Function

```typescript
time_ago(timestamp: number, detailed_label: boolean, lang: Language): string;
```

#### Parameters

- `timestamp`: The timestamp (in milliseconds) for which you want to calculate the elapsed time.
- `detailed_label`: A boolean value indicating whether to use detailed labels.
- `lang`: The language for the output.

#### Example

```typescript
const timestamp = new Date('2024-06-27T20:00:00+03:00').getTime();
const detailedLabel = true;
const language = Language.ENG;

const elapsedTime = time_ago(timestamp, detailedLabel, language);
console.log(elapsedTime); // Output will be in the format of "1 year, 2 months ago" or similar
```

## Supported Languages

The `Language` enum provides a wide range of languages you can use.

| Language Code | Full Language Name      | Language Family |
| ------------- | ----------------------- | --------------- |
| ACM           | Mesopotamian Arabic     | Afroasiatic     |
| AEC           | Saidi Arabic            | Afroasiatic     |
| AEB           | Tunisian Arabic         | Afroasiatic     |
| AMH           | Amharic                 | Afroasiatic     |
| APC           | North Levantine Arabic  | Afroasiatic     |
| APD           | Samar-Leyte Visayan     | Austronesian    |
| ARQ           | Algerian Arabic         | Afroasiatic     |
| ARS           | Najdi Arabic            | Afroasiatic     |
| ARY           | Moroccan Arabic         | Afroasiatic     |
| ARZ           | Egyptian Arabic         | Afroasiatic     |
| ASM           | Assamese                | Indo-European   |
| AWD           | Awadhi                  | Indo-European   |
| AYN           | Northern Altai          | Turkic          |
| AZB           | South Azerbaijani       | Turkic          |
| AZE           | Azerbaijani             | Turkic          |
| BGC           | Haryanvi                | Indo-European   |
| BHJ           | Bhojpuri                | Indo-European   |
| BLG           | Bulgarian               | Indo-European   |
| BMS           | Javanese                | Austronesian    |
| BNG           | Bengali                 | Indo-European   |
| CEB           | Cebuano                 | Austronesian    |
| CFR           | Crimean Tatar           | Turkic          |
| CHN           | Mandarin Chinese        | Sino-Tibetan    |
| CJY           | Jinyu Chinese           | Sino-Tibetan    |
| CIT           | Citak                   | Austronesian    |
| CCX           | Northern Luri           | Indo-European   |
| CZC           | Czech                   | Indo-European   |
| DCC           | Dakhini                 | Indo-European   |
| DUT           | Dutch                   | Indo-European   |
| ENG           | English                 | Indo-European   |
| FRN           | French                  | Indo-European   |
| FUV           | Nigerian Fulfulde       | Niger-Congo     |
| GAZ           | West Central Oromo      | Afroasiatic     |
| GER           | German                  | Indo-European   |
| GJR           | Gujarati                | Indo-European   |
| GRK           | Greek, Modern (1453-)   | Indo-European   |
| HAK           | Hakka Chinese           | Sino-Tibetan    |
| HAT           | Haitian; Haitian Creole | Indo-European   |
| HIL           | Hiligaynon              | Austronesian    |
| HND           | Hindi                   | Indo-European   |
| HNE           | Chhattisgarhi           | Indo-European   |
| HNG           | Hungarian               | Uralic          |
| HSN           | Xiang Chinese           | Sino-Tibetan    |
| HUA           | Mandarin Chinese        | Sino-Tibetan    |
| IGR           | Igbo                    | Niger-Congo     |
| ILO           | Iloko                   | Austronesian    |
| INZ           | Indonesian              | Austronesian    |
| ITN           | Italian                 | Indo-European   |
| JAN           | Javanese                | Austronesian    |
| JPN           | Japanese                | Japonic         |
| KAZ           | Kazakh                  | Turkic          |
| KJV           | Kannada                 | Indo-European   |
| KKN           | Kokni                   | Indo-European   |
| KMR           | Khmer                   | Austroasiatic   |
| KNN           | Gan Chinese             | Sino-Tibetan    |
| KUR           | Central Kurdish         | Indo-European   |
| LMO           | Lombard                 | Indo-European   |
| MEX           | Mexican Spanish         | Indo-European   |
| MJS           | Malaccan Creole Malay   | Austronesian    |
| MKD           | Macedonian              | Indo-European   |
| MKP           | Mohawk                  | Iroquoian       |
| MHJ           | Mahajani                | Indo-European   |
| MLI           | Malayalam               | Dravidian       |
| MNP           | Manipuri                | Sino-Tibetan    |
| MQM           | Macaense                | Indo-European   |
| MRT           | Marathi                 | Indo-European   |
| NEP           | Nepali                  | Indo-European   |
| NPL           | Nepali                  | Indo-European   |
| ORY           | Odia                    | Indo-European   |
| PBT           | Southern Pashto         | Indo-European   |
| PBU           | Northern Pashto         | Indo-European   |
| PES           | Western Farsi           | Indo-European   |
| PNJ           | Eastern Panjabi         | Indo-European   |
| PNB           | Western Panjabi         | Indo-European   |
| PQL           | Pomeranian              | Indo-European   |
| POR           | Portuguese              | Indo-European   |
| PRS           | Eastern Farsi           | Indo-European   |
| RUA           | Kinyarwanda             | Niger-Congo     |
| RUM           | Romanian                | Indo-European   |
| RUS           | Russian                 | Indo-European   |
| RUW           | Kinyarwanda             | Niger-Congo     |
| SHD           | Kundal Shahi            | Indo-European   |
| SKR           | Saraiki                 | Indo-European   |
| SNH           | Sinhala                 | Indo-European   |
| SND           | Sindhi                  | Indo-European   |
| SOM           | Somali                  | Afroasiatic     |
| SPN           | Spanish                 | Indo-European   |
| SRC           | Serbian                 | Indo-European   |
| SUO           | Finnish                 | Uralic          |
| SWD           | Swedish                 | Indo-European   |
| TCV           | Caviteño                | Austronesian    |
| TCW           | Telugu                  | Indo-European   |
| TGL           | Tagalog                 | Austronesian    |
| THJ           | Tai                     | Kra-Dai         |
| TTR           | Tatar                   | Turkic          |
| TTS           | Tausug                  | Austronesian    |
| TRK           | Turkish                 | Turkic          |
| TWS           | Twi                     | Niger-Congo     |
| UKR           | Ukrainian               | Indo-European   |
| URD           | Urdu                    | Indo-European   |
| UZB           | Uzbek                   | Turkic          |
| UIG           | Uighur                  | Turkic          |
| VIE           | Vietnamese              | Austroasiatic   |
| WUU           | Wu Chinese              | Sino-Tibetan    |
| YOR           | Yoruba                  | Niger-Congo     |
| YUH           | Chinese, Yue            | Sino-Tibetan    |
| ZUU           | Zulu                    | Niger-Congo     |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Fatih Yavuz

## Repository

[GitHub](https://github.com/yvzfth/timeagoplus)

---

This README includes a brief introduction, installation instructions, usage examples, a description of the `time_ago` function, supported languages, licensing information, and links to the author and repository.
